fn check_validity(serie: &[i32]) -> bool {
    match serie[0] < serie[1] {
        true => serie.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] < 4),
        false => serie.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] < 4),
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|serie| check_validity(serie))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let mut out = 0;

    for serie in input.lines().map(|line| {
        line.split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }) {
        match check_validity(&serie) {
            true => {
                out += 1;
            }
            false => {
                for i in 0..serie.len() {
                    let mut new_serie = serie.clone();
                    new_serie.remove(i);

                    if check_validity(&new_serie) {
                        out += 1;
                        break;
                    }
                }
            }
        }
    }

    out
}

#[test]
fn test_day_2() {
    let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    assert_eq!(part_1(input), 2);
    assert_eq!(part_2(input), 4);
}
