use crate::dto::new_client_dto::NewClientDto;
use crate::dto::new_credit_transaction::NewCreditTransactionDto;
use crate::dto::new_debit_transaction::NewDebitTransactionDto;
use crate::errors::common_error::CommonError;
use crate::mapper::new_client_mapper::client_map;
use crate::model::client_model::Client;
use crate::state::app_state::AppState;
use async_trait::async_trait;
use log::warn;
use log::{debug, error, info};
use rust_decimal::Decimal;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Client service
#[async_trait]
pub trait ClientServiceTrait {
    /// Create new Client from [NewClientDto] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client: NewClientDto) -> Result<i32, CommonError>;

    /// Create a new credit transaction from [NewCreditTransactionDto] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_credit_transaction(
        &self,
        credit_transaction: NewCreditTransactionDto,
    ) -> Result<Decimal, CommonError>;
    /// Create a new debit transaction from [NewDebitTransactionDto] debit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_debit_transaction(
        &self,
        debit_transaction: NewDebitTransactionDto,
    ) -> Result<Decimal, CommonError>;
}

/// Client service implementation struct
pub struct ClientService {
    app_state: Arc<AppState>,
}

/// Initialization
impl ClientService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        ClientService { app_state }
    }
}

/// Client service implement logic
#[async_trait]
impl ClientServiceTrait for ClientService {
    /// Create new Client from [NewClientDto] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client: NewClientDto) -> Result<i32, CommonError> {
        info!("create_new_client - start");
        let document_number = &new_client.document_number;

        // verify client document exists
        match self.validate_client_document(document_number) {
            Ok(_exist) => {
                // Generate unique id for each client
                let client_id = self.generate_client_id();

                // map Client from NewClientDto
                let populate_new_client = client_map(new_client, client_id);

                // Insert Client and id in app_state
                match self.app_state.clients.write() {
                    Ok(mut app_state) => {
                        app_state.insert(client_id, populate_new_client);
                        info!("create_new_client - done");
                        Ok(client_id)
                    }
                    Err(_) => {
                        error!("create_new_client - error - has occurred an error while try write in app_state");
                        Err(CommonError::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Err(error) => {
                error!("create_new_client - error - error: {:?}", error);
                error!("create_new_client - error - document_number: {document_number}");
                Err(error)
            }
        }
    }
    /// Create a new transaction from [NewCreditTransactionDto] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_credit_transaction(
        &self,
        transaction: NewCreditTransactionDto,
    ) -> Result<Decimal, CommonError> {
        info!("create_new_credit_transaction - start");
        let client_id = transaction.client_id;

        // validate if client id exists
        match self.validate_client_id(client_id) {
            Ok(client_id) => {
                // update client balance
                match self.new_credit_on_client_account(client_id, transaction.credit_amount) {
                    Ok(balance) => {
                        info!("create_new_credit_transaction - done");
                        Ok(balance)
                    }
                    Err(error) => {
                        error!("create_new_credit_transaction - error: {:?}", error);
                        Err(error)
                    }
                }
            }
            Err(error) => {
                error!("create_new_credit_transaction - error: {:?}", error);
                Err(error)
            }
        }
    }
    /// Create a new debit transaction from [NewCreditTransactionDto] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_debit_transaction(
        &self,
        transaction: NewDebitTransactionDto,
    ) -> Result<Decimal, CommonError> {
        info!("create_new_debit_transaction - start");

        let client_id = transaction.client_id;

        // validate if client id exists
        match self.validate_client_id(client_id) {
            Ok(client_id) => {
                // update client balance
                match self.new_debit_on_client_account(client_id, transaction.debit_amount) {
                    Ok(balance) => {
                        info!("create_new_debit_transaction - done");
                        Ok(balance)
                    }
                    Err(error) => {
                        error!("create_new_debit_transaction - error: {:?}", error);
                        Err(error)
                    }
                }
            }
            Err(error) => {
                error!("create_new_credit_transaction - error: {:?}", error);
                Err(error)
            }
        }
    }
}
/// Client service "private" implement logic
impl ClientService {
    /// Validate if the client document number exists based on [String] document_number
    fn validate_client_document(&self, document_number: &str) -> Result<bool, CommonError> {
        debug!("validate_client_document - start");

        match self.app_state.clients.read() {
            Ok(clients_map) => {
                // get a document if exists
                let validate_document = clients_map
                    .values()
                    .any(|client| client.document_number == document_number);

                if !validate_document {
                    debug!("validate_client_document - document unique - done");
                    Ok(true)
                }else{
                    error!("validate_client_id - error - the document must be unique"
                    );
                    Err(CommonError::FORBIDEN)
                } 
            }
            Err(_) => {
                error!(
                    "validate_client_id - error -has occurred an error while try read in app_state"
                );
                Err(CommonError::INTERNAL_SERVER_ERROR)
            }
        }
    }
    /// Generate a client id unique
    fn generate_client_id(&self) -> i32 {
        self.app_state
            .client_id_unique
            .fetch_add(1, Ordering::SeqCst)
    }

    /// Validate if client id exists based on [Decimal] client_id
    fn validate_client_id(&self, client_id: i32) -> Result<i32, CommonError> {
        debug!("validate_client_id - start");

        match self.app_state.clients.read() {
            Ok(clients_map) => {
                // get a client id if exists
                match clients_map.get(&client_id) {
                    Some(client) => {
                        debug!("validate_client_id - done");
                        Ok(client.client_id)
                    }
                    None => {
                        error!(
                            "validate_client_id - error - client id not found - client id: {}",
                            client_id
                        );
                        Err(CommonError::NOT_FOUND)
                    }
                }
            }
            Err(_) => {
                error!(
                    "validate_client_id - error -has occurred an error while try read in app_state"
                );
                Err(CommonError::INTERNAL_SERVER_ERROR)
            }
        }
    }

    // Create a new credit on a client account from [Decimal] credit_amount based on [i32] client_id
    fn new_credit_on_client_account(
        &self,
        client_id: i32,
        credit_amount: Decimal,
    ) -> Result<Decimal, CommonError> {
        debug!("new_credit_on_client_account - start");
        
        match self.app_state.clients.write() {
            Ok(mut clients_map) => {
                match clients_map.get_mut(&client_id) {
                    Some(client) => {
                        client.balance += credit_amount;
                        debug!("new_credit_on_client_account - done");
                        debug!("new_credit_on_client_account - Client {:?}",client);
                        Ok(client.balance)
                    }
                    None => {
                        error!("new_credit_on_client_account - error - client id not found- client id:{}",client_id);
                        Err(CommonError::NOT_FOUND)
                    }
                }
            }
            Err(_) => {
                error!("new_credit_on_client_account - error -has occurred an error while try write in app_state");
                Err(CommonError::INTERNAL_SERVER_ERROR)
            }
        }
    }

    // Create new debit on a client account from [Decimal] debit_amount based on [i32] client_id
    fn new_debit_on_client_account(
        &self,
        client_id: i32,
        debit_amount: Decimal,
    ) -> Result<Decimal, CommonError> {
        debug!("new_debit_on_client_account - start");

        match self.app_state.clients.write() {
            Ok(mut clients_map) => {
                // get client and update balance
                match clients_map.get_mut(&client_id) {
                    Some(client) => {
                        client.balance -= debit_amount;
                        debug!("new_debit_on_client_account - done");
                        debug!("new_debit_on_client_account - Client {:?}",client);
                        Ok(client.balance)
                    }
                    None => {
                        error!("new_debit_on_client_account - error - client id not found- client id:{}", client_id);
                        Err(CommonError::NOT_FOUND)
                    }
                }
            }
            Err(_) => {
                error!("new_debit_on_client_account - error -has occurred an error while try write in app_state");
                Err(CommonError::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

/// Client service trait dyn type
pub type DynClientService = Arc<dyn ClientServiceTrait + Send + Sync>;
