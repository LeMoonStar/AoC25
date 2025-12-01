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
    fn get_full_rotation_zero_count(&self) -> usize {
        let mut current: i16 = 50;
        let mut count = 0;

        for rotation in &self.0 {
            current = wrap_number(current + rotation.as_signed_int(), 0, 99);

            if current == 0 {
                count += 1;
            }
        }

        count
    }

    fn get_partial_rotation_zero_count(&self) -> usize {
        let mut current: i16 = 50;
        let mut count = 0;

        for rotation in &self.0 {
            let mut current_rotation = *rotation;

            loop {
                match &mut current_rotation {
                    Rotation::Left(0) => break,
                    Rotation::Right(0) => break,
                    Rotation::Left(value) => {
                        current += 1;
                        *value -= 1;
                    }
                    Rotation::Right(value) => {
                        current -= 1;
                        *value -= 1;
                    }
                }

                current = wrap_number(current, 0, 99);

                if current == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}

type Data = RotationSequence;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test01.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(6))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_full_rotation_zero_count() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_partial_rotation_zero_count() as u64)
    }
}
