pub mod stub {
    use crate::dto::new_credit_transaction::NewCreditTransaction;
    use crate::stub::client_info_stub::stub::CLIENT_ID;
    use once_cell::sync::Lazy;
    use rust_decimal::Decimal;

    pub static CLIENT_CREDIT_AMOUNT: Lazy<Decimal> = Lazy::new(|| Decimal::new(100, 2));

    /// Create a [NewCreditTransaction] populated with basic stub data
    pub fn create_new_credit_transaction_stub() -> NewCreditTransaction {
        NewCreditTransaction {
            client_id: CLIENT_ID,
            credit_amount: CLIENT_CREDIT_AMOUNT.clone(),
        }
    }
}
