fn compute_results(values: &[usize]) -> Vec<usize> {
    if values.is_empty() {
        return Vec::new();
    }

    if values.len() == 1 {
        return vec![values[0]];
    }

    let mut ret = Vec::new();

    for v in compute_results(&values[0..(values.len() - 1)]) {
        ret.push(values[values.len() - 1] + v);
        ret.push(values[values.len() - 1] * v);
    }

    ret
}

pub fn process(input: &str) -> usize {
    let mut valids = Vec::new();

    for line in input.lines() {
        let result = line.split(':').next().unwrap().parse::<usize>().unwrap();
        let values: Vec<usize> = line
            .rsplit(':')
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        if compute_results(&values).contains(&result) {
            valids.push(result);
        }
    }

    valids.iter().sum()
}

#[test]
fn test_day_07_part_1() {
    let test_input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(process(test_input), 3749);
}
