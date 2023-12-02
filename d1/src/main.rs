use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    pub fn into_value(&self) -> usize {
        return match self {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        };
    }

    pub fn from_usize(int: u32) -> Digit {
        return match int {
            0 => Digit::Zero,
            1 => Digit::One,
            2 => Digit::Two,
            3 => Digit::Three,
            4 => Digit::Four,
            5 => Digit::Five,
            6 => Digit::Six,
            7 => Digit::Seven,
            8 => Digit::Eight,
            9 => Digit::Nine,
            _ => panic!("Huh!"),
        };
    }

    pub fn try_from_str(input: &str) -> Option<Digit> {
        return Some(match input {
            "zero" => Self::Zero,
            "one" => Self::One,
            "two" => Self::Two,
            "three" => Self::Three,
            "four" => Self::Four,
            "five" => Self::Five,
            "six" => Self::Six,
            "seven" => Self::Seven,
            "eight" => Self::Eight,
            "nine" => Self::Nine,
            _ => return None,
        });
    }

    fn parse_slice(s: &str) -> Vec<Digit> {
        let mut vec = Vec::new();
        let mut start = 0;
        let mut end = 1;
        loop {
            //Pre
            if start >= s.len() {
                break;
            }

            // Loop main
            let slice = &s[start..end];
            if let Some(res) = Digit::try_from_str(slice) {
                vec.push(res);
                start += 1;
                end += start + 1;
            } else {
                end += 1;
            }

            //Post
            if end > s.len() {
                start = start + 1;
                end = start + 1;
            }
        }

        return vec;
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let file = fs::read_to_string(file_path)?;

    let res = file
        .lines()
        .map(|l| {
            let mut v = Vec::<Digit>::new();
            let mut start = 0;
            for (i, ch) in l.chars().enumerate() {
                if let Some(int) = ch.to_digit(10) {
                    let prior = Digit::parse_slice(&l[start..i]);
                    v.extend(prior);
                    v.push(Digit::from_usize(int));
                    start = i + 1;
                }
            }
            v.extend(Digit::parse_slice(&l[start..(l.len())]));
            dbg!(&v);
            return v[0].into_value() * 10 + v.last().unwrap().into_value();
        })
        .collect::<Vec<usize>>();

    dbg!(&res);

    println!("{}", res.iter().sum::<usize>());

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::Digit;

    #[test]
    fn parse_from_slice_test_all_digits() {
        let input = "onetwothree";
        let answer = vec![Digit::One, Digit::Two, Digit::Three];

        let result = Digit::parse_slice(input);

        assert_eq!(answer, result);
    }

    #[test]
    fn parse_from_slice_test_some_digits() {
        let input = "dawdfourdawdfivedawdwasixdawda";
        //               four    five      six
        let answer = vec![Digit::Four, Digit::Five, Digit::Six];

        let result = Digit::parse_slice(input);

        assert_eq!(answer, result);
    }

    #[test]
    fn parse_from_slice_test_no_digits() {
        let input = "dawddawddawdwadawda";
        //               four    five      six
        let answer: Vec<Digit> = vec![];

        let result = Digit::parse_slice(input);

        assert_eq!(answer, result);
    }
}
