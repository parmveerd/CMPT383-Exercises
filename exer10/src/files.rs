use std::fs;
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SummationError {
    msg: String,
}

impl std::error::Error for SummationError {}

impl fmt::Display for SummationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<std::io::Error> for SummationError {
    fn from(e: std::io::Error) -> SummationError {
        SummationError {
            msg: format!("io::Error: {}", e),
        }
    }
}

impl From<std::num::ParseIntError> for SummationError {
    fn from(e: std::num::ParseIntError) -> SummationError {
        SummationError {
            msg: format!("ParseIntError: {}", e),
        }
    }
}

pub fn sum_file_1(path: &Path) -> Result<i64, SummationError> {
    let contents = fs::read_to_string(path)?;

    let sum = contents
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?
        .iter()
        .sum();

    Ok(sum)
}

pub fn sum_file_2(path: &Path) -> Result<i64, SummationError> {
    let contents = fs::read_to_string(path)?;

    let sum = contents
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?
        .iter()
        .sum();

    Ok(sum)
}
