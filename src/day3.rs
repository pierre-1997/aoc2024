use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|catch| {
            catch.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * catch.get(2).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do(?:n't)?\(\)").unwrap();
    let mut todo = true;
    let mut ret = 0;

    for catch in re.captures_iter(input) {
        match catch.get(0).unwrap().as_str() {
            "do()" => {
                todo = true;
            }
            "don't()" => {
                todo = false;
            }
            _ => {
                if todo {
                    ret += catch.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * catch.get(2).unwrap().as_str().parse::<usize>().unwrap();
                }
            }
        }
    }

    ret
}

#[test]
fn test_day_3() {
    let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part_1(input), 161);

    let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part_2(input), 48);
}
