use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let code = line
                .chars()
                .filter(|ch| ch.is_numeric())
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>();

            let mut first = String::from(code.first().unwrap());
            let last = code.last().unwrap();

            first.push_str(last);
            first.parse::<u32>().unwrap()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let sum = input
        .lines()
        .map(|line| {
            println!("line: {}", line);
            let mut indexes: HashMap<usize, i32> = HashMap::new();
            for number in numbers.keys() {
                let number_int = numbers.get(number).unwrap();

                line.match_indices(number).for_each(|indi| {
                    indexes.insert(indi.0, *number_int);
                });

                line.match_indices(number_int.to_string().as_str())
                    .for_each(|indi| {
                        indexes.insert(indi.0, *number_int);
                    });
            }

            let mut sorted_indexes: Vec<_> = indexes.into_iter().collect();
            sorted_indexes.sort_by(|a, b| a.0.cmp(&b.0));

            let wanted: Option<u32> = match sorted_indexes.len() {
                0 => {
                    dbg!("here");
                    None
                },
                _ => {
                    let wanted = format!(
                        "{}{}",
                        sorted_indexes.first().unwrap().1.to_owned(),
                        sorted_indexes.last().unwrap().1.to_owned()
                    )
                    .parse::<u32>()
                    .unwrap();

                    Some(wanted)
                }
            };

            println!("wanted: {}", wanted.unwrap());

            wanted.unwrap()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(55447));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(54706));
    }
}
