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

fn get_highest_possible_number(sequence: &[u8], num_digits: u32) -> Option<u64> {
    for digit in (0..=9).rev() {
        if let Some((position, digit)) = sequence.iter().enumerate().find(|v| *v.1 == digit) {
            if num_digits <= 1 {
                return Some(*digit as u64);
            }

            if let Some(following) =
                get_highest_possible_number(&sequence[(position + 1)..], num_digits - 1)
            {
                return Some(*digit as u64 * 10_u64.pow(num_digits - 1) + following);
            }
        }
    }

    None
}

impl BatteryBank {
    fn get_maximum_sequential_joltage(&self, num_digits: u32) -> u64 {
        get_highest_possible_number(&self.batteries, num_digits).unwrap()
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
    fn get_maximum_joltage(&self, num_digits: u32) -> u64 {
        self.banks
            .iter()
            .map(|bank| bank.get_maximum_sequential_joltage(num_digits))
            .sum()
    }
}

type Data = BatteryBankCollection;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test03.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(357), Answer::Number(3121910778619))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_maximum_joltage(2))
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_maximum_joltage(12))
    }
}
