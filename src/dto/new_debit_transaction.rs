use rust_decimal::Decimal;
use serde::Deserialize;

/// New Debit Transaction struct
#[derive(Deserialize)]
pub struct NewDebitTransactionDto {
    // client id
    pub client_id: i32,
    // money debited
    pub debit_amount: Decimal,
}

/// Unit test cases
#[cfg(test)]
mod tests {}
