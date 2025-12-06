use std::collections::HashMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone)]
pub struct PaperDepartmentMap {
    paper_rolls: HashMap<(isize, isize), bool>,
    width: usize,
    height: usize,
}

impl From<&str> for PaperDepartmentMap {
    fn from(value: &str) -> Self {
        let (mut width, mut height) = (0, 0);
        let mut paper_rolls = HashMap::new();

        value
            .lines()
            .enumerate()
            .inspect(|(y, _)| {
                if *y + 1 > height {
                    height = *y + 1
                }
            })
            .for_each(|(y, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|(x, _)| {
                        if *x + 1 > width {
                            width = *x + 1
                        }
                    })
                    .for_each(|(x, char)| {
                        if char == '@' {
                            paper_rolls.insert((x as isize, y as isize), true);
                        }
                    })
            });

        Self {
            paper_rolls,
            width,
            height,
        }
    }
}

impl PaperDepartmentMap {
    fn get_neighbours(&self, subject_position: &(isize, isize)) -> HashMap<(isize, isize), bool> {
        let mut neighbours = HashMap::with_capacity(8);

        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                let position = (subject_position.0 + x, subject_position.1 + y);

                if let Some(paper_roll) = self.paper_rolls.get(&position) {
                    neighbours.insert(position, *paper_roll);
                }
            }
        }

        neighbours
    }

    fn get_all_positions(&self) -> Vec<(isize, isize)> {
        (0..self.width as isize)
            .map(|x| (0..self.height as isize).map(move |y| (x, y)))
            .flatten()
            .collect()
    }

    fn get_accessible_paper_rolls(&self) -> Vec<(isize, isize)> {
        self.get_all_positions()
            .into_iter()
            .filter(|position| self.paper_rolls.contains_key(&position))
            .filter(|position| self.get_neighbours(&position).iter().count() < 4)
            .collect()
    }

    fn count_accessible_paper_rolls(&self) -> usize {
        self.get_accessible_paper_rolls().iter().count()
    }

    fn try_remove_all(&mut self) -> usize {
        let mut total_removed = 0;

        loop {
            let to_remove = self.get_accessible_paper_rolls();
            let to_remove_count = to_remove.iter().count();

            if to_remove_count == 0 {
                return total_removed;
            }

            to_remove.iter().for_each(|position| {
                self.paper_rolls.remove(&position);
            });

            total_removed += to_remove.iter().count();
        }
    }
}

type Data = PaperDepartmentMap;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(43))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.count_accessible_paper_rolls() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.try_remove_all() as u64)
    }
}
