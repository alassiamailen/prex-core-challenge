use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;

/// Client Balance struct
#[derive(Deserialize)]
pub struct ClientBalanceDto {
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

/// Unit test cases
#[cfg(test)]
mod tests {}
