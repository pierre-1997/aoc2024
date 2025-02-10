#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

fn guard_path(
    map: &mut Vec<char>,
    w: i32,
    l: i32,
    pos: i32,
    dir: Direction,
    pathed: &mut Vec<usize>,
) {
    let new_pos = match dir {
        Direction::Up => pos - w,
        Direction::Right => pos + 1,
        Direction::Down => pos + w,
        Direction::Left => pos - 1,
    };

    if new_pos < 0 || new_pos >= l {
        return;
    }

    match map[new_pos as usize] {
        '#' => guard_path(map, w, l, pos, dir.increment(), pathed),
        '.' => {
            map[new_pos as usize] = 'X';
            pathed.push(new_pos as usize);
            guard_path(map, w, l, new_pos, dir, pathed)
        }
        // Can't put an obstacle in the starting position of the guard.
        '^' => {
            map[new_pos as usize] = 'X';
            guard_path(map, w, l, new_pos, dir, pathed)
        }
        'X' => guard_path(map, w, l, new_pos, dir, pathed),
        x => unreachable!("Map should only contains '#' and '.', found '{x}'."),
    }
}

fn guard_loop(
    map: &mut Vec<char>,
    w: i32,
    l: i32,
    pos: i32,
    dir: Direction,
    pathed: &mut Vec<(usize, Direction)>,
) -> bool {
    let Some(new_pos) = (match dir {
        Direction::Up => ((pos - w) >= 0).then_some(pos - w),
        Direction::Right => ((pos + 1) % w != 0).then_some(pos + 1),
        Direction::Down => ((pos + w) < l).then_some(pos + w),
        Direction::Left => ((((pos - 1) % w) + w) % w != (w - 1)).then_some(pos - 1),
    }) else {
        return false;
    };

    match map[new_pos as usize] {
        '#' | 'O' => guard_loop(map, w, l, pos, dir.increment(), pathed),
        '.' => {
            map[new_pos as usize] = 'X';
            pathed.push((new_pos as usize, dir));
            guard_loop(map, w, l, new_pos, dir, pathed)
        }
        '^' => {
            // Returned to the starting position
            if dir == Direction::Up {
                return true;
            }

            map[new_pos as usize] = 'X';
            pathed.push((new_pos as usize, dir));
            guard_loop(map, w, l, new_pos, dir, pathed)
        }
        'X' => {
            if pathed.contains(&(new_pos as usize, dir)) {
                return true;
            }

            guard_loop(map, w, l, new_pos, dir, pathed)
        }
        x => unreachable!("Map should only contains '#' and '.', found '{x}'."),
    }
}

pub fn process(input: &str) -> usize {
    let w = input.lines().next().unwrap().len();
    let initial_map: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let l = initial_map.len();

    // Always starts facing upwards
    let p = initial_map.iter().position(|c| *c == '^').unwrap() as i32;

    // First, get the list of each position pathed by the guard in the initial map.
    let mut path = Vec::new();
    guard_path(
        &mut initial_map.clone(),
        w as i32,
        l as i32,
        p,
        Direction::Up,
        &mut path,
    );

    // Then, try switching each pathed position to an obstacle and check if it makes a loop.
    path.iter().fold(0, |acc, pos| {
        let mut map_copy = initial_map.clone();
        map_copy[*pos] = 'O';

        match guard_loop(
            &mut map_copy,
            w as i32,
            l as i32,
            p,
            Direction::Up,
            &mut vec![(p as usize, Direction::Up)],
        ) {
            true => {
                // println!("Position #{} = {}", acc, pos);
                acc + 1
            }
            false => acc,
        }
    })
}

#[test]
fn test_day_06_part_2() {
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

    assert_eq!(process(test_input), 6);
}
