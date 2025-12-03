use std::collections::HashMap;

use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 3;

type Battery = u8;

#[derive(Debug, Clone)]
pub struct BatteryBank {
    batteries: Vec<Battery>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        Self {
            batteries: value
                .chars()
                .map(|v| v.to_digit(10).unwrap() as Battery)
                .collect(),
        }
    }
}

fn get_lowest_index<T: PartialEq>(sequence: &[T], value: &T) -> Option<usize> {
    Some(sequence.iter().enumerate().find(|(_, v)| *v == value)?.0)
}

fn get_highest_index<T: PartialEq>(sequence: &[T], value: &T) -> Option<usize> {
    Some(
        sequence
            .iter()
            .enumerate()
            .rev()
            .find(|(_, v)| *v == value)?
            .0,
    )
}

impl BatteryBank {
    fn get_maximum_sequential_joltage(&self) -> u64 {
        dprintln!("{:?}", self.batteries);

        let lowest_indexes = (0..=9)
            .map(|digit| (digit, get_lowest_index(&self.batteries, &digit)))
            .collect::<HashMap<_, _>>();

        let highest_indexes = (0..=9)
            .map(|digit| (digit, get_highest_index(&self.batteries, &digit)))
            .collect::<HashMap<_, _>>();

        let occurances = (0..=9)
            .map(|digit| {
                (
                    digit,
                    self.batteries.iter().filter(|v| **v == digit).count(),
                )
            })
            .collect::<HashMap<_, _>>();

        for first_digit in (0..=9).rev() {
            dprintln!("  FIRST DIGIT: {}", first_digit);
            if *occurances.get(&first_digit).unwrap() >= 2 {
                dprintln!("    Occurs at least twice!");
                dprintln!("==> {}", first_digit as u64 * 11);
                return first_digit as u64 * 11;
            }

            let first_digit_index = lowest_indexes.get(&first_digit).unwrap();
            dprintln!("    Index: {:?}", first_digit_index);

            if first_digit_index.is_none() {
                continue;
            }

            for second_digit in (0..=9).rev() {
                if first_digit == second_digit {
                    continue;
                }

                dprintln!("    SECOND DIGIT: {}", second_digit);
                let second_digit_index = highest_indexes.get(&second_digit).unwrap();
                dprintln!("      Index: {:?}", second_digit_index);

                if second_digit_index.is_none() {
                    continue;
                }

                if second_digit_index.unwrap() > first_digit_index.unwrap() {
                    dprintln!("==> {}{}", first_digit, second_digit);
                    return first_digit as u64 * 10 + second_digit as u64;
                }
            }
        }

        panic!("Well fuck");
    }
}

#[derive(Debug, Clone)]
pub struct BatteryBankCollection {
    banks: Vec<BatteryBank>,
}

impl From<&str> for BatteryBankCollection {
    fn from(value: &str) -> Self {
        Self {
            banks: value.lines().map(|v| v.into()).collect(),
        }
    }
}

impl BatteryBankCollection {
    fn get_maximum_joltage(&self) -> u64 {
        self.banks
            .iter()
            .map(|bank| bank.get_maximum_sequential_joltage())
            .sum()
    }
}

type Data = BatteryBankCollection;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test03.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(357), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_maximum_joltage())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
