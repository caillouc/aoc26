use crate::utils::parser;

pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> i64 {
    let ranges: Vec<String> = parser::parse_vec_delimiter(input, ",");
    let mut sum: i64 = 0;
    ranges.iter().for_each(|range| {
        let bounds: Vec<i64> = range.split("-").map(|b| b.parse().unwrap()).collect();
        for i in bounds[0]..bounds[1] + 1 {
            let i_str = i.to_string();
            if i_str.len() % 2 == 0 {
                if i_str[..i_str.len() / 2] == i_str[i_str.len() / 2..] {
                    sum += i;
                }
            }
        }
    });
    sum
}

fn solve2(input: &String) -> i64 {
    let ranges: Vec<String> = parser::parse_vec_delimiter(input, ",");
    let mut sum: i64 = 0;
    ranges.iter().for_each(|range| {
        let bounds: Vec<i64> = range.split("-").map(|b| b.parse().unwrap()).collect();
        for to_check in bounds[0]..bounds[1] + 1 {
            let to_check_str = to_check.to_string();
            for i in 1..to_check_str.len() / 2 + 1{
                if to_check_str[to_check_str.len() - i..].to_string()
                    + &to_check_str[0..to_check_str.len() - i]
                    == to_check_str
                {
                    sum += to_check;
                    break;
                }
            }
        }
    });
    sum
}

#[test]
fn part_one() {
    let data = String::from(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    );

    let res = solve1(&data);
    assert_eq!(1227775554, res)
}

#[test]
fn part_two() {
    let data = String::from(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    );

    let res = solve2(&data);
    assert_eq!(4174379265, res)
}
