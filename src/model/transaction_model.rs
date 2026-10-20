use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transaction type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Credit,
    Debit,
}

/// Transaction model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    // transaction id
    pub transaction_id: i32,
    // client id associated with the transaction
    pub client_id: i32,
    // transaction type
    pub transaction_type: TransactionType,
    // transaction amount
    pub amount: Decimal,
    // transaction timestamp
    pub timestamp: DateTime<Local>,
    // client balance after the transaction
    pub balance_after: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use rust_decimal::Decimal;

    const TRANSACTION_ID: i32 = i32::MAX;
    const CLIENT_ID: i32 = i32::MAX - 1;

    /// Get Credit Transaction model
    #[tokio::test]
    async fn test_credit_transaction_model() {
        let transaction_type = TransactionType::Credit;
        let amount = Decimal::new(1000, 2);
        let balance_after = Decimal::new(2000, 2);
        let timestamp = Local::now();

        let expected_result = Transaction {
            transaction_id: TRANSACTION_ID,
            client_id: CLIENT_ID,
            transaction_type: transaction_type.clone(),
            amount,
            timestamp,
            balance_after,
        };

        assert_eq!(TRANSACTION_ID, expected_result.transaction_id);
        assert_eq!(CLIENT_ID, expected_result.client_id);
        assert_eq!(transaction_type, expected_result.transaction_type);
        assert_eq!(amount, expected_result.amount);
        assert_eq!(timestamp, expected_result.timestamp);
        assert_eq!(balance_after, expected_result.balance_after);
    }

    /// Get Debit Transaction model
    #[tokio::test]
    async fn test_debit_transaction_type() {
        let transaction_type = TransactionType::Debit;

        assert_eq!(TransactionType::Debit, transaction_type);
    }

    /// Get Credit Transaction model
    #[tokio::test]
    async fn test_credit_transaction_type() {
        let transaction_type = TransactionType::Credit;

        assert_eq!(TransactionType::Credit, transaction_type);
    }
}
