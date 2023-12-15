extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let steps = input.split(",").map(|s| s.trim());

    let score: usize = steps.map(|s| hash(s)).sum();

    Ok(score.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("1320", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    let steps = input.split(",").map(|s| s.trim());

    for s in steps {
        if s.contains("-") {
            let label = s.strip_suffix("-").unwrap();

            let box_nbr = hash(label);
            let mut idx = None;
            for (i, l) in boxes[box_nbr].iter().enumerate() {
                if l.label == label {
                    idx = Some(i);
                    break;
                }
            }

            if let Some(i) = idx {
                boxes[box_nbr].remove(i);
            }
        } else if s.contains("=") {
            let (label, focal_len) = s.split_once("=").unwrap();

            let box_nbr = hash(label);

            let mut idx = None;
            for (i, l) in boxes[box_nbr].iter().enumerate() {
                if l.label == label {
                    idx = Some(i);
                    break;
                }
            }

            let lens = Lens {
                label: label.to_string(),
                focal_length: focal_len.parse().unwrap(),
            };

            if let Some(i) = idx {
                boxes[box_nbr][i] = lens;
            } else {
                boxes[box_nbr].push(lens);
            }
        }
    }

    let mut score = 0;
    for (i, lenses) in boxes.iter().enumerate() {
        for (j, lens) in lenses.iter().enumerate() {
            score += (i + 1) * (j + 1) * lens.focal_length;
        }
    }

    Ok(score.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("145", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

fn hash(s: &str) -> usize {
    let mut hash: usize = 0;
    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
    }
    return hash % 256;
}

struct Lens {
    label: String,
    focal_length: usize,
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
