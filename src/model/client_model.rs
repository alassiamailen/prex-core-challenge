use chrono::NaiveDate;
use rust_decimal::Decimal;

/// Client model
#[derive(Debug, Clone)]
pub struct Client {
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
    pub balance: Decimal,
}
/// Unit test cases
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    const CLIENT_ID: i32 = i32::MAX;
    const CLIENT_NAME: &str = "some-name-value";
    const DOCUMENT_NUMBER: &str = "some-document-number-value";
    const COUNTRY: &str = "some-country-value";

    /// Get Client model
    #[tokio::test]
    async fn test_client_model() {
        let balance = Decimal::new(10, 2);
        let birth_date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();

        let expected_result = Client {
            client_id: CLIENT_ID,
            client_name: CLIENT_NAME.to_string(),
            birth_date,
            document_number: DOCUMENT_NUMBER.to_string(),
            country: COUNTRY.to_string(),
            balance,
        };

        assert_eq!(CLIENT_ID, expected_result.client_id);
        assert_eq!(balance, expected_result.balance);
        assert_eq!(birth_date, expected_result.birth_date);
        assert_eq!(COUNTRY, expected_result.country);
        assert_eq!(DOCUMENT_NUMBER, expected_result.document_number);
        assert_eq!(CLIENT_NAME, expected_result.client_name);
    }
}
