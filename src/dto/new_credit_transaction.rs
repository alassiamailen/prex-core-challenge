use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// New Credit Transaction struct
#[derive(Deserialize, Serialize)]
pub struct NewCreditTransaction {
    // client id
    pub client_id: i32,
    // money credited
    pub credit_amount: Decimal,
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::stub::new_credit_transaction_stub::stub::{create_new_credit_transaction_stub, CLIENT_CREDIT_AMOUNT};
    use crate::stub::client_info_stub::stub::*;

    /// Scenario:
    /// Creates a [NewCreditTransaction] struct with valid values
    /// Expectation:
    /// A [NewCreditTransaction] with proper values should be created
    #[test]
    fn when_create_new_credit_transaction_with_proper_values_should_retrieve_set_values(){
        let target= create_new_credit_transaction_stub();
        
        assert_eq!(CLIENT_ID, target.client_id);
        assert_eq!(CLIENT_CREDIT_AMOUNT.clone(), target.credit_amount);
         
    }
}
