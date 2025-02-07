pub fn process(input: &str) -> usize {
    let mut parsing_rules = true;
    let (mut rules, mut updates) = (Vec::new(), Vec::new());

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let mut s = line.split('|');
            rules.push((
                s.next().unwrap().parse::<usize>().unwrap(),
                s.next().unwrap().parse::<usize>().unwrap(),
            ));
        } else {
            updates.push(
                line.split(',')
                    .map(|nb| nb.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }

    let mut corrected_ones = Vec::new();

    for update in updates {
        for (first, second) in &rules {
            match (
                update.iter().position(|v| v == first),
                update.iter().position(|v| v == second),
            ) {
                (Some(ia), Some(ib)) => {
                    if ia > ib {
                        let mut corrected = update.clone();

                        'again: loop {
                            for (first, second) in &rules {
                                match (
                                    corrected.iter().position(|v| v == first),
                                    corrected.iter().position(|v| v == second),
                                ) {
                                    (Some(ia), Some(ib)) => {
                                        if ia > ib {
                                            corrected.swap(ia, ib);
                                            continue 'again;
                                        }
                                    }
                                    _ => continue,
                                }
                            }

                            break;
                        }

                        corrected_ones.push(corrected);
                        break;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }

    corrected_ones
        .iter()
        .map(|updates| updates[updates.len() / 2])
        .sum()
}

#[test]
fn test_day_05_part_2() {
    let test_input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    assert_eq!(process(test_input), 123);
}
