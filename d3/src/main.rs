use std::{collections::HashMap, env, fs};

fn is_indexable(x: isize, y: isize, x_max: isize, y_max: isize) -> bool {
    return x >= 0 && y >= 0 && x < x_max && y < y_max;
}

fn is_symbol(x: u8) -> bool {
    let x = x as char;
    return x != '.' && !x.is_digit(10);
}

fn is_gear(x: u8) -> bool {
    let x = x as char;
    return x == '*';
}

fn check(input: &Vec<&str>, x: isize, y: isize) -> bool {
    let y_max = input.len() as isize;
    let x_max = input[0].len() as isize;
    if is_indexable(x, y, x_max, y_max) {
        return is_symbol(input[y as usize].as_bytes()[x as usize]);
    } else {
        return false;
    }
}

fn check_and_update_gear(
    input: &Vec<&str>,
    val: usize,
    map: &mut HashMap<String, (usize, usize)>,
    x: isize,
    y: isize,
) -> bool {
    let y_max = input.len() as isize;
    let x_max = input[0].len() as isize;
    if is_indexable(x, y, x_max, y_max) && is_gear(input[y as usize].as_bytes()[x as usize]) {
        let hash = get_gear_hash(x, y);
        let old_val = map.get(&hash);
        if let Some(old_val) = old_val {
            map.insert(hash, (old_val.0 + 1, old_val.1 * val));
        } else {
            map.insert(hash, (1, val));
        }
        return true;
    }
    return false;
}

fn has_adjacent_symbol(input: &Vec<&str>, (x, y): (isize, isize), num_length: usize) -> bool {
    for i in 0..num_length {
        let i = i as isize;
        if check(input, x + i + 1, y)
            || check(input, x + i - 1, y)
            || check(input, x + i, y + 1)
            || check(input, x + i, y - 1)
            || check(input, x + i + 1, y + 1)
            || check(input, x + i + 1, y - 1)
            || check(input, x + i - 1, y + 1)
            || check(input, x + i - 1, y - 1)
        {
            return true;
        }
    }

    return false;
}

fn get_gear_hash(x: isize, y: isize) -> String {
    return format!("{x}_{y}");
}

fn check_for_adjacent_gear(
    input: &Vec<&str>,
    val: usize,
    map: &mut HashMap<String, (usize, usize)>,
    (x, y): (isize, isize),
    num_length: usize,
) -> bool {
    let mut reminder_map = HashMap::<String, bool>::new();
    for i in 0..num_length {
        let i = i as isize;
        let x_cur = x + i + 1;
        let y_cur = y;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i - 1;
        let y_cur = y;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i;
        let y_cur = y + 1;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i;
        let y_cur = y - 1;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i + 1;
        let y_cur = y + 1;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i + 1;
        let y_cur = y - 1;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i - 1;
        let y_cur = y + 1;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
        let x_cur = x + i - 1;
        let y_cur = y - 1;
        if !reminder_map
            .get(get_gear_hash(x_cur, y_cur).as_str())
            .unwrap_or(&false)
            && check_and_update_gear(input, val, map, x_cur, y_cur)
        {
            reminder_map.insert(get_gear_hash(x_cur, y_cur), true);
        }
    }

    return false;
}

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)?;
    let mut valids = vec![];

    let lines = content.lines().collect::<Vec<&str>>();
    let _ = lines.iter().enumerate().for_each(|(idx, l)| {
        let mut i = 0;
        'l1: while i < l.len() {
            let mut j = l.len();
            while j > i {
                let slice = &l[i..j];
                let mut chars = slice.chars();
                if chars.all(|ch| ch.is_digit(10)) {
                    if has_adjacent_symbol(&lines, (i as isize, idx as isize), j - i) {
                        valids.push(slice.parse::<usize>().unwrap());
                    }
                    i = j + 1;
                    continue 'l1;
                }
                j -= 1;
            }

            i += 1;
        }
    });

    let mut map: HashMap<String, (usize, usize)> = HashMap::new();
    let _ = lines.iter().enumerate().for_each(|(idx, l)| {
        let mut i = 0;
        'l1: while i < l.len() {
            let mut j = l.len();
            while j > i {
                let slice = &l[i..j];
                let mut chars = slice.chars();
                if chars.all(|ch| ch.is_digit(10)) {
                    check_for_adjacent_gear(
                        &lines,
                        slice.parse::<usize>().unwrap(),
                        &mut map,
                        (i as isize, idx as isize),
                        j - i,
                    );
                    i = j + 1;
                    continue 'l1;
                }
                j -= 1;
            }

            i += 1;
        }
    });

    println!(
        "{:?}",
        map.iter()
            .filter_map(|(_, v)| {
                if v.0 == 2 {
                    return Some(v.1);
                }
                return None;
            })
            .sum::<usize>()
    );

    return Ok(());
}
