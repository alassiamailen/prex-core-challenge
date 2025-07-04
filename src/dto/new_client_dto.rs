use chrono::NaiveDate;
use serde::Deserialize;

/// New Client struct
#[derive(Deserialize)]
pub struct NewClient{
    // client name
    pub client_name: String,
    // client birth date
    pub birth_date: NaiveDate,
    // document number
    pub document_number: String,
    //country
    pub country: String,
}
/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::stub::new_client_stub::stub::*;

    /// Scenario:
    /// Creates a [NewClient] struct with valid values
    /// Expectation:
    /// A [NewClient] with proper values should be created
    #[test]
    fn when_create_new_client_with_proper_values_should_retrieve_set_values(){
        let target= create_new_client_stub();
        
        assert_eq!(CLIENT_NAME, target.client_name);
        assert_eq!(CLIENT_BIRTH_DATE.clone(), target.birth_date);
        assert_eq!(CLIENT_DOCUMENT_NUMBER, target.document_number);
        assert_eq!(CLIENT_COUNTRY, target.country);
    }
}
