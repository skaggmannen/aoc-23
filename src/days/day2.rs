use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect::<Vec<_>>();

    let process = util::compose!(parse_game, get_valid_game_id);

    let score: u32 = lines.into_iter().map(process).sum();

    Ok(format!("{}", score))
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect::<Vec<_>>();

    let process = util::compose!(parse_game, get_game_power);

    let score: u32 = lines.into_iter().map(process).sum();

    Ok(format!("{}", score))
}

fn parse_game(s: String) -> Game {
    let parts = s.split(":").collect::<Vec<&str>>();
    let id = parts[0]
        .strip_prefix("Game ")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let cube_sets = parts[1].split(";").collect::<Vec<&str>>();

    let mut game = Game{id, red: 0, green: 0, blue: 0};

    for set in cube_sets {
        let cubes = set.split(",");
        for cube in cubes {
            let parts = cube.trim().split(" ").collect::<Vec<&str>>();
            let count = parts[0].parse::<u32>().unwrap();
            let color = parts[1];

            match color {
                "red" => if count > game.red { game.red = count },
                "green" => if count > game.green { game.green = count },
                "blue" => if count > game.blue { game.blue = count },
                _ => {},
            }
        }
    }

    game
}

fn get_valid_game_id(g: Game) -> u32 {
    if g.red > 12 || g.green > 13 || g.blue > 14 {
        0
    } else {
        g.id
    }
}

fn get_game_power(g: Game) -> u32 {
    g.red * g.green * g.blue
}

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[test]
fn test_part1() {
    assert_eq!("8", part1(TEST_INPUT).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!("2286", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
