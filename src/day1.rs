pub fn solve(input: String) {
    println!("Part one : {}", solve1(&input));
    println!("Part two : {}", solve2(&input));
}

fn solve1(input: &String) -> i32 {
    let mut zeros = 0;
    let mut counter = 50;
    input.lines().for_each(|mv| {
        if !mv.is_empty() {
            let value: i32 = mv[1..].parse().unwrap();
            if mv.starts_with('L') {
                counter -= value;
            } else {
                counter += value;
            }
            counter %= 100;
            if counter < 0 {
                counter += 100
            }
            if counter == 0 {
                zeros += 1;
            }
        }
    });
    zeros
}

fn solve2(input: &String) -> i32 {
    let mut zeros = 0;
    let mut counter = 50;
    input.lines().for_each(|mv| {
        if !mv.is_empty() {
            let value: i32 = mv[1..].parse().unwrap();
            if mv.starts_with('L') {
                counter -= value;
                if counter <= 0 {
                    zeros += (counter / 100).abs();
                    if counter + value > 0 {
                        // Count for the initial 0
                        zeros += 1
                    }
                }
            } else {
                counter += value;
                if counter >= 100 {
                    zeros += (counter / 100).abs();
                }
            }
            counter %= 100;
            if counter < 0 {
                counter += 100
            }
        }
    });
    zeros
}

#[test]
fn part_one() {
    let data = String::from(
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    );

    let res = solve1(&data);
    assert_eq!(3, res)
}

#[test]
fn part_two() {
    let data = String::from(
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    );

    let res = solve2(&data);
    assert_eq!(6, res)
}
