use crate::dto::transaction_summary_dto::TransactionSummary;
use crate::errors::common_error::CommonError;
use crate::model::transaction_model::{Transaction, TransactionType};
use crate::model::user_model::User;
use crate::service::authorization_service::DynAuthorizationService;
use crate::state::app_state::AppState;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use log::{debug, error, info};
use mockall::automock;
use rust_decimal::Decimal;
use std::sync::Arc;

/// Transaction service
#[async_trait]
#[cfg_attr(test, automock)]
pub trait TransactionServiceTrait {
    /// Get all transactions from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transactions(
        &self,
        user: &User,
        client_id: i32,
    ) -> Result<Vec<Transaction>, CommonError>;

    /// Get transactions by type from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transactions_by_type(
        &self,
        user: &User,
        client_id: i32,
        transaction_type: TransactionType,
    ) -> Result<Vec<Transaction>, CommonError>;

    /// Get transactions by period from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transactions_by_period(
        &self,
        user: &User,
        client_id: i32,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> Result<Vec<Transaction>, CommonError>;

    /// Get transaction summary from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transaction_summary(
        &self,
        user: &User,
        client_id: i32,
    ) -> Result<TransactionSummary, CommonError>;
}

/// Transaction service implementation struct
pub struct TransactionService {
    pub app_state: Arc<AppState>,
    pub authorization_service: DynAuthorizationService,
}

/// Initialization
impl TransactionService {
    pub fn new(app_state: Arc<AppState>, authorization_service: DynAuthorizationService) -> Self {
        TransactionService {
            app_state,
            authorization_service,
        }
    }
}
/// Transaction service implement logic
#[async_trait]
impl TransactionServiceTrait for TransactionService {
    /// Get all transactions from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transactions(
        &self,
        user: &User,
        client_id: i32,
    ) -> Result<Vec<Transaction>, CommonError> {
        info!("get_transactions - start");
        match self
            .authorization_service
            .authorize_client_access(user, client_id)
            .await
        {
            Ok(()) => match self.app_state.data.read() {
                Ok(app_data) => {
                    let transactions = app_data
                        .transactions
                        .values()
                        .filter(|transaction| transaction.client_id == client_id)
                        .cloned()
                        .collect::<Vec<Transaction>>();

                    debug!("get_transactions - done");
                    Ok(transactions)
                }
                Err(_) => {
                    error!(
                        "get_transactions - error - has occurred an error while try read in app_state"
                    );
                    Err(CommonError::LockReadFailed)
                }
            },
            Err(error) => {
                error!(
                    "get_transactions - authorization_service - error: {:?}",
                    error
                );
                Err(error)
            }
        }
    }

    /// Get transactions by type from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transactions_by_type(
        &self,
        user: &User,
        client_id: i32,
        transaction_type: TransactionType,
    ) -> Result<Vec<Transaction>, CommonError> {
        info!("get_transactions_by_type - start");

        match self
            .authorization_service
            .authorize_client_access(user, client_id)
            .await
        {
            Ok(()) => match self.app_state.data.read() {
                Ok(app_data) => {
                    let transactions = app_data
                        .transactions
                        .values()
                        .filter(|transaction| {
                            transaction.client_id == client_id
                                && transaction.transaction_type == transaction_type
                        })
                        .cloned()
                        .collect::<Vec<Transaction>>();

                    debug!("get_transactions_by_type - done");
                    Ok(transactions)
                }
                Err(_) => {
                    error!(
                    "get_transactions_by_type - error - has occurred an error while try read in app_state"
                );
                    Err(CommonError::LockReadFailed)
                }
            },
            Err(error) => {
                error!(
                    "get_transactions_by_type - authorization_service - error: {:?}",
                    error
                );
                Err(error)
            }
        }
    }

    /// Get transactions by period from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transactions_by_period(
        &self,
        user: &User,
        client_id: i32,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> Result<Vec<Transaction>, CommonError> {
        info!("get_transactions_by_period - start");

        match self
            .authorization_service
            .authorize_client_access(user, client_id)
            .await
        {
            Ok(()) => match self.app_state.data.read() {
                Ok(app_data) => {
                    let mut transactions = app_data
                        .transactions
                        .values()
                        .filter(|transaction| {
                            transaction.client_id == client_id
                                && transaction.timestamp >= from
                                && transaction.timestamp <= to
                        })
                        .cloned()
                        .collect::<Vec<Transaction>>();

                    transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                    debug!("get_transactions_by_period - done");
                    Ok(transactions)
                }
                Err(_) => {
                    error!(
                    "get_transactions_by_period - error - has occurred an error while try read in app_state"
                );
                    Err(CommonError::LockReadFailed)
                }
            },
            Err(error) => {
                error!(
                    "get_transactions_by_period - authorization_service - error: {:?}",
                    error
                );
                Err(error)
            }
        }
    }

    /// Get transaction summary from [i32] client_id
    /// Returns a [CommonError] if the user is not authorized or service throws any error
    async fn get_transaction_summary(
        &self,
        user: &User,
        client_id: i32,
    ) -> Result<TransactionSummary, CommonError> {
        info!("get_transaction_summary - start");

        match self
            .authorization_service
            .authorize_client_access(user, client_id)
            .await
        {
            Ok(()) => match self.app_state.data.read() {
                Ok(app_data) => {
                    let transactions = app_data
                        .transactions
                        .values()
                        .filter(|transaction| transaction.client_id == client_id);

                    let mut total_credits = Decimal::ZERO;
                    let mut total_debits = Decimal::ZERO;
                    let mut transaction_count = 0;

                    for transaction in transactions {
                        transaction_count += 1;

                        match transaction.transaction_type {
                            TransactionType::Credit => {
                                total_credits += transaction.amount;
                            }
                            TransactionType::Debit => {
                                total_debits += transaction.amount;
                            }
                        }
                    }

                    let transaction_summary = TransactionSummary {
                        total_credits,
                        total_debits,
                        transaction_count,
                    };

                    debug!("get_transaction_summary - done");
                    Ok(transaction_summary)
                }
                Err(_) => {
                    error!(
                    "get_transaction_summary - error - has occurred an error while try read in app_state"
                );
                    Err(CommonError::LockReadFailed)
                }
            },
            Err(error) => {
                error!(
                    "get_transaction_summary - authorization_service - error: {:?}",
                    error
                );
                Err(error)
            }
        }
    }
}

/// Transaction service trait dyn type
pub type DynTransactionService = Arc<dyn TransactionServiceTrait + Send + Sync>;

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::errors::common_error::CommonError;
    use crate::model::transaction_model::{Transaction, TransactionType};
    use crate::model::user_model::{Role, User};
    use crate::service::authorization_service::AuthorizationService;
    use crate::service::transaction_service::{TransactionService, TransactionServiceTrait};
    use crate::state::app_state::{AppData, AppState};
    use chrono::Local;
    use rust_decimal::Decimal;
    use serial_test::serial;
    use std::collections::HashMap;
    use std::sync::atomic::AtomicI32;
    use std::sync::{Arc, RwLock};
    const MOCK_CLIENT_ID: i32 = 1;
    const MOCK_USER_ID: i32 = 1;

    /// Scenario:
    /// Execute get_transactions when [User] is authorized and client has transactions
    /// Expectation:
    /// A list of client transactions should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_with_authorized_user_should_return_transactions() {
        let client_id = MOCK_USER_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1000, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(transaction.transaction_id, transaction.clone());

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service.get_transactions(&user, client_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![transaction]);
    }

    /// Scenario:
    /// Execute get_transactions when [User] is not authorized to access client
    /// Expectation:
    /// A [CommonError::Forbiden] should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_with_unauthorized_user_should_return_forbiden() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::User,
        };

        let client_id = 2;

        let app_data = AppData {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service.get_transactions(&user, client_id).await;

        assert_eq!(result, Err(CommonError::Forbiden));
    }

    /// Scenario:
    /// Execute get_transactions when [User] has [Role::SuperAdmin]
    /// Expectation:
    /// A list of client transactions should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_with_super_admin_should_return_transactions() {
        let user = User {
            user_id: 1,
            client_id: 1,
            role: Role::SuperAdmin,
        };

        let client_id = 2;

        let transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1000, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(transaction.transaction_id, transaction.clone());

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service.get_transactions(&user, client_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![transaction]);
    }

    /// Scenario:
    /// Execute get_transactions when client has no transactions
    /// Expectation:
    /// An empty list of transactions should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_without_transactions_should_return_empty_list() {
        let client_id = 1;

        let user = User {
            user_id: 1,
            client_id,
            role: Role::User,
        };

        let app_data = AppData {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service.get_transactions(&user, client_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    /// Scenario:
    /// Execute get_transactions_by_type when [User] is authorized and client has transactions
    /// Expectation:
    /// A list with client credit transactions should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_type_with_authorized_user_should_return_credit_transactions()
    {
        let client_id = MOCK_CLIENT_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let credit_transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1000, 0),
        };

        let debit_transaction = Transaction {
            transaction_id: 2,
            client_id,
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(200, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(800, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(
            credit_transaction.transaction_id,
            credit_transaction.clone(),
        );
        transactions.insert(debit_transaction.transaction_id, debit_transaction);

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_type(&user, client_id, TransactionType::Credit)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![credit_transaction]);
    }

    /// Scenario:
    /// Execute get_transactions_by_type when [User] is authorized and client has transactions
    /// Expectation:
    /// A list with client debit transactions should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_type_with_authorized_user_should_return_debit_transactions() {
        let client_id = MOCK_CLIENT_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let credit_transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1000, 0),
        };

        let debit_transaction = Transaction {
            transaction_id: 2,
            client_id,
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(200, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(800, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(credit_transaction.transaction_id, credit_transaction);
        transactions.insert(debit_transaction.transaction_id, debit_transaction.clone());

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_type(&user, client_id, TransactionType::Debit)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![debit_transaction]);
    }
    /// Scenario:
    /// Execute get_transactions_by_type when [User] is not authorized to access client
    /// Expectation:
    /// A [CommonError::Forbiden] should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_type_with_unauthorized_user_should_return_forbiden() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::User,
        };

        let client_id = MOCK_CLIENT_ID + 1;

        let app_data = AppData {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_type(&user, client_id, TransactionType::Credit)
            .await;

        assert_eq!(result, Err(CommonError::Forbiden));
    }
    /// Scenario:
    /// Execute get_transactions_by_type when [User] has [Role::SuperAdmin]
    /// Expectation:
    /// A list with client transactions of the requested type should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_type_with_super_admin_should_return_transactions() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::SuperAdmin,
        };

        let client_id = MOCK_CLIENT_ID + 1;

        let credit_transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1000, 0),
        };

        let debit_transaction = Transaction {
            transaction_id: 2,
            client_id,
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(200, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(800, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(
            credit_transaction.transaction_id,
            credit_transaction.clone(),
        );
        transactions.insert(debit_transaction.transaction_id, debit_transaction);

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_type(&user, client_id, TransactionType::Credit)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![credit_transaction]);
    }

    /// Scenario:
    /// Execute get_transactions_by_period when [User] is authorized and client has transactions
    /// Expectation:
    /// Transactions within the requested period should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_period_with_authorized_user_should_return_transactions() {
        let client_id = MOCK_CLIENT_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let from = Local::now() - chrono::Duration::days(10);
        let to = Local::now();

        let transaction_in_period = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now() - chrono::Duration::days(5),
            balance_after: Decimal::new(1000, 0),
        };

        let transaction_outside_period = Transaction {
            transaction_id: 2,
            client_id,
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(200, 0),
            timestamp: Local::now() - chrono::Duration::days(20),
            balance_after: Decimal::new(800, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(
            transaction_in_period.transaction_id,
            transaction_in_period.clone(),
        );
        transactions.insert(
            transaction_outside_period.transaction_id,
            transaction_outside_period,
        );

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_period(&user, client_id, from, to)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![transaction_in_period]);
    }

    /// Scenario:
    /// Execute get_transactions_by_period when client has no transactions within the requested period
    /// Expectation:
    /// An empty list of transactions should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_period_without_transactions_should_return_empty_list() {
        let client_id = MOCK_CLIENT_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let from = Local::now() - chrono::Duration::days(10);
        let to = Local::now();

        let transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now() - chrono::Duration::days(20),
            balance_after: Decimal::new(1000, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(transaction.transaction_id, transaction);

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_period(&user, client_id, from, to)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    /// Scenario:
    /// Execute get_transactions_by_period when [User] is not authorized to access client
    /// Expectation:
    /// A [CommonError::Forbiden] should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_period_with_unauthorized_user_should_return_forbiden() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::User,
        };

        let client_id = MOCK_CLIENT_ID + 1;

        let from = Local::now() - chrono::Duration::days(10);
        let to = Local::now();

        let app_data = AppData {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_period(&user, client_id, from, to)
            .await;

        assert_eq!(result, Err(CommonError::Forbiden));
    }
    /// Scenario:
    /// Execute get_transactions_by_period when [User] has [Role::SuperAdmin]
    /// Expectation:
    /// Transactions within the requested period should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transactions_by_period_with_super_admin_should_return_transactions() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::SuperAdmin,
        };

        let client_id = MOCK_CLIENT_ID + 1;

        let from = Local::now() - chrono::Duration::days(10);
        let to = Local::now();

        let transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now() - chrono::Duration::days(5),
            balance_after: Decimal::new(1000, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(transaction.transaction_id, transaction.clone());

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transactions_by_period(&user, client_id, from, to)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![transaction]);
    }

    /// Scenario:
    /// Execute get_transaction_summary when [User] is authorized and client has transactions
    /// Expectation:
    /// A [TransactionSummary] with total credits, total debits and transaction count should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transaction_summary_with_authorized_user_should_return_summary() {
        let client_id = MOCK_CLIENT_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let credit_transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(1000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1000, 0),
        };

        let second_credit_transaction = Transaction {
            transaction_id: 2,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(3000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(4000, 0),
        };

        let debit_transaction = Transaction {
            transaction_id: 3,
            client_id,
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(500, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(3500, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(credit_transaction.transaction_id, credit_transaction);
        transactions.insert(
            second_credit_transaction.transaction_id,
            second_credit_transaction,
        );
        transactions.insert(debit_transaction.transaction_id, debit_transaction);

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transaction_summary(&user, client_id)
            .await;

        assert!(result.is_ok());

        let summary = result.unwrap();

        assert_eq!(Decimal::new(4000, 0), summary.total_credits);
        assert_eq!(Decimal::new(500, 0), summary.total_debits);
        assert_eq!(3, summary.transaction_count);
    }

    /// Scenario:
    /// Execute get_transaction_summary when client has no transactions
    /// Expectation:
    /// A [TransactionSummary] with zero values should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transaction_summary_without_transactions_should_return_empty_summary() {
        let client_id = MOCK_CLIENT_ID;

        let user = User {
            user_id: MOCK_USER_ID,
            client_id,
            role: Role::User,
        };

        let app_data = AppData {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transaction_summary(&user, client_id)
            .await;

        assert!(result.is_ok());

        let summary = result.unwrap();

        assert_eq!(Decimal::ZERO, summary.total_credits);
        assert_eq!(Decimal::ZERO, summary.total_debits);
        assert_eq!(0, summary.transaction_count);
    }

    /// Scenario:
    /// Execute get_transaction_summary when [User] is not authorized to access client
    /// Expectation:
    /// A [CommonError::Forbiden] should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transaction_summary_with_unauthorized_user_should_return_forbiden() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::User,
        };

        let client_id = MOCK_CLIENT_ID + 1;

        let app_data = AppData {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transaction_summary(&user, client_id)
            .await;

        assert_eq!(result, Err(CommonError::Forbiden));
    }

    /// Scenario:
    /// Execute get_transaction_summary when [User] has [Role::SuperAdmin]
    /// Expectation:
    /// A [TransactionSummary] with client transaction values should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_transaction_summary_with_super_admin_should_return_summary() {
        let user = User {
            user_id: MOCK_USER_ID,
            client_id: MOCK_CLIENT_ID,
            role: Role::SuperAdmin,
        };

        let client_id = MOCK_CLIENT_ID + 1;

        let credit_transaction = Transaction {
            transaction_id: 1,
            client_id,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(2000, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(2000, 0),
        };

        let debit_transaction = Transaction {
            transaction_id: 2,
            client_id,
            transaction_type: TransactionType::Debit,
            amount: Decimal::new(500, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(1500, 0),
        };

        let mut transactions = HashMap::new();
        transactions.insert(credit_transaction.transaction_id, credit_transaction);
        transactions.insert(debit_transaction.transaction_id, debit_transaction);

        let app_data = AppData {
            clients: HashMap::new(),
            transactions,
        };

        let app_state = Arc::new(AppState {
            data: Arc::new(RwLock::new(app_data)),
            client_id_unique: AtomicI32::new(1),
            transaction_id_unique: AtomicI32::new(1),
        });

        let authorization_service = Arc::new(AuthorizationService::new());

        let transaction_service = TransactionService::new(app_state, authorization_service);

        let result = transaction_service
            .get_transaction_summary(&user, client_id)
            .await;

        assert!(result.is_ok());

        let summary = result.unwrap();

        assert_eq!(Decimal::new(2000, 0), summary.total_credits);
        assert_eq!(Decimal::new(500, 0), summary.total_debits);
        assert_eq!(2, summary.transaction_count);
    }
}
