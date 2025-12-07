use std::collections::{HashMap, HashSet};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

#[derive(Debug, Clone)]
pub struct TachyonManifold {
    splitters: HashMap<(usize, usize), bool>,
    start_position: (usize, usize),
    depth: usize,
}

impl From<&str> for TachyonManifold {
    fn from(value: &str) -> Self {
        let (splitters, start_position) = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, char)| (x, y, char)))
            .fold(
                (HashMap::new(), None),
                |(mut splitters, mut start_pos), (x, y, char)| {
                    match char {
                        'S' => start_pos = Some((x, y)),
                        '^' => {
                            splitters.insert((x, y), true);
                        }
                        _ => {}
                    }
                    (splitters, start_pos)
                },
            );

        let depth = splitters.keys().map(|(_, y)| *y).max().unwrap() + 1;

        Self {
            splitters,
            start_position: start_position.unwrap(),
            depth,
        }
    }
}

impl TachyonManifold {
    fn perform_splits(&self, rays: HashSet<usize>, layer: usize) -> (usize, HashSet<usize>) {
        let mut splits = 0;
        let new_rays = rays
            .iter()
            .flat_map(|ray| {
                if let Some(_) = self.splitters.get(&(*ray, layer)) {
                    splits += 1;
                    vec![ray - 1, ray + 1]
                } else {
                    vec![*ray]
                }
            })
            .collect();

        return (splits, new_rays);
    }

    fn trace_rays(&self) -> (usize, HashSet<usize>) {
        let (mut splits, mut rays) = (0, HashSet::new());
        rays.insert(self.start_position.0);

        for layer in 0..self.depth {
            let (new_splits, new_rays) = self.perform_splits(rays, layer);
            rays = new_rays;
            splits += new_splits;
        }

        (splits, rays)
    }
}

type Data = TachyonManifold;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test07.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(21), Answer::Number(40))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.trace_rays().0 as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
