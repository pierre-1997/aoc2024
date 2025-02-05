use regex::Regex;

pub fn process(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|catch| {
            catch.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * catch.get(2).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum()
}

#[test]
fn test_day_01() {
    let test_input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    assert_eq!(process(test_input), 161);
}
