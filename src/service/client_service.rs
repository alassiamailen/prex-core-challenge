use async_trait::async_trait;
use log::info;
use std::result::Result;
use crate::dto::new_client_dto::NewClientDto;
use crate::dto::new_credit_transaction_dto::NewCreditTransactionDto;
use crate::errors::common_error::CommonError;
use crate::state::AppState;
use crate::mapper::new_client_mapper::client_map;


/// Client service
#[async_trait]
pub trait ClientServiceTrait {
    /// Create new Client from [NewClientDto] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client:NewClientDto)->Result<i32, CommonError>;

    /// Create a new transaction from [NewCreditTransactionDto] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_credit_transaction(&self, credit_transaction: NewCreditTransactionDto) -> Result<Decimal, CommonError>;
}
/// Client service implementation struct
pub struct ClientService{
    app_state: Arc<AppState>,
}

/// Initialization
impl ClientService{
    pub fn new(app_state: Arc<AppState>) -> Self {
        ClientService{app_state}
    }
}

/// Client service implement logic
#[async_trait]
impl ClientServiceTrait for ClientService {
    /// Create new Client from [NewClientDto] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client:NewClientDto)->Result<i32, CommonError>{
        info!("create_new_client - start");
        let document_number = &new_client.document_number;

        // verify client document exists
       if(!self.validate_client_document(document_number)){
           error!("create_new_client - error - the entered document already exists - document number: {document_number}");
           return Err(CommonError::BAD_REQUEST);
       }

        // Generate unique id for each client
        let client_id = self.generate_client_id();

        // map Client from NewClientDto
        let populate_new_client= client_map(new_client,client_id);

        // Insert Client and id in app_state
        match self.app_state.clients.write(){
            Ok(app_state)=>{
                app_state.insert(client_id,populate_new_client);
                info!("create_new_client - done");
                Ok(client_id)
            }
            Err(_)=>{
                error!("create_new_client - error - has occurred an error while try write in app_state");
                Err(CommonError::INTERNAL_SERVER_ERROR)
            }
        }
    }
    /// Create a new transaction from [NewCreditTransactionDto] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_credit_transaction(&self, credit_transaction: NewCreditTransactionDto) -> Result<Decimal, CommonError>{
        info!("create_new_credit_transaction - start");
        let client_id= credit_transaction.client_id;

        // validate if client id exists
        match self.validate_client_id(client_id){
            Ok(client_id)=>{
               // update client balance
                match self.new_credit_on_client_account(client_id,credit_transaction.balance){
                    Ok(balance)=>{
                        info!("create_new_credit_transaction - done");
                        Ok(balance)
                    }
                    Err(error)=>{
                        error!("create_new_credit_transaction - error: {}",error);
                        Err(CommonError::BAD_REQUEST)
                    }
                }
            }
            Err(error)=>{
                error!("create_new_credit_transaction - error: {}",error);
                Err(CommonError::NOT_FOUND)
            }
        }
    }
}
/// Client service "private" implement logic
impl ClientService {
/// Validate if the client document number exists based on [String] document_number
fn validate_client_document(&self,document_number: &str) -> bool{
    info!("validate_client_document - start");
    /// map clients
    let clients_map= self.app_state.clients.read();
    /// get a document if exists
    let get_document_number = clients_map.values().any(|client: Client|client.document_number == document_number);

       if(get_document_number){
           warn!("validate_client_document - the document entered already exists");
           false
       }else{
           debug!("validate_client_document - document unique - done");
           true
       }
}
    /// Generate a client id unique
    fn generate_client_id(&self) -> i32 {
        self.app_state.client_id_unique.fetch_add(1, Ordering::SeqCst)
    }

    /// Validate if client id exists based on [Decimal] client_id
    fn validate_client_id(&self, client_id: i32) -> Result<i32, CommonError>{
        debug!("validate_client_id - start");

        /// map clients
        let clients_map= self.app_state.clients.read();

        /// get a client id if exists
        match clients_map.get(&client_id){
            Some(client)=>{
                debug!("validate_client_id - done");
                Ok(client.client_id)
            }
            None=>{
                error!("validate_client_id - error - client id not found client id: {}",client_id);
                Err(CommonError::NOT_FOUND)
            }
        }
    }

    // Create new credit on client account from [Decimal] credit_amount based on [i32] client_id
    fn new_credit_on_client_account(&self, client_id: i32, credit_amount: Decimal) -> Result<Decimal, CommonError>{
        debug!("new_credit_on_client_account - start");

        /// map clients
        let clients_map= self.app_state.clients.write();
        // get client and update balance
        match clients_map.get_mut(&client_id){
            Some(client)=>{
                debug!("new_credit_on_client_account - done");
                client.balance += credit_amount;
                Ok(client.balance)
            }
            None=>{
                error!("new_credit_on_client_account - error - has occurred an error while try write in app_state");
                Err(CommonError::BAD_REQUEST)
            }
        }
    }

}

/// Client service trait dyn type
pub type DynClientService = Arc<dyn ClientServiceTrait + Send  + Sync>;