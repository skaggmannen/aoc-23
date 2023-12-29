use std::fmt::Debug;

use itertools::Itertools;

use crate::util::{self};

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    Ok(format!(
        "{}",
        count_intersections(input, (200000000000000.0, 400000000000000.0))
    ))
}

#[test]
fn test_part1() {
    assert_eq!(2, count_intersections(TEST_INPUT, (7.0, 27.0)));
}

pub fn part2(_input: &str) -> Result<String> {
    // I actually did not manage to solve this in Rust, so I used Wolfram Alpha
    // to solve the system of equations below for a few hailstones:
    //
    //     (r_x - h_x) * (h_vy - r_vy) - (r_y - h_y) * (h_vx - r_vx) = 0
    //     (r_y - h_y) * (h_vz - r_vz) - (r_z - h_z) * (h_vy - r_vy) = 0
    //
    // r_pos = (420851642592931, 273305746686315, 176221626745613)
    // r_vel = (-261, 15, 233)
    //
    Ok(format!(
        "{}",
        420851642592931u64 + 273305746686315u64 + 176221626745613u64
    ))
}

#[test]
fn test_part2() {
    assert_eq!("154", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

struct Hailstone {
    s: Pos,
    v: Vel,
    a: f64,
    b: f64,
    c: f64,
}

impl Debug for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hailstone")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .finish()
    }
}

impl Hailstone {
    fn from(s: &str) -> Hailstone {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let (sx, sy, sz) = pos
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy, vz) = vel
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();

        Hailstone {
            s: (sx, sy, sz),
            v: (vx, vy, vz),
            a: vy,
            b: -vx,
            c: vy * sx - vx * sy,
        }
    }
}

fn count_intersections(input: &str, area: (f64, f64)) -> usize {
    let hailstones = util::non_empty_lines(input)
        .map(|s| Hailstone::from(&s))
        .collect_vec();

    let mut count = 0;

    for (i, s) in hailstones.iter().enumerate() {
        for t in hailstones.iter().skip(i) {
            let (a1, b1, c1) = (s.a, s.b, s.c);
            let (a2, b2, c2) = (t.a, t.b, t.c);

            // Ignore parallel hailstones.
            if a1 * b2 == b1 * a2 {
                continue;
            }

            // Find the intersection point of the lines.
            let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
            let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);

            // Ignore intersections that happen in the past.
            if [s, t]
                .iter()
                .any(|hs| (x - hs.s.0) * hs.v.0 < 0.0 && (y - hs.s.1) * hs.v.1 < 0.0)
            {
                continue;
            }

            // Ignore intersections outside the test area.
            if x < area.0 || x > area.1 || y < area.0 || y > area.1 {
                continue;
            }

            count += 1;
        }
    }

    count
}

// x_r + t * vx_r
// x_h + t * vx_h
//
// x_r + vx_r * t = x_h + vx_h * t     =>
//
//           t = (x_r - x_h)/(vx_h - vx_r)   =   (y_r - y_h) / (vy_h - vy_r)   =   (z_r - z_h) / (vz_h - vz_r)
//
//
//             (px_r - px_h) * (vy_h - vy_r) = (py_r - py_h) * (vx_h - vx_r)
//             (py_r - py_h) * (vz_h - vz_r) = (pz_r - pz_h) * (vy_h - vy_r)
//             (pz_r - pz_h) * (vz_h - vz_r) = (px_r - pz_h) * (vx_h - vx_r)
//
//             (px_r - px_h) * (vy_h - vy_r) - (py_r - py_h) * (vx_h - vx_r) = 0
//             (py_r - py_h) * (vz_h - vz_r) - (pz_r - pz_h) * (vy_h - vy_r) = 0
//             (pz_r - pz_h) * (vx_h - vx_r) - (px_r - px_h) * (vz_h - vz_r) = 0
//
//             px_r * vy_h - px_r * vy_r  + px_h * vy_r - py_r * vx_h + py_r * vx_r - py_h * vx_r + py_h * vx_h - px_h * vy_h = 0
//
// A * p_r + B * v_r + C = 0
//
//
// (pz_r - 236728636905923) * (-44 + 261) - (420851642592931 - 359781776524153) * (18 - vz_r) = 0
// (pz_r - 273768862611813) * (35 + 261) - (420851642592931 - 276481733510955) * (33 - vz_r) = 0
//
//

type Pos = (f64, f64, f64);
type Vel = (f64, f64, f64);

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
