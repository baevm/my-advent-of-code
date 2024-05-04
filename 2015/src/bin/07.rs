use std::collections::HashMap;

advent_of_code::solution!(7);

struct Signals {
    wires: HashMap<String, u16>,
}

impl Signals {
    fn default() -> Self {
        Self {
            wires: HashMap::new(),
        }
    }

    fn add(&mut self, signal: &str, end: &str) {
        let is_signal_numeric = is_numeric_str(signal);

        if self.wires.contains_key(end) {
            return;
        }

        if is_signal_numeric {
            let signal: u16 = signal.parse().unwrap();
            self.wires.insert(end.to_string(), signal);
        } else {
            let ch_signal = self.wires.get(signal);

            match ch_signal {
                Some(sig) => {
                    self.wires.insert(end.to_string(), *sig);
                }
                None => return,
            }
        }
    }

    fn signal_not(&mut self, start: &str, end: &str) {
        let ch1_signal = self.wires.get(start);

        if ch1_signal.is_none() {
            return;
        }

        let res = !ch1_signal.unwrap();
        self.wires.insert(end.to_string(), res);
    }

    fn signal_or(&mut self, left: &str, right: &str, end: &str) {
        let ch1 = self.wires.get(left);
        let ch2 = self.wires.get(right);

        if ch1.is_none() || ch2.is_none() {
            return;
        }

        let res = ch1.unwrap() | ch2.unwrap();
        self.wires.insert(end.to_string(), res);
    }

    fn signal_and(&mut self, left: &str, right: &str, end: &str) {
        // AND operation can be like this: 1 AND am -> an
        let is_ch1_numeric = is_numeric_str(left);
        let mut ch1 = 0;
        let mut ch2 = 0;

        if is_ch1_numeric {
            ch1 = left.parse().unwrap();
            let ch2_option = self.wires.get(right);

            if ch2_option.is_none() {
                return;
            }

            ch2 = *ch2_option.unwrap();
        } else {
            let ch1_option = self.wires.get(left);
            let ch2_option = self.wires.get(right);

            if ch1_option.is_none() || ch2_option.is_none() {
                return;
            }

            ch1 = *ch1_option.unwrap();
            ch2 = *ch2_option.unwrap();
        }

        let res = ch1 & ch2;
        self.wires.insert(end.to_string(), res);
    }

    fn signal_lshift(&mut self, left: &str, value: &u16, end: &str) {
        let ch1 = self.wires.get(left);

        if ch1.is_none() {
            return;
        }

        let res = ch1.unwrap() << value;
        self.wires.insert(end.to_string(), res);
    }

    fn signal_rshift(&mut self, left: &str, value: &u16, end: &str) {
        let ch1 = self.wires.get(left);

        if ch1.is_none() {
            return;
        }

        let res = ch1.unwrap() >> value;
        self.wires.insert(end.to_string(), res);
    }
}

fn solve(input: &str, signals: &mut Signals) -> Option<u16> {
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
            3 => {
                let signal = args[0];
                let end = args[2];
                signals.add(signal, end)
            }
            4 => {
                let start = args[1];
                let end = args[3];
                signals.signal_not(start, end)
            }
            5 => {
                let left = args[0];
                let operation = args[1];
                let right = args[2];
                let end = args[4];

                match operation {
                    "AND" => {
                        signals.signal_and(left, right, end);
                    }
                    "OR" => {
                        signals.signal_or(left, right, end);
                    }
                    "LSHIFT" => {
                        let value: u16 = right.parse().unwrap();
                        signals.signal_lshift(left, &value, end);
                    }
                    "RSHIFT" => {
                        let value: u16 = right.parse().unwrap();
                        signals.signal_rshift(left, &value, end);
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

        if signals.wires.contains_key("a") {
            return signals.wires.get("a").copied();
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut signals = Signals::default();
    solve(input, &mut signals)
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut signals = Signals::default();

    let a_signal = solve(input, &mut signals).unwrap();

    signals.wires.clear();
    signals.wires.insert("b".to_string(), a_signal);

    solve(input, &mut signals)
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
