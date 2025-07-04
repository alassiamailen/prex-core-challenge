use rust_decimal::Decimal;
use serde::Deserialize;

/// New Debit Transaction struct
#[derive(Deserialize)]
pub struct NewDebitTransaction {
    // client id
    pub client_id: i32,
    // money debited
    pub debit_amount: Decimal,
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::stub::client_info_stub::stub::CLIENT_ID;  
    use crate::stub::new_debit_transaction_stub::stub::{create_new_debit_transaction_stub, CLIENT_DEBIT_AMOUNT};

    /// Scenario:
    /// Creates a [NewDebitTransaction] struct with valid values
    /// Expectation:
    /// A [NewDebitTransaction] with proper values should be created
    #[test]
    fn when_create_new_debit_transaction_with_proper_values_should_retrieve_set_values(){
        let target= create_new_debit_transaction_stub();

        assert_eq!(CLIENT_ID, target.client_id);
        assert_eq!(CLIENT_DEBIT_AMOUNT.clone(), target.debit_amount);

    }
}
