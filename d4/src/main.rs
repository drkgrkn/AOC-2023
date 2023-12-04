use std::{env, fs};

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    own: Vec<usize>,
}

impl Card {
    fn new(id: usize) -> Card {
        return Card {
            id,
            winning: Vec::new(),
            own: Vec::new(),
        };
    }

    fn score(&self) -> usize {
        let mut point = 0;
        for own in &self.own {
            if self.winning.contains(&own) {
                point += 1;
            }
        }
        return point;
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let file = fs::read_to_string(file_path)?;

    // part1
    let mut cards = file
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let mut card = Card::new(id + 1);
            let mut is_winning = true;
            let numbers = l.split(":").skip(1).next().unwrap();
            for word in numbers.split(" ") {
                let word = word.trim();
                match word {
                    "" => continue,
                    "|" => is_winning = false,
                    num => {
                        let num = num.parse().unwrap();
                        if is_winning {
                            card.winning.push(num);
                        } else {
                            card.own.push(num);
                        }
                    }
                }
            }
            return card;
        })
        .collect::<Vec<Card>>();

    let part1_points: usize = cards
        .iter()
        .map(|card| {
            let mut point = 0;
            for own in &card.own {
                if card.winning.contains(&own) {
                    if point == 0 {
                        point = 1;
                    } else {
                        point *= 2;
                    }
                }
            }
            return point;
        })
        .sum();

    println!("Part 1 point is: {}", part1_points);

    // part2

    let mut card_counts = vec![];
    card_counts.resize(cards.len(), 1);
    // you start with 6 cards
    cards.sort_by_key(|c| c.id);
    for card in &cards {
        println!(
            "Id {}, Score {}, Count {}",
            card.id,
            card.score(),
            card_counts[card.id - 1]
        );
        if card_counts[card.id - 1] == 0 {
            break;
        }
        let score = card.score();
        for i in 0..score {
            card_counts[card.id + i] += card_counts[card.id - 1];
        }
    }

    println!("Part 2 score is: {}", card_counts.iter().sum::<usize>());

    return Ok(());
}
