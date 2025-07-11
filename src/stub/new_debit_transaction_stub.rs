pub mod stub {
    use crate::dto::new_debit_transaction::NewDebitTransaction;
    use crate::stub::client_info_stub::stub::CLIENT_ID;
    use once_cell::sync::Lazy;
    use rust_decimal::Decimal;

    pub static CLIENT_DEBIT_AMOUNT: Lazy<Decimal> = Lazy::new(|| Decimal::new(100, 2));

    /// Create a [NewDebitTransaction] populated with basic stub data
    pub fn create_new_debit_transaction_stub() -> NewDebitTransaction {
        NewDebitTransaction {
            client_id: CLIENT_ID,
            debit_amount: CLIENT_DEBIT_AMOUNT.clone(),
        }
    }
}
