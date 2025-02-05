pub fn process(input: &str) -> usize {
    let (mut first, mut second) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let splitted: Vec<i32> = line
            .split_whitespace()
            .map(|splitted| splitted.parse::<i32>().unwrap())
            .collect();
        first.push(splitted[0]);
        second.push(splitted[1]);
    }

    first.sort();
    second.sort();

    first
        .iter()
        .zip(second)
        .map(|(f, s)| (s - f).abs())
        .sum::<i32>() as usize
}

#[test]
fn test_day_1() {
    let test_input = r"3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(process(test_input), 11);
}
