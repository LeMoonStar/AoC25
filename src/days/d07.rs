use std::collections::{HashMap, HashSet};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

#[derive(Debug, Clone)]
pub struct TachyonManifold {
    splitters: HashSet<(usize, usize)>,
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
                (HashSet::new(), None),
                |(mut splitters, mut start_pos), (x, y, char)| {
                    match char {
                        'S' => start_pos = Some((x, y)),
                        '^' => {
                            splitters.insert((x, y));
                        }
                        _ => {}
                    }
                    (splitters, start_pos)
                },
            );

        let depth = splitters.iter().map(|(_, y)| *y).max().unwrap() + 1;

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
                if self.splitters.contains(&(*ray, layer)) {
                    splits += 1;
                    vec![ray - 1, ray + 1]
                } else {
                    vec![*ray]
                }
            })
            .collect();

        (splits, new_rays)
    }

    fn count_splits(&self) -> usize {
        let (mut splits, mut rays) = (0, HashSet::new());
        rays.insert(self.start_position.0);

        for layer in 0..self.depth {
            let (new_splits, new_rays) = self.perform_splits(rays, layer);
            rays = new_rays;
            splits += new_splits;
        }

        splits
    }

    fn perform_splits_overlapping(
        &self,
        rays: HashMap<usize, usize>,
        layer: usize,
    ) -> HashMap<usize, usize> {
        rays.iter()
            .flat_map(|ray| {
                if self.splitters.contains(&(*ray.0, layer)) {
                    vec![(ray.0 - 1, ray.1), (ray.0 + 1, ray.1)]
                } else {
                    vec![(*ray.0, ray.1)]
                }
            })
            .fold(HashMap::new(), |mut acc, (pos, count)| {
                *acc.entry(pos).or_insert(0) += count;
                acc
            })
    }

    fn trace_rays_overlapping(&self) -> HashMap<usize, usize> {
        let mut rays = HashMap::new();
        rays.insert(self.start_position.0, 1);

        for layer in 0..self.depth {
            rays = self.perform_splits_overlapping(rays, layer);
        }

        rays
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
        Answer::Number(data.count_splits() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.trace_rays_overlapping().values().sum::<usize>() as u64)
    }
}
