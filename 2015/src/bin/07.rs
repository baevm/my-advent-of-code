use std::collections::HashMap;

advent_of_code::solution!(7);

fn solve<'a, 'b>(input: &'a str, char_signals: &'b mut HashMap<&'a str, u16>) -> Option<u16> {
    let lines: Vec<&str> = input.lines().collect();

    let mut i = 0;
    let n = lines.len() - 1;

    loop {
        let line = lines[i];

        let args: Vec<&str> = line.split_whitespace().collect();

        // signal operations examples and its length:
        // 3 == 123 -> x
        // 4 == NOT x -> h
        // 5 == x LSHIFT 2 -> f
        match args.len() {
            3 => 'label3: {
                let result_ch = args[2];
                let is_signal_numeric = is_numeric_str(args[0]);

                if char_signals.contains_key(result_ch) {
                    break 'label3;
                }

                if is_signal_numeric {
                    let signal: u16 = args[0].parse().unwrap();
                    char_signals.insert(result_ch, signal);
                } else {
                    let ch_signal = char_signals.get(args[0]);

                    match ch_signal {
                        Some(sig) => {
                            char_signals.insert(result_ch, *sig);
                        }
                        None => break 'label3,
                    }
                }
            }
            4 => 'label4: {
                let operation = args[0];
                let ch1_signal = char_signals.get(args[1]);

                if ch1_signal.is_none() {
                    break 'label4;
                }

                let result_ch = args[3];

                match operation {
                    "NOT" => {
                        let res = !ch1_signal.unwrap();
                        char_signals.insert(result_ch, res);
                    }
                    _ => unreachable!(),
                }
            }
            5 => 'label5: {
                let operation = args[1];
                let result_ch = args[4];

                match operation {
                    "AND" => {
                        // AND operation can be like this: 1 AND am -> an
                        let is_ch1_numeric = is_numeric_str(args[0]);
                        let mut ch1 = 0;
                        let mut ch2 = 0;

                        if is_ch1_numeric {
                            ch1 = args[0].parse().unwrap();
                            let ch2_option = char_signals.get(args[2]);

                            if ch2_option.is_none() {
                                break 'label5;
                            }

                            ch2 = *ch2_option.unwrap();
                        } else {
                            let ch1_option = char_signals.get(args[0]);
                            let ch2_option = char_signals.get(args[2]);

                            if ch1_option.is_none() || ch2_option.is_none() {
                                break 'label5;
                            }

                            ch1 = *ch1_option.unwrap();
                            ch2 = *ch2_option.unwrap();
                        }

                        let res = ch1 & ch2;
                        char_signals.insert(result_ch, res);
                    }
                    "OR" => {
                        let ch1 = char_signals.get(args[0]);
                        let ch2 = char_signals.get(args[2]);

                        if ch1.is_none() || ch2.is_none() {
                            break 'label5;
                        }

                        let res = ch1.unwrap() | ch2.unwrap();
                        char_signals.insert(result_ch, res);
                    }
                    "LSHIFT" => {
                        let ch1 = char_signals.get(args[0]);

                        if ch1.is_none() {
                            break 'label5;
                        }

                        let ch2: u16 = args[2].parse().unwrap();
                        let res = ch1.unwrap() << ch2;
                        char_signals.insert(result_ch, res);
                    }
                    "RSHIFT" => {
                        let ch1 = char_signals.get(args[0]);

                        if ch1.is_none() {
                            break 'label5;
                        }

                        let ch2: u16 = args[2].parse().unwrap();
                        let res = ch1.unwrap() >> ch2;
                        char_signals.insert(result_ch, res);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }

        if i == n {
            i = 0;
        } else {
            i += 1;
        }

        if char_signals.contains_key("a") {
            return char_signals.get("a").copied();
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut char_signals: HashMap<&str, u16> = HashMap::new();
    solve(input, &mut char_signals)
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut char_signals: HashMap<&str, u16> = HashMap::new();

    let a_signal = solve(input, &mut char_signals).unwrap();

    char_signals.clear();
    char_signals.insert("b", a_signal);

    solve(input, &mut char_signals)
}

pub fn is_numeric_str(input: &str) -> bool {
    return input.chars().next().unwrap().is_numeric();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
