use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

/// Turns a number into a Vector of individual digits.
fn to_digits(mut number: u64) -> Vec<u8> {
    let mut digits = Vec::with_capacity(16);

    while number > 0 {
        digits.push((number % 10) as u8);
        number /= 10;
    }

    digits.reverse();
    digits
}

/// Checks if a slice is made up of a repetition of a specified length and returns that repetition pattern.
fn detect_repetition_of_length<T: PartialEq>(slice: &[T], sequence_length: usize) -> Option<&[T]> {
    let slice_length = slice.len();

    // if the length of the slice is not a multiple of the sequence length,
    // it cannot be made up of repetitions of this sequence.
    if sequence_length == 0 || slice_length % sequence_length != 0 {
        return None;
    }

    let sequence = &slice[0..sequence_length];
    let mut matches = true;

    for chunk in slice.chunks(sequence_length).skip(1) {
        if chunk != sequence {
            matches = false;
            break;
        }
    }

    if matches { Some(sequence) } else { None }
}

/// Detects all repetitions in a slice making up the complete slice.
///
/// Example:
/// `[1,1,1,1] => [([1], 4), ([1, 1], 2)]`
fn detect_repetition<T: PartialEq>(slice: &[T]) -> Vec<(&[T], usize)> {
    let slice_length = slice.len();

    (1..=(slice_length / 2))
        .filter_map(|sequence_length| {
            detect_repetition_of_length(slice, sequence_length)
                .map(|sequence| (sequence, sequence_length))
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct ProductIdRange {
    start: u64,
    end: u64,
}

impl From<&str> for ProductIdRange {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('-').expect("unexpected input");

        Self {
            start: start.parse().expect("Unexpected input"),
            end: end.parse().expect("Unexpected input"),
        }
    }
}

impl ProductIdRange {
    fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..=self.end
    }
}

#[derive(Debug, Clone)]
pub struct ProductIdRangeList(Vec<ProductIdRange>);

impl From<&str> for ProductIdRangeList {
    fn from(value: &str) -> Self {
        Self(value.split(',').map(|v| v.into()).collect())
    }
}

impl ProductIdRangeList {
    fn sum_twice_invalid_ids(&self) -> u64 {
        self.0
            .iter()
            .flat_map(|range| range.iter())
            .filter(|id| {
                let digits = to_digits(*id);

                if digits.len() % 2 != 0 {
                    return false;
                }

                detect_repetition_of_length(&digits, digits.len() / 2).is_some()
            })
            //.inspect(|id| println!("{}", id))
            .sum()
    }

    fn sum_any_invalid_ids(&self) -> u64 {
        self.0
            .iter()
            .flat_map(|range| range.iter())
            .filter(|id| {
                let digits = to_digits(*id);

                !detect_repetition(&digits).is_empty()
            })
            //.inspect(|id| println!("{}", id))
            .sum()
    }
}

type Data = ProductIdRangeList;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test02.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(1227775554), Answer::Number(4174379265))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.sum_twice_invalid_ids())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.sum_any_invalid_ids())
    }
}
