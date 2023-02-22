use std::{env, process, str::FromStr};

#[derive(Debug)]
enum ParseMoneyError {
    InvalidInput,
    ParseFailed,
}

struct Money {
    amount: f32,
    currency: String,
}

impl Money {
    fn new(amount: f32, currency: &str) -> Self {
        Self {
            amount,
            currency: currency.to_string(),
        }
    }
}

// Implements the `FromStr` trait for our `Money` struct so that the compiler will be able to resolve the `parse()`
// function on any `str` value.
impl FromStr for Money {
    // Declare a local type representing the error that will be returned.
    type Err = ParseMoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split_whitespace().collect();

        match segments[..] {
            [amount, currency] => amount
                .parse::<f32>()
                .map(|amount| Money::new(amount, currency))
                .map_err(|_| ParseMoneyError::ParseFailed),

            _ => Err(ParseMoneyError::InvalidInput),
        }
    }
}

fn main() {
    let Some(user_input) = env::args().nth(1) else {
        eprintln!("Usage: money_parser.exe <value_to_parse>");
        return;
    };

    match user_input.parse::<Money>() {
        Ok(money) => println!("Amount: {}, currency: {}", money.amount, money.currency),
        Err(error) => {
            eprintln!("Parsing error: {:?}", error);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_from_str_should_return_invalid_input_const_for_empty_string() {
        let result = Money::from_str("");

        assert!(result.is_err());
    }

    #[test]
    fn money_from_str_should_return_invalid_input_const_for_string_without_currency() {
        let result = Money::from_str("123");

        assert!(result.is_err());
    }

    #[test]
    fn money_from_str_should_return_invalid_input_const_for_string_without_amount() {
        let result = Money::from_str("eur");

        assert!(result.is_err());
    }

    #[test]
    fn money_from_str_should_return_integer_amount_and_currency() {
        let money = Money::from_str("123 €").unwrap();

        assert_eq!(money.amount, 123.0);
        assert_eq!(money.currency, "€");
    }

    #[test]
    fn money_from_str_should_return_float_amount_and_currency() {
        let money = Money::from_str("123.45 €").unwrap();

        assert_eq!(money.amount, 123.45);
        assert_eq!(money.currency, "€");
    }

    #[test]
    fn money_from_str_should_return_negative_integer_amount_and_currency() {
        let money = Money::from_str("-123 €").unwrap();

        assert_eq!(money.amount, -123.0);
        assert_eq!(money.currency, "€");
    }

    #[test]
    fn money_from_str_should_return_negative_float_amount_and_currency() {
        let money = Money::from_str("-123.45 €").unwrap();

        assert_eq!(money.amount, -123.45);
        assert_eq!(money.currency, "€");
    }
}
