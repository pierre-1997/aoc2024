pub fn process(input: &str) -> usize {
    let (mut first, mut second) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let splitted: Vec<usize> = line
            .split_whitespace()
            .map(|splitted| splitted.parse::<usize>().unwrap())
            .collect();
        first.push(splitted[0]);
        second.push(splitted[1]);
    }

    first
        .iter()
        .map(|f| f * second.iter().filter(|s| *s == f).count())
        .sum()
}

#[test]
fn test_day_1_part_2() {
    let test_input = r"3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(process(test_input), 31);
}
