use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transaction Summary struct
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct TransactionSummary {
    // total credits
    pub total_credits: Decimal,
    // total debits
    pub total_debits: Decimal,
    // transaction count
    pub transaction_count: i32,
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    /// Scenario:
    /// Creates a [TransactionSummary] struct with valid values
    /// Expectation:
    /// A [TransactionSummary] with proper values should be created
    #[test]
    fn when_create_transaction_summary_with_proper_values_should_retrieve_set_values() {
        let target = TransactionSummary {
            total_credits: Decimal::new(1000, 0),
            total_debits: Decimal::new(200, 0),
            transaction_count: 2,
        };

        assert_eq!(Decimal::new(1000, 0), target.total_credits);
        assert_eq!(Decimal::new(200, 0), target.total_debits);
        assert_eq!(2, target.transaction_count);
    }
}
