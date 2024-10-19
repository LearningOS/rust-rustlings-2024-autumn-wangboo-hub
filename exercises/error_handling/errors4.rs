#![allow(clippy::comparison_chain)]

#[derive(PartialEq, Debug)]

enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Ok(Self(value as u64))
        // Self(value as u64);
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(Self(value as u64))
        }
    }
}

fn main() {
    // You can optionally experiment here.
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse().unwrap();
    println!("output={:?}", PositiveNonzeroInteger::new(x).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
