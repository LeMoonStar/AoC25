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
    fn get_neighbours(&self, subject_position: (isize, isize)) -> HashMap<(isize, isize), bool> {
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

    fn count_accessible_paper_rolls(&self) -> usize {
        (0..self.width as isize)
            .map(|x| {
                (0..self.height as isize)
                    .filter(move |y| self.paper_rolls.contains_key(&(x, *y)))
                    .map(move |y| self.get_neighbours((x, y)).iter().count())
                    .filter(|neighbour_count| *neighbour_count < 4)
            })
            .flatten()
            .count()
    }
}

type Data = PaperDepartmentMap;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.count_accessible_paper_rolls() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
