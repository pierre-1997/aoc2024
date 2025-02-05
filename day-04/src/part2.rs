const VALID_PATTERNS: [[char; 5]; 4] = [
    ['M', 'S', 'A', 'M', 'S'],
    ['S', 'M', 'A', 'S', 'M'],
    ['M', 'M', 'A', 'S', 'S'],
    ['S', 'S', 'A', 'M', 'M'],
];

pub fn process(input: &str) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let nb_lines = input.lines().count();

    let input: Vec<char> = input.lines().map(|line| line.chars()).flatten().collect();
    let mut out = 0;

    // Horizontal
    for first_line in 0..=(nb_lines - 3) {
        for first_col in 0..=(line_len - 3) {
            let chunk = [
                input[first_line * line_len + first_col],
                input[first_line * line_len + first_col + 2],
                input[(first_line + 1) * line_len + first_col + 1],
                input[(first_line + 2) * line_len + first_col],
                input[(first_line + 2) * line_len + first_col + 2],
            ];

            if VALID_PATTERNS.contains(&chunk) {
                out += 1;
            }
        }
    }

    out
}

#[test]
fn test_day_04_part_2() {
    let test_input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    assert_eq!(process(test_input), 9);
}
