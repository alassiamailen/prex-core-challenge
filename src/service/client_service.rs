use crate::dto::new_client_dto::NewClient;
use crate::dto::new_credit_transaction::NewCreditTransaction;
use crate::dto::new_debit_transaction::NewDebitTransaction;
use crate::dto::client_info_dto::ClientInfo;
use crate::errors::common_error::CommonError;
use crate::mapper::new_client_mapper::map_client;
use crate::mapper::client_info_mapper::map_client_info;
use crate::state::app_state::AppState;
use async_trait::async_trait;
use tokio::fs::{self};
use tokio::io::AsyncWriteExt;
use chrono::Local;
use std::path::Path;
use log::{debug, error, info};
use rust_decimal::Decimal;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use rust_decimal::prelude::Zero;
use crate::constants::constants::{CLIENT_BALANCE_FOLDER, PREFIX_FILE};
use crate::model::client_model::Client;
use mockall::automock;

/// Client service
#[async_trait]
#[cfg_attr(test, automock)]
pub trait ClientServiceTrait {
    /// Create new Client from [NewClient] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client: NewClient) -> Result<i32, CommonError>;

    /// Create a new credit transaction from [NewCreditTransaction] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_credit_transaction(
        &self,
        credit_transaction: NewCreditTransaction,
    ) -> Result<Decimal, CommonError>;
    /// Create a new debit transaction from [NewDebitTransaction] debit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_debit_transaction(
        &self,
        debit_transaction: NewDebitTransaction,
    ) -> Result<Decimal, CommonError>;
    /// Generate file.DAT with all client's balances
    /// Returns a [CommonError] if the file cannot be generator or service throws any error
    async fn generate_file_with_all_clients_balances(&self) -> Result<(), CommonError>;
    /// Get [ClientInfo] from [i32] client_id
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn get_client_balance(&self, client_id:i32) ->Result<ClientInfo, CommonError>;
}

/// Client service implementation struct

pub struct ClientService {
    pub app_state: Arc<AppState>,
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
    /// Create new Client from [NewClient] new_client
    /// Returns a [CommonError] if the document number already exists or service throws any error
    async fn create_new_client(&self, new_client: NewClient) -> Result<i32, CommonError> {
        info!("create_new_client - start");
        let document_number = &new_client.document_number;

        // verify client document exists
        match self.validate_client_document(document_number) {
            Ok(_exist) => {
                // Generate unique id for each client
                let client_id = self.generate_client_id();

                // map Client from NewClient
                let populate_new_client = map_client(new_client, client_id);

                // Insert Client and client id in app_state
                match self.app_state.clients.write() {
                    Ok(mut app_state) => {
                        app_state.insert(client_id, populate_new_client);
                        info!("create_new_client - done");
                        Ok(client_id)
                    }
                    Err(_) => {
                        error!("create_new_client - error - has occurred an error while try write in app_state");
                        Err(CommonError::LockWriteFailed)
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
    /// Create a new transaction from [NewCreditTransaction] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_credit_transaction(
        &self,
        transaction: NewCreditTransaction,
    ) -> Result<Decimal, CommonError> {
        info!("create_new_credit_transaction - start");
        let client_id = transaction.client_id;

        // validate if client id exists
        match self.validate_client_id(client_id) {
            Ok(client) => {
                println!("YA LEI AL CLIENTE");
                // update client balance
                match self.new_credit_on_client_account(client.client_id, transaction.credit_amount) {
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
    /// Create a new debit transaction from [NewCreditTransaction] credit_transaction
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn create_new_debit_transaction(
        &self,
        transaction: NewDebitTransaction,
    ) -> Result<Decimal, CommonError> {
        info!("create_new_debit_transaction - start");

        let client_id = transaction.client_id;

        // validate if client id exists
        match self.validate_client_id(client_id) {
            Ok(client) => {
                // update client balance
                match self.new_debit_on_client_account(client.client_id, transaction.debit_amount) {
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
    /// Generate file.txt with all client's balances
    /// Returns a [CommonError] if the file cannot be generator or service throws any error
    async fn generate_file_with_all_clients_balances(&self) -> Result<(), CommonError>{
        info!("generate_file_with_all_clients_balances - start");

        let date= Local::now();
        let date_to_string= date.format("%d%m%Y").to_string();
        match self.generate_next_balance_file_name(date_to_string).await{
            
            Ok(file_name)=>{
                
                match self.write_in_the_file_the_balance_of_the_clients(file_name).await{
                    Ok(())=>{
                        debug!("generate_file_with_all_clients_balances - done");
                        Ok(())
                    }
                    Err(error)=>{
                        error!("generate_file_with_all_clients_balances - error: {:?}", error);
                        Err(error)
                    }
                }                
            }
            Err(error) =>{
                error!("generate_file_with_all_clients_balances - error: {:?}", error);
                Err(error)
            }
        }
    }

    /// Get [ClientInfo] from [i32] client_id
    /// Returns a [CommonError] if client_id has not existed or service throws any error
    async fn get_client_balance(&self, client_id: i32) ->Result<ClientInfo, CommonError>{
        info!("get_client_balance - start");

        match self.validate_client_id(client_id){
            Ok(client) =>{
                let client_info_dto= map_client_info(client);
                debug!("get_client_balance - done");
                Ok(client_info_dto)
            }
            Err(error)=>{
                error!("get_client_balance - error: {:?}", error);
                Err(error)
            }
        }
    }
}
/// Client service "private" implement logic
impl ClientService {
    /// Validate if the client document number exists based on [String] document_number
    /// Returns a [CommonError] if RwLock cannot be read or the document number already exists
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
                    Err(CommonError::Forbiden)
                }
            }
            Err(_) => {
                error!(
                    "validate_client_id - error -has occurred an error while try read in app_state"
                );
                Err(CommonError::LockReadFailed)
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
    /// Returns a [CommonError] if the RwLock cannot be read or cannot find the Client
    fn validate_client_id(&self, client_id: i32) -> Result<Client, CommonError> {
        debug!("validate_client_id - start");

        match self.app_state.clients.read() {
            Ok(clients_map) => {
                // get a client id if exists
                match clients_map.get(&client_id) {
                    Some(client) => {
                        debug!("validate_client_id - done");
                        println!("validate_client_id YA LEI AL CLIENTE");
                        Ok(client.clone())
                    }
                    None => {
                        error!(
                            "validate_client_id - error - client id not found - client id: {}",
                            client_id
                        );
                        Err(CommonError::NotFound)
                    }
                }
            }
            Err(_) => {
                error!(
                    "validate_client_id - error -has occurred an error while try read in app_state"
                );
                Err(CommonError::LockReadFailed)
            }
        }
    }

    /// Create a new credit on a client account from [Decimal] credit_amount based on [i32] client_id
    /// Returns a [CommonError] if the RwLock cannot be written or cannot find the Client
    fn new_credit_on_client_account(
        &self,
        client_id: i32,
        credit_amount: Decimal,
    ) -> Result<Decimal, CommonError> {
        debug!("new_credit_on_client_account - start");

        match self.app_state.clients.write() {
            Ok(mut clients_map) => {
                println!("new_credit_on_client_account YA entre AL WRITE");
                match clients_map.get_mut(&client_id) {
                    Some(client) => {
                        client.balance += credit_amount;
                        debug!("new_credit_on_client_account - done");
                        debug!("new_credit_on_client_account - Client {:?}",client);
                        Ok(client.balance)
                    }
                    None => {
                        error!("new_credit_on_client_account - error - client id not found- client id:{}",client_id);
                        Err(CommonError::NotFound)
                    }
                }
            }
            Err(_) => {
                error!("new_credit_on_client_account - error -has occurred an error while try write in app_state");
                Err(CommonError::LockWriteFailed)
            }
        }
    }

    /// Create new debit on a client account from [Decimal] debit_amount based on [i32] client_id
    /// Returns a [CommonError] if the RwLock cannot be written or cannot find the Client
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
                        Err(CommonError::NotFound)
                    }
                }
            }
            Err(_) => {
                error!("new_debit_on_client_account - error -has occurred an error while try write in app_state");
                Err(CommonError::LockWriteFailed)
            }
        }
    }
    /// Generates the full path for the next client balance file in the format `DDMMYYYY_N.DAT`, based on how many files already exist in the storage folder
    /// Returns a [CommonError] if throws any error
    async fn generate_next_balance_file_name(&self,date:String) -> Result<String,CommonError>{
        debug!("generate_next_balance_file_name - start");

        let prefix_file_name= format!("{}_{}",date,"");
        // create folder
        if !Path::new(CLIENT_BALANCE_FOLDER).exists(){
           fs::create_dir(CLIENT_BALANCE_FOLDER).await.map_err(|error| {
               error!("generate_next_balance_file_name - error when creating folder error: {:?}",error);
               CommonError::FolderCreationFailed })?;
        }

        let mut file_counter= 1;
        // read folder
        let mut read_folder= fs::read_dir(CLIENT_BALANCE_FOLDER).await.map_err(|error| {
            error!("generate_next_balance_file_name - error when reading folder error: {:?}",error);
            CommonError::FolderReadFailed
        })?;
        while let Some(file) = read_folder.next_entry().await.map_err(|error| {
            error!("generate_next_balance_file_name - error when reading file error: {:?}",error);
            CommonError::FolderReadFailed
        })?{
            let get_file_name= file.file_name();
            let file_name= get_file_name.to_string_lossy();
            if file_name.starts_with(&prefix_file_name) && file_name.ends_with(PREFIX_FILE){
                file_counter+=1;
            }
        }
        let format_file_name= format!("{}/{}_{}{}",CLIENT_BALANCE_FOLDER,date,file_counter,PREFIX_FILE);
        
        Ok(format_file_name)

    }
    /// Create a text file and save client balances
    /// Returns a [CommonError] if throws any error
    async fn write_in_the_file_the_balance_of_the_clients(&self, format_file_name: String)->Result<(),CommonError> {
        debug!("write_in_the_file_the_balance_of_the_clients - start");

        let mut temporal_client_data: Vec<(i32, Decimal)> = Vec::new();
        {
            let clients_map = self.app_state.clients.read().map_err(|error| {
                error!("write_in_the_file_the_balance_of_the_clients - error when reading app_state - error: {:?}",error);
                CommonError::LockReadFailed
            })?;
            for client in clients_map.values() {
                temporal_client_data.push((client.client_id, client.balance));
            }
        }
        // sort client id in ascending order
        temporal_client_data.sort_by_key(|(client_id,_)|*client_id);

        let mut new_file = fs::File::create(&format_file_name).await.map_err(|error| {
            error!("write_in_the_file_the_balance_of_the_clients - error when creating file error: {:?}",error);
            CommonError::FileCreationFailed
        })?;

        for (client_id, balance) in temporal_client_data {
            // format client id and balance
            let each_client = format!("{:02} {:.2}\n", client_id, balance);
            new_file.write_all(each_client.as_bytes()).await.map_err(|error| {
                error!("write_in_the_file_the_balance_of_the_clients - error when writing to the file - file name: {format_file_name} - error: {:?}",error);              
                CommonError::FileWriteFailed
            })?;
        }
        // update balances in app_state
        let mut clients_map = self.app_state.clients.write().map_err(|error| {
            error!("write_in_the_file_the_balance_of_the_clients - error when writing app_state - error: {:?}",error);
            CommonError::LockWriteFailed
        })?;
        for client in clients_map.values_mut() {
           client.balance= Decimal::zero();
        }
        debug!("write_in_the_file_the_balance_of_the_clients - done");
        Ok(())
    }
}

/// Client service trait dyn type
pub type DynClientService = Arc<dyn ClientServiceTrait + Send + Sync>;

/// Unit tests cases
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::future;
    use serial_test::serial;
    use std::sync::atomic::AtomicI32;
    use std::sync::RwLock;
    use crate::model::client_model::Client;
    use super::*;
    use crate::stub::new_client_stub::stub::create_new_client_stub;
    use crate::service::client_service::{MockClientServiceTrait,ClientService};
    use crate::stub::client_info_stub::stub::create_client_info_stub;
    use crate::stub::new_credit_transaction_stub::stub::create_new_credit_transaction_stub;
    use crate::stub::new_debit_transaction_stub::stub::create_new_debit_transaction_stub;


    const MOCK_CLIENT_ID: i32 = 1;
    
    /// Scenario:
    /// Execute create_new_client when [NewClient] is valid
    /// Expectation:
    /// A client id should be returned
    #[tokio::test]
    #[serial]
   async fn when_create_new_client_with_valid_values_should_return_client_id(){
        let new_client= create_new_client_stub();

        let mut client_service_mock= MockClientServiceTrait::new();

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(HashMap::new())),
            client_id_unique: AtomicI32::new(MOCK_CLIENT_ID),
        });
        let client_service = ClientService::new(app_state);

        client_service_mock
            .expect_create_new_client()
            .return_once(move |_|Box::pin(future::ready(Ok(MOCK_CLIENT_ID))));

        let expected_result= client_service.create_new_client(new_client).await.unwrap();

        assert_eq!(MOCK_CLIENT_ID,expected_result);
    }

    /// Scenario:
    /// Execute create_new_client when [NewClient] is valid but write RwLock failed
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    #[serial]
    async fn when_create_new_client_with_valid_values_and_rwlock_failed_should_return_common_error(){
        let new_client= create_new_client_stub();

        let client = Arc::new(RwLock::new(HashMap::<i32, Client>::new()));

        let mut client_service_mock= MockClientServiceTrait::new();

        let _ = std::panic::catch_unwind(|| {
            let _write_lock = client.write().unwrap();
            panic!("error trying write");
        });

        let app_state = Arc::new(AppState{
            clients: client,
            client_id_unique: AtomicI32::new(MOCK_CLIENT_ID),
        });

        let client_service = ClientService::new(app_state);

        client_service_mock
            .expect_create_new_client()
            .return_once(move |_|Box::pin(future::ready(Err(CommonError::LockReadFailed))));

        let expected_result= client_service.create_new_client(new_client).await;

        assert_eq!(CommonError::LockReadFailed, expected_result.unwrap_err());

    }


    // #[tokio::tests]
    // async fn when_create_new_client_with_valid_values_and_rwlock_failed_when_try_write_should_return_common_error(){
    //     let new_client= create_new_client_stub();
    //
    //     let mut client_service_mock= MockClientServiceTrait::new();
    //
    //     let client_for_read = RwLock::new(HashMap::<i32, Client>::new());
    //     //let client_for_write =Arc::clone(&client_for_read);
    //     //let client_for_state= Arc::clone(&client_for_read);
    //
    //     {
    //         let read = client_for_read.read().unwrap();
    //         info!("entre al read");
    //         assert!(read.is_empty());
    //     }
    //     // let _ = thread::spawn(move || {
    //     //     let _write_lock = client_for_write.write().unwrap();
    //     //     info!("entre al write");
    //     //     panic!("error");
    //     // }).join();
    //
    //     //let app_state_client= Arc::try_unwrap(client_for_read).ok().unwrap();
    //
    //     let app_state = Arc::new(AppState{
    //         clients: client_for_read,
    //         client_id_unique: AtomicI32::new(CLIENT_ID),
    //     });
    //
    //     client_service_mock
    //         .expect_create_new_client()
    //         .returning(move |_|Box::pin(future::ready(Err(CommonError::LockWriteFailed))));
    //
    //     let client_service = ClientService::new(app_state);
    //
    //     let expected_result= client_service.create_new_client(new_client).await;
    //
    //     assert_eq!(CommonError::LockWriteFailed, expected_result.unwrap_err());
    //
    // }

    /// Scenario:
    /// Execute create_new_credit_transaction when [NewCreditTransaction] is valid
    /// Expectation:
    /// A client id should be returned
    #[tokio::test]
    #[serial]
    async fn when_create_new_credit_transaction_with_valid_values_should_return_client_id(){
        let new_credit= create_new_credit_transaction_stub();
        let client_stub= create_new_client_stub();
        let balance = Decimal::new(100,2);

        let client = Client{
            client_id:new_credit.client_id,
            client_name: client_stub.client_name,
            birth_date: client_stub.birth_date,
            document_number: client_stub.document_number,
            country: client_stub.country,
            balance: Decimal::zero(),
        };

        let mut client_service_mock= MockClientServiceTrait::new();
        let mut hashmap = HashMap::new();
        hashmap.insert(new_credit.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(new_credit.client_id),
        });

        let client_service = ClientService::new(app_state);

        client_service_mock
            .expect_create_new_credit_transaction()
            .return_once(move |_|Box::pin(future::ready(Ok(balance))));

        let expected_result= client_service.create_new_credit_transaction(new_credit).await.unwrap();

        assert_eq!(balance, expected_result);
    }

    /// Scenario:
    /// Execute create_new_credit_transaction when [NewCreditTransaction] is invalid client id
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    #[serial]
    async fn when_create_new_credit_transaction_with_invalid_values_should_return_common_error(){
        let new_credit= create_new_credit_transaction_stub();
        let client_stub= create_new_client_stub();

        let client = Client{
            client_id:new_credit.client_id,
            client_name: client_stub.client_name,
            birth_date: client_stub.birth_date,
            document_number: client_stub.document_number,
            country: client_stub.country,
            balance: Decimal::zero(),
        };


        let mut hashmap = HashMap::new();
        hashmap.insert(new_credit.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(new_credit.client_id),
        });


        {   let read_lock = app_state.clients.read().unwrap();
            assert!(read_lock.contains_key(&new_credit.client_id));
        }
        {
            let mut write_lock = app_state.clients.write().unwrap();
            write_lock.remove(&new_credit.client_id);
            write_lock.get(&new_credit.client_id);
            assert!(write_lock.get(&new_credit.client_id).is_none());
        }

        let client_service = ClientService::new(app_state);

        let expected_result= client_service.create_new_credit_transaction(new_credit).await;

        assert_eq!(CommonError::NotFound, expected_result.unwrap_err());

    }

    /// Scenario:
    /// Execute create_new_debit_transaction when [NewDebitTransaction] is valid
    /// Expectation:
    /// A client id should be returned
    #[tokio::test]
    #[serial]
    async fn when_create_new_debit_transaction_with_valid_values_should_return_client_id(){
        let new_debit= create_new_debit_transaction_stub();
        let client_stub= create_new_client_stub();
        let balance = Decimal::new(100,2);
        let updated_balance = Decimal::new(0, 2);

        let client = Client{
            client_id:new_debit.client_id,
            client_name: client_stub.client_name,
            birth_date: client_stub.birth_date,
            document_number: client_stub.document_number,
            country: client_stub.country,
            balance,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert(new_debit.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(new_debit.client_id),
        });

        let client_service = ClientService::new(app_state);

        let expected_result= client_service.create_new_debit_transaction(new_debit).await.unwrap();

        assert_eq!(updated_balance, expected_result);
    }

    /// Scenario:
    /// Execute create_new_debit_transaction when [NewDebitTransaction] is valid but write RwLock failed
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    #[serial]
    async fn when_create_new_debit_transaction_with_valid_values_and_rwlock_failed_should_return_common_error(){
        let new_debit= create_new_debit_transaction_stub();

        let client = Arc::new(RwLock::new(HashMap::<i32, Client>::new()));

        let _ = std::panic::catch_unwind(|| {
            let _readlock = client.read().unwrap();
            panic!("error trying read");
        });

        let app_state = Arc::new(AppState{
            clients: client,
            client_id_unique: AtomicI32::new(MOCK_CLIENT_ID),
        });

        let client_service = ClientService::new(app_state);

        let expected_result= client_service.create_new_debit_transaction(new_debit).await;

        assert_eq!(CommonError::NotFound, expected_result.unwrap_err());

    }

    /// Scenario:
    /// Execute get_client_balance when [client_id] is valid
    /// Expectation:
    /// A [ClientInfo] should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_client_balance_with_valid_values_and_rwlock_failed_should_return_common_error(){
        let client= create_client_info_stub();
        let result_client_info= create_client_info_stub();
        let client_id= client.client_id;

        let client = Client{
            client_id:client.client_id,
            client_name: client.client_name,
            birth_date: client.birth_date,
            document_number: client.document_number,
            country: client.country,
            balance:client.balance,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert(client.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(client_id),
        });

        let client_service = ClientService::new(app_state);

        let expected_result= client_service.get_client_balance(client_id).await.unwrap();

        assert_eq!(result_client_info, expected_result);

    }

    /// Scenario:
    /// Execute get_client_balance when [client_id] is invalid by client id
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    #[serial]
    async fn when_get_client_balance_with_invalid_client_id_and_rwlock_failed_should_return_common_error(){
        let client= create_client_info_stub();
        let client_id= client.client_id;

        let client = Client{
            client_id:client.client_id,
            client_name: client.client_name,
            birth_date: client.birth_date,
            document_number: client.document_number,
            country: client.country,
            balance:client.balance,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert(client.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(client_id),
        });


        let client_service = ClientService::new(app_state);

        let expected_result= client_service.get_client_balance(3).await;

        assert_eq!(CommonError::NotFound, expected_result.unwrap_err());

    }
    /// Scenario:
    /// Execute generate_file_with_all_clients_balances with exit
    /// Expectation:
    /// A [Ok()] should be returned
    #[tokio::test]
    #[serial]
    async fn when_generate_file_with_all_clients_balances_with_exit_should_return_ok(){
        let client= create_client_info_stub();
        let client_id= client.client_id;

        let client = Client{
            client_id:client.client_id,
            client_name: client.client_name,
            birth_date: client.birth_date,
            document_number: client.document_number,
            country: client.country,
            balance:client.balance,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert(client.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(client_id),
        });

        let _ = tokio::fs::remove_dir_all(CLIENT_BALANCE_FOLDER).await;

        let client_service = ClientService::new(app_state.clone());

        let expected_result= client_service.generate_file_with_all_clients_balances().await;
        assert!(expected_result.is_ok());

        let folder = std::fs::read_dir(CLIENT_BALANCE_FOLDER)
            .unwrap()
            .filter_map(|file| file.ok())
            .collect::<Vec<_>>();
        assert!(!folder.is_empty());

        let read_lock= app_state.clients.read().unwrap();
        let client= read_lock.get(&client_id).unwrap();
        assert_eq!(client.balance,  Decimal::zero());

    }
    /// Scenario:
    /// Execute generate_file_with_all_clients_balances and create folder failed
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    #[serial]
    async fn when_generate_file_with_all_clients_balances_and_create_folder_failed_should_return_common_error(){
        let client= create_client_info_stub();
        let client_id= client.client_id;

        let _ = std::fs::remove_file(CLIENT_BALANCE_FOLDER);
        let _ = std::fs::remove_dir_all(CLIENT_BALANCE_FOLDER);

        File::create(CLIENT_BALANCE_FOLDER).expect("Cannot create file");

        let client = Client{
            client_id:client.client_id,
            client_name: client.client_name,
            birth_date: client.birth_date,
            document_number: client.document_number,
            country: client.country,
            balance:client.balance,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert(client.client_id, client);

        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(client_id),
        });

        let client_service = ClientService::new(app_state.clone());

        let expected_result= client_service.generate_file_with_all_clients_balances().await;

        assert_eq!(CommonError::FolderReadFailed, expected_result.unwrap_err());

        let _ = std::fs::remove_file(CLIENT_BALANCE_FOLDER);

    }

    /// Scenario:
    /// Execute generate_file_with_all_clients_balances and failed when try write in lock
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    #[serial]
    async fn when_generate_file_with_all_clients_balances_and_failed_try_write_in_lock_should_return_common_error(){
        let client= create_client_info_stub();
        let client_id= client.client_id;

        let client = Client{
            client_id:client.client_id,
            client_name: client.client_name,
            birth_date: client.birth_date,
            document_number: client.document_number,
            country: client.country,
            balance:client.balance,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert(client.client_id, client);       
        
        let _ = tokio::fs::remove_dir_all(CLIENT_BALANCE_FOLDER).await;
        let app_state = Arc::new(AppState{
            clients: Arc::new(RwLock::new(hashmap)),
            client_id_unique: AtomicI32::new(client_id),
        });
        let app_state_clone = app_state.clone();
        let _ = std::thread::spawn(move || {
            let _write_lock = app_state_clone.clients.write().unwrap();
            panic!("error");
        }).join();
        
        let client_service = ClientService::new(app_state.clone());

        let expected_result= client_service.generate_file_with_all_clients_balances().await;       

        let folder = std::fs::read_dir(CLIENT_BALANCE_FOLDER)
            .unwrap()
            .filter_map(|file| file.ok())
            .collect::<Vec<_>>();
        assert!(folder.is_empty());

        assert_eq!(CommonError::LockReadFailed, expected_result.unwrap_err());
    }




}
