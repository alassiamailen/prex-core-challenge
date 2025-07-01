use chrono::NaiveDate;
use serde::Deserialize;

/// New Client struct
#[derive(Deserialize)]
pub struct NewClientDto {
    // client name   
    pub client_name: String,
    // client birth date
    pub birth_date:NaiveDate,
    // document number
    pub document_number: String,
    //country
    pub country: String,
}
/// Unit test cases
#[cfg(test)]
mod tests {}