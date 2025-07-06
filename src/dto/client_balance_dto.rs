use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;

/// Client Balance struct
#[derive(Deserialize)]
pub struct ClientBalance {
    // client id
    pub client_id: i32,
    // client name
    pub client_name: String,
    // client birth date
    pub birth_date: NaiveDate,
    // document number
    pub document_number: String,
    //country
    pub country: String,
    // money in account
    pub credit_amount: Decimal,
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::stub::client_info_stub::stub::*;
    use crate::stub::new_client_stub::stub::*;    

    /// Scenario:
    /// Creates a [ClientBalance] struct with valid values
    /// Expectation:
    /// A [ClientBalance] with proper values should be created
    #[test]
    fn when_create_client_balance_with_proper_values_should_retrieve_set_values(){
        let target= create_client_info_stub();
        
        assert_eq!(CLIENT_ID, target.client_id);
        assert_eq!(CLIENT_NAME, target.client_name);
        assert_eq!(CLIENT_BIRTH_DATE.clone(), target.birth_date);
        assert_eq!(CLIENT_DOCUMENT_NUMBER, target.document_number);
        assert_eq!(CLIENT_COUNTRY, target.country);
        assert_eq!(CLIENT_BALANCE.clone(), target.balance);
        
    }
}
