use std::collections::HashMap;

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
            Self::Multiply => numbers.iter().product(),
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

impl HomeworkCalculation {
    pub fn normal_from_column(value: &[String]) -> Self {
        dprintln!("normal_from_column: {:?}", value);
        Self {
            numbers: value[..value.len() - 1]
                .iter()
                .map(|v| v.trim().parse().unwrap())
                .collect(),
            operation: Operation::from(value.last().unwrap().as_str().trim()),
        }
    }

    pub fn cephalopod_from_column(value: &[String]) -> Self {
        dprintln!("cephalopod_from_column: {:?}", value);
        let mut numbers: HashMap<usize, usize> =
            HashMap::with_capacity(value.first().unwrap().len());

        value.iter().for_each(|line| {
            line.chars().enumerate().for_each(|(char_num, char)| {
                if !char.is_ascii_digit() {
                    return;
                }
                let mut num = *numbers.get(&char_num).unwrap_or(&0);

                dprintln!("num {:?} char_num {:?} char {:?}", num, char_num, char);

                num = num * 10 + char.to_digit(10).unwrap() as usize;

                numbers.insert(char_num, num);
            })
        });

        dprintln!("cephalopod_from_column result: {:?}\n\n", numbers);

        Self {
            numbers: numbers.into_values().collect(),
            operation: Operation::from(value.last().unwrap().as_str().trim()),
        }
    }

    pub fn get_result(&self) -> usize {
        self.operation.calculate(&self.numbers)
    }
}

#[derive(Debug, Clone)]
pub struct HomeworkSheet {
    columns: Vec<Vec<String>>,
}

impl From<&str> for HomeworkSheet {
    fn from(value: &str) -> Self {
        let last_line = value.lines().last().unwrap();
        let column_starts: Vec<usize> = last_line
            .match_indices(|c: char| !c.is_whitespace())
            .map(|(i, _)| i)
            .collect();

        dprintln!("Column starts: {:?}", column_starts);

        let mut split_lines: Vec<Vec<String>> = Vec::with_capacity(value.lines().count());
        for line in value.lines() {
            let mut split_line = Vec::with_capacity(column_starts.len());

            dprintln!("LINE {:?}", line);

            for i in 0..column_starts.len() {
                let start = column_starts[i];
                let end = column_starts.get(i + 1).copied().unwrap_or(line.len());

                split_line.push(line[start..end.min(line.len())].to_owned());
            }

            split_lines.push(split_line)
        }

        dprintln!("{:?}", split_lines);

        let num_cols = split_lines.first().unwrap().len();

        Self {
            columns: (0..num_cols)
                .map(|i| split_lines.iter().map(|line| line[i].to_owned()).collect())
                .collect(),
        }
    }
}

impl HomeworkSheet {
    fn get_normal_homework_calculations(&self) -> Vec<HomeworkCalculation> {
        self.columns
            .iter()
            .map(|v| HomeworkCalculation::normal_from_column(v))
            .collect()
    }

    fn get_cephalopod_homework_calculations(&self) -> Vec<HomeworkCalculation> {
        self.columns
            .iter()
            .map(|v| HomeworkCalculation::cephalopod_from_column(v))
            .collect()
    }

    fn get_normal_results(&self) -> Vec<usize> {
        self.get_normal_homework_calculations()
            .iter()
            .map(|v| v.get_result())
            .collect()
    }

    fn get_cephalopod_results(&self) -> Vec<usize> {
        self.get_cephalopod_homework_calculations()
            .iter()
            .map(|v| v.get_result())
            .collect()
    }
}

type Data = HomeworkSheet;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test06.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(4277556), Answer::Number(3263827))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_normal_results().into_iter().sum::<usize>() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_cephalopod_results().into_iter().sum::<usize>() as u64)
    }
}
