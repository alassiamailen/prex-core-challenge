pub mod stub {
    use crate::dto::client_info_dto::ClientInfo;
    use crate::stub::new_client_stub::stub::*;
    use once_cell::sync::Lazy;
    use rust_decimal::Decimal;

    pub static CLIENT_BALANCE: Lazy<Decimal> = Lazy::new(|| Decimal::new(100, 2));
    pub const CLIENT_ID: i32 = 1;

    /// Create a [ClientInfo] populated with basic stub data
    pub fn create_client_info_stub() -> ClientInfo {
        ClientInfo {
            client_id: CLIENT_ID,
            client_name: CLIENT_NAME.to_string(),
            birth_date: CLIENT_BIRTH_DATE.clone(),
            document_number: CLIENT_DOCUMENT_NUMBER.to_string(),
            country: CLIENT_COUNTRY.to_string(),
            balance: CLIENT_BALANCE.clone(),
        }
    }
}
