use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Future,
    Past,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reading {
    nums: VecDeque<i64>,
}

impl Reading {
    pub fn analyze(&self, mode: Mode) -> i64 {
        let mut deque = VecDeque::new();
        deque.push_back(self.nums.clone());
        let mut frame = VecDeque::new();

        while let Some(front) = deque.front() {
            if front.iter().all(|&x| x == 0) {
                break;
            }
            for i in 0..front.len() - 1 {
                frame.push_back(front[i + 1] - front[i]);
            }
            deque.push_front(frame);
            frame = VecDeque::new();
        }

        while deque.len() > 1 {
            frame = deque.pop_front().unwrap();
            let front = deque.front_mut().unwrap();

            match mode {
                Mode::Future => {
                    let i = front.back().unwrap();
                    let j = frame.back().unwrap();
                    front.push_back(i + j);
                }
                Mode::Past => {
                    let i = front.front().unwrap();
                    let j = frame.front().unwrap();
                    front.push_front(i - j);
                }
            };
        }

        let front = deque.front().unwrap();
        match mode {
            Mode::Future => *front.back().unwrap(),
            Mode::Past => *front.front().unwrap(),
        }
    }
}

impl FromStr for Reading {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let nums = s.split(' ').map(i64::from_str).collect::<Result<_, _>>()?;

        Ok(Self { nums })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_DATA: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45\n";

    #[test]
    fn parse_reading() {
        let expected = vec![
            Reading {
                nums: VecDeque::from([0, 3, 6, 9, 12, 15]),
            },
            Reading {
                nums: VecDeque::from([1, 3, 6, 10, 15, 21]),
            },
            Reading {
                nums: VecDeque::from([10, 13, 16, 21, 30, 45]),
            },
        ];

        for (s, expected) in EXAMPLE_DATA.lines().zip(expected) {
            let reading = Reading::from_str(s).unwrap();
            assert_eq!(reading, expected);
        }
    }

    #[test]
    fn reading_analyze_future() {
        let expected = vec![18, 28, 68];

        for (s, expected) in EXAMPLE_DATA.lines().zip(expected) {
            let reading = Reading::from_str(s).unwrap();
            assert_eq!(reading.analyze(Mode::Future), expected);
        }
    }

    #[test]
    fn reading_analyze_past() {
        let expected = vec![-3, 0, 5];

        for (s, expected) in EXAMPLE_DATA.lines().zip(expected) {
            let reading = Reading::from_str(s).unwrap();
            assert_eq!(reading.analyze(Mode::Past), expected);
        }
    }
}
