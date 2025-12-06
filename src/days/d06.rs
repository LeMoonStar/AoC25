use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 6;

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Addition,
}

impl Operation {
    fn calculate(&self, numbers: &[usize]) -> usize {
        match self {
            Self::Multiply => numbers.iter().fold(1, |acc, v| acc * v),
            Self::Addition => numbers.iter().sum(),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Addition,
            "*" => Self::Multiply,
            _ => panic!("Invalid Operation"),
        }
    }
}

#[derive(Debug, Clone)]
struct HomeworkCalculation {
    numbers: Vec<usize>,
    operation: Operation,
}

impl From<&[&str]> for HomeworkCalculation {
    fn from(value: &[&str]) -> Self {
        Self {
            numbers: value[..value.len() - 1]
                .iter()
                .map(|v| v.parse().unwrap())
                .collect(),
            operation: Operation::from(*value.last().unwrap()),
        }
    }
}

impl HomeworkCalculation {
    pub fn get_result(&self) -> usize {
        self.operation.calculate(&self.numbers)
    }
}

#[derive(Debug, Clone)]
pub struct HomeworkSheet {
    calculations: Vec<HomeworkCalculation>,
}

impl From<&str> for HomeworkSheet {
    fn from(value: &str) -> Self {
        let split_lines = value
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let num_cols = split_lines.first().unwrap().len();

        Self {
            calculations: (0..num_cols)
                .map(|i| {
                    let column: Vec<&str> = split_lines.iter().map(|line| line[i]).collect();
                    HomeworkCalculation::from(column.as_slice())
                })
                .collect(),
        }
    }
}

impl HomeworkSheet {
    fn get_results(&self) -> Vec<usize> {
        self.calculations.iter().map(|v| v.get_result()).collect()
    }
}

type Data = HomeworkSheet;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test06.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(4277556), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_results().into_iter().sum::<usize>() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
