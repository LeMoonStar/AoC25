use crate::dprintln;

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
    fn is_in_range(value: usize, range: &(usize, usize)) -> bool {
        range.0 <= value && value <= range.1
    }

    fn is_fresh(&self, ingredient_id: usize) -> bool {
        for range in &self.fresh_ranges {
            if Self::is_in_range(ingredient_id, range) {
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

    fn get_fresh_ingredient_count(&self) -> usize {
        let mut ranges = self.fresh_ranges.clone();

        ranges.sort_by_key(|k| k.0);

        let mut merged_ranges: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());
        merged_ranges.push(ranges[0]);

        for (start, end) in ranges.into_iter().skip(1) {
            dprintln!("RANGE: {} - {}", start, end);
            if let Some(last_range) = merged_ranges.last_mut() {
                if start <= last_range.1 + 1 {
                    dprintln!(
                        "  Starts within last range (which ends at {})",
                        last_range.1
                    );
                    dprintln!("    Extending last range to {}", end.max(last_range.1));
                    last_range.1 = end.max(last_range.1);
                } else {
                    dprintln!("  No overlap. adding.");
                    merged_ranges.push((start, end));
                }
            }
        }

        dprintln!("=> {:?}", merged_ranges);

        merged_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum()
    }
}

type Data = Inventory;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test05.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(14))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_available_fresh_ingredients().len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_fresh_ingredient_count() as u64)
    }
}
