pub mod stub {
    use crate::dto::new_client_dto::NewClient;
    use chrono::NaiveDate;
    use once_cell::sync::Lazy;

    pub const CLIENT_NAME: &str = "CLIENT_NAME_STUB";
    pub const CLIENT_DOCUMENT_NUMBER: &str = "CLIENT_DOCUMENT_NUMBER_STUB";
    pub const CLIENT_COUNTRY: &str = "CLIENT_COUNTRY_STUB";
    pub static CLIENT_BIRTH_DATE: Lazy<NaiveDate> =
        Lazy::new(|| NaiveDate::parse_from_str("03-07-2025", "%d-%m-%Y").unwrap());

    /// Create a [NewClient] populated with basic stub data
    pub fn create_new_client_stub() -> NewClient {
        NewClient {
            client_name: String::from(CLIENT_NAME),
            birth_date: CLIENT_BIRTH_DATE.clone(),
            document_number: String::from(CLIENT_DOCUMENT_NUMBER),
            country: String::from(CLIENT_COUNTRY),
        }
    }
}
