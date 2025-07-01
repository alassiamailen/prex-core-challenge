use async_trait::async_trait;
use log::info;
use std::result::Result;
use crate::dto::new_client_dto::NewClientDto;
use crate::errors::common_error::CommonError;
use crate::state::AppState;

/// Client service
#[async_trait]
pub trait ClientServiceTrait {
    /// Create new Client from [NewClientDto] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client:NewClientDto)->Result<i32, CommonError>;
}
/// Client service implementation struct
pub struct ClientService{
    app_state: Arc<AppState>,
}

/// default initialization
impl Default for ClientService{
    fn default() -> Self {
        ClientService
    }
}

/// Client service implement logic
#[async_trait]
impl ClientServiceTrait for ClientService {
    /// Create a [NewClientDto] based on [NewClientDto]
    async fn create_new_client(&self, new_client:NewClientDto)->Result<i32, CommonError>{
        info!("create_new_client - start");
        let document_number = new_client.document_number;
        // verify id client exists
       match self.validate_client_id(document_number).await{
            true => {
                // Generate unique id for each client
                let client_id = self.generate_client_id();
                // dsp   aqui deberia mappear el dto al modelo y guardarlo en mi estrucutra memoria

            }
           false => {
               error!("create_new_client - error - the entered document already exists - document number: {document_number}");
               Err(CommonError::BAD_REQUEST)
           }
       }



    }
}
/// Client service "private" implement logic
impl ClientService {
/// Validate if id client exists based on [i32] client_id
async fn validate_client_id(&self,document_number: String) -> bool{
    info!("validate_client_id - start");
    /// map clients
    let clients_map= self.app_state.clients.read().await;
    /// get a document if exists
    let get_document_number = clients_map.values().any(|client: Client|client.document_number == document_number);

       if(get_document_number){
           warn!("validate_client_id - the document entered already exists");
           false
       }else{
           debug!("validate_client_id - document unique - done");
           true
       }
}
    /// Generate a client id unique
    fn generate_client_id(&self) -> i32 {
        self.app_state.client_id_unique.fetch_add(1, Ordering::SeqCst);
    }
}