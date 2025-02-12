pub fn process(input: &str) -> usize {
    let mut valids_sum = 0;

    for line in input.lines() {
        let result = line.split(':').next().unwrap().parse::<usize>().unwrap();
        let values: Vec<usize> = line
            .rsplit(':')
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        for mut i in 0..(3usize.pow(values.len() as u32 - 1)) {
            let mut r = values[0];

            for j in 0..(values.len() - 1) {
                let op = i % 3;
                i /= 3;

                match op {
                    0 => r += values[j + 1],
                    1 => r *= values[j + 1],
                    _ => r = format!("{}{}", r, values[j + 1]).parse().unwrap(),
                }
            }

            if r == result {
                valids_sum += result;
                break;
            }
        }
    }

    valids_sum
}

#[test]
fn test_day_07_part_2() {
    let test_input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(process(test_input), 11387);
}
