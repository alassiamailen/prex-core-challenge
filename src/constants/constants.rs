/// Main Path
pub const MAIN_PATH: &str = "/api/v1/client";

/// New Client Path
pub const NEW_CLIENT_PATH: &str = "/new_client";
/// New Debit Path
pub const NEW_DEBIT_TRANSACTION_PATH: &str = "/new_debit_transaction";
/// New Credit Path
pub const NEW_CREDIT_TRANSACTION_PATH: &str = "/new_credit_transaction";
/// Store Balance Path
pub const STORE_BALANCE_PATH: &str = "/store_balance";
/// Get Client Balance
pub const CLIENT_BALANCE_PATH: &str = "/client_balance/{id}";

/// Folder for save the client's balances
pub const CLIENT_BALANCE_FOLDER: &str = "./store_balances";

/// Balance file prefix
pub const PREFIX_FILE: &str = ".DAT";

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::constants::constants::*;
    
    /// Scenario:
    /// Creates constant with valid values
    #[test]
    fn test_constants() {
        assert_eq!("/api/v1/client", MAIN_PATH);
        assert_eq!("/new_client", NEW_CLIENT_PATH);
        assert_eq!("/new_debit_transaction", NEW_DEBIT_TRANSACTION_PATH);
        assert_eq!("/new_credit_transaction",NEW_CREDIT_TRANSACTION_PATH);
        assert_eq!("/store_balance",STORE_BALANCE_PATH);
        assert_eq!("/client_balance/{id}",CLIENT_BALANCE_PATH);
        assert_eq!("./store_balances",CLIENT_BALANCE_FOLDER);
        assert_eq!(".DAT",PREFIX_FILE);
    }
    
}
