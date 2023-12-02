use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    pub fn to_string(&self) -> &str {
        return match self {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::Green => "green",
        };
    }

    pub fn from_string(color: &str) -> Option<Color> {
        return Some(match color {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            _ => return None,
        });
    }
}

#[derive(Debug)]
struct Set {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let file = fs::read_to_string(file_path)?;

    let limit = Set {
        red: 12,
        blue: 14,
        green: 13,
    };

    let games = file
        .lines()
        .map(|l| {
            let mut line = l.split(':');
            let game_id = line
                .next()
                .expect("line empty")
                .split(' ')
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .expect("not integer");
            let sets: Vec<Set> = line
                .next()
                .expect("not sets")
                .split(';')
                .into_iter()
                .map(|s| {
                    let set_results: Vec<(Color, usize)> = s
                        .split(",")
                        .map(|c| {
                            let x: Vec<&str> = c.split(' ').collect();
                            return (
                                Color::from_string(x[2]).expect("huh!"),
                                x[1].parse().unwrap(),
                            );
                        })
                        .collect();
                    let red = set_results
                        .iter()
                        .filter(|(color, _)| *color == Color::Red)
                        .next()
                        .map_or(0, |x| x.1);

                    let blue = set_results
                        .iter()
                        .filter(|(color, _)| *color == Color::Blue)
                        .next()
                        .map_or(0, |x| x.1);

                    let green = set_results
                        .iter()
                        .filter(|(color, _)| *color == Color::Green)
                        .next()
                        .map_or(0, |x| x.1);

                    return Set { red, blue, green };
                })
                .collect();

            return Game { id: game_id, sets };
        })
        .collect::<Vec<Game>>();

    dbg!(&games);

    let successfull_games_id_sum = games
        .iter()
        .filter(|g| {
            return g
                .sets
                .iter()
                .all(|s| s.red <= limit.red && s.blue <= limit.blue && s.green <= limit.green);
        })
        .map(|g| g.id)
        .sum::<usize>();

    println!("Part 1: {}", successfull_games_id_sum);

    let minimums_power_sum: usize = games
        .iter()
        .map(|g| {
            let red = g.sets.iter().max_by_key(|s| s.red).unwrap().red;
            let blue = g.sets.iter().max_by_key(|s| s.blue).unwrap().blue;
            let green = g.sets.iter().max_by_key(|s| s.green).unwrap().green;
            return red * blue * green;
        })
        .sum();

    println!("Part 2: {}", minimums_power_sum);

    return Ok(());
}
