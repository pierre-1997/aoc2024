enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn increment(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

// NOTE: We maybe could have used a `fold()` instead of recursion ?
fn guard(map: &mut Vec<char>, w: i32, l: i32, pos: i32, dir: Direction) -> usize {
    let new_pos = match dir {
        Direction::Up => pos - w,
        Direction::Right => pos + 1,
        Direction::Down => pos + w,
        Direction::Left => pos - 1,
    };

    if new_pos < 0 || new_pos >= l {
        return 0;
    }

    match map[new_pos as usize] {
        '#' => guard(map, w, l, pos, dir.increment()),
        '.' => {
            map[new_pos as usize] = 'X';
            1 + guard(map, w, l, new_pos, dir)
        }
        'X' | '^' => guard(map, w, l, new_pos, dir),
        x => unreachable!("Map should only contains '#' and '.', found '{x}'."),
    }
}

pub fn process(input: &str) -> usize {
    let w = input.lines().next().unwrap().len();
    let mut sinput: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let l = sinput.len();

    // Always starts facing upwards
    let p = sinput.iter().position(|c| *c == '^').unwrap() as i32;

    1 + guard(&mut sinput, w as i32, l as i32, p, Direction::Up)
}

#[test]
fn test_day_06_part_1() {
    let test_input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    assert_eq!(process(test_input), 41);
}
