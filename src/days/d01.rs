use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left(i16),
    Right(i16),
}

impl Rotation {
    fn as_signed_int(&self) -> i16 {
        match *self {
            Self::Left(value) => value,
            Self::Right(value) => -value,
        }
    }
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let (direction, amount) = value.split_at(1);
        match direction {
            "R" => Rotation::Right(amount.parse().unwrap()),
            "L" => Rotation::Left(amount.parse().unwrap()),
            _ => panic!("Unexpected input."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RotationSequence(Vec<Rotation>);

impl From<&str> for RotationSequence {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|v| v.into()).collect())
    }
}

/**
 * Wraps a number.
 *
 * number=245, min=0, max=99 => 45
 */
fn wrap_number(number: i16, min: i16, max: i16) -> i16 {
    let range = max - min + 1;
    let mut result = (number - min) % range;
    if result < 0 {
        result += range;
    }
    result + min
}

impl RotationSequence {
    fn get_direction_sequence(&self) -> Vec<u8> {
        let mut directions = Vec::with_capacity(self.0.len());

        let mut current: i16 = 50;
        for rotation in &self.0 {
            current = wrap_number(current + rotation.as_signed_int(), 0, 99);

            directions.push(current as u8);
        }

        directions
    }
}

type Data = RotationSequence;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test01.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let sequence = data.get_direction_sequence();

        Answer::Number(sequence.into_iter().filter(|v| *v == 0).count() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
