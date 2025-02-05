const NORMAL: [char; 4] = ['X', 'M', 'A', 'S'];
const REVERSED: [char; 4] = ['S', 'A', 'M', 'X'];

/// OPTI: We could process all the iterators and then just check the corresponding chunks.
pub fn process(input: &str) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let nb_lines = input.lines().count();

    let input: Vec<char> = input.lines().map(|line| line.chars()).flatten().collect();
    let mut out = 0;

    // Horizontal
    for line in 0..nb_lines {
        for idx in 0..=(line_len - 4) {
            let chunk = &input[(line * line_len + idx)..(line * line_len + idx + 4)];
            if chunk == NORMAL || chunk == REVERSED {
                out += 1;
            }
        }
    }

    // Vertical
    for col in 0..line_len {
        for idx in 0..=(nb_lines - 4) {
            let chunk = [
                input[col + idx * line_len],
                input[col + idx * line_len + line_len],
                input[col + idx * line_len + 2 * line_len],
                input[col + idx * line_len + 3 * line_len],
            ];

            if chunk == NORMAL || chunk == REVERSED {
                out += 1;
            }
        }
    }

    // Right Diagonal
    for first_col in 0..=(line_len - 4) {
        for first_line in 0..=(nb_lines - 4) {
            let chunk = [
                input[first_col + first_line * line_len],
                input[first_col + first_line * line_len + 1 + line_len],
                input[first_col + first_line * line_len + 2 + 2 * line_len],
                input[first_col + first_line * line_len + 3 + 3 * line_len],
            ];

            if chunk == NORMAL || chunk == REVERSED {
                out += 1;
            }
        }
    }

    // Left Diagonal
    for last_col in 3..line_len {
        for first_line in 0..=(nb_lines - 4) {
            let chunk = [
                input[last_col + first_line * line_len],
                input[last_col + first_line * line_len - 1 + line_len],
                input[last_col + first_line * line_len - 2 + 2 * line_len],
                input[last_col + first_line * line_len - 3 + 3 * line_len],
            ];

            if chunk == NORMAL || chunk == REVERSED {
                out += 1;
            }
        }
    }

    out
}

#[test]
fn test_day_04_part_1() {
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

    assert_eq!(process(test_input), 18);
}
