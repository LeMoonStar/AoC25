use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

#[derive(Debug, Clone)]
pub struct TachyonManifold {
    splitters: Vec<Vec<bool>>,
    start_position: (usize, usize),
    depth: usize,
    width: usize,
}

impl From<&str> for TachyonManifold {
    fn from(value: &str) -> Self {
        let mut start_pos = None;
        let mut splitter_positions = Vec::new();
        let mut width = 0;
        let mut depth = 0;

        value.lines().enumerate().for_each(|(y, line)| {
            width = width.max(line.len());
            depth = depth.max(y + 1);
            line.chars().enumerate().for_each(|(x, char)| match char {
                'S' => start_pos = Some((x, y)),
                '^' => splitter_positions.push((x, y)),
                _ => {}
            })
        });

        let mut splitters = vec![vec![false; width]; depth];
        for (x, y) in splitter_positions {
            splitters[y][x] = true;
        }

        Self {
            splitters,
            start_position: start_pos.unwrap(),
            depth,
            width,
        }
    }
}

impl TachyonManifold {
    fn perform_splits(&self, rays: Vec<bool>, layer: usize) -> (usize, Vec<bool>) {
        let mut splits = 0;
        let mut new_rays = vec![false; self.width];

        for (x, &has_ray) in rays.iter().enumerate() {
            if !has_ray {
                continue;
            }

            if self.splitters[layer][x] {
                splits += 1;

                new_rays[x - 1] = true;
                new_rays[x + 1] = true;
            } else {
                new_rays[x] = true;
            }
        }

        (splits, new_rays)
    }

    fn count_splits(&self) -> usize {
        let mut splits = 0;
        let mut rays = vec![false; self.width];

        rays[self.start_position.0] = true;

        for layer in 0..self.depth {
            let (new_splits, new_rays) = self.perform_splits(rays, layer);
            rays = new_rays;
            splits += new_splits;
        }

        splits
    }

    fn perform_splits_overlapping(&self, rays: Vec<usize>, layer: usize) -> Vec<usize> {
        let mut new_rays = vec![0; self.width];
        for (x, &count) in rays.iter().enumerate() {
            if count == 0 {
                continue;
            }

            if self.splitters[layer][x] {
                new_rays[x - 1] += count;
                new_rays[x + 1] += count;
            } else {
                new_rays[x] += count;
            }
        }
        new_rays
    }

    fn count_paths(&self) -> usize {
        let mut rays = vec![0; self.width];
        rays[self.start_position.0] = 1;

        for layer in 0..self.depth {
            rays = self.perform_splits_overlapping(rays, layer);
        }

        rays.iter().sum::<usize>()
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
        Answer::Number(data.count_paths() as u64)
    }
}
