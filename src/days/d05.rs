use std::collections::HashSet;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 5;

#[derive(Debug, Clone)]
pub struct Inventory {
    fresh_ranges: Vec<(usize, usize)>,
    available_ingredients: Vec<usize>,
}

impl From<&str> for Inventory {
    fn from(value: &str) -> Self {
        let (fresh_ranges_str, available_ingredients_str) = value.split_once("\n\n").unwrap();

        Self {
            fresh_ranges: fresh_ranges_str
                .lines()
                .map(|line| {
                    let (start, end) = line.split_once('-').unwrap();
                    (start.parse().unwrap(), end.parse().unwrap())
                })
                .collect(),
            available_ingredients: available_ingredients_str
                .lines()
                .map(|v| v.parse().unwrap())
                .collect(),
        }
    }
}

impl Inventory {
    fn is_fresh(&self, ingredient_id: usize) -> bool {
        for range in &self.fresh_ranges {
            if range.0 <= ingredient_id && ingredient_id <= range.1 {
                return true;
            }
        }

        false
    }

    fn get_available_fresh_ingredients(&self) -> Vec<usize> {
        self.available_ingredients
            .clone()
            .into_iter()
            .filter(|v| self.is_fresh(*v))
            .collect()
    }
}

type Data = Inventory;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test05.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_available_fresh_ingredients().len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
