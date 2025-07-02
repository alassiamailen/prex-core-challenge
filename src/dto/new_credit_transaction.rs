use rust_decimal::Decimal;
use serde::Deserialize;

/// New Credit Transaction struct
#[derive(Deserialize)]
pub struct NewCreditTransactionDto {
    // client id
    pub client_id: i32,
    // money credited
    pub credit_amount: Decimal,
}

/// Unit test cases
#[cfg(test)]
mod tests {}
