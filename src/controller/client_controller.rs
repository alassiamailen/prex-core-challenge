use crate::constants::constants::{MAIN_PATH, CLIENT_BALANCE_PATH, NEW_CLIENT_PATH, NEW_CREDIT_TRANSACTION_PATH, NEW_DEBIT_TRANSACTION_PATH, STORE_BALANCE_PATH};
use crate::dto::new_client_dto::NewClient;
use crate::dto::new_credit_transaction::NewCreditTransaction;
use crate::dto::new_debit_transaction::NewDebitTransaction;
use crate::errors::common_error::CommonError;
use crate::service::client_service::DynClientService;
use actix_web::web::route;
use actix_web::{self, web, HttpResponse, Responder, Scope};

/// Client controller
#[derive(Clone)]
pub struct ClientController {
    client_service: DynClientService,
}

/// Implementation Client controller
impl ClientController {
    pub fn new(service: DynClientService) -> Self {
        Self {
            client_service: service,
        }
    }
    /// Configure declared endpoints for this controller
    pub fn create_routes(&self) -> Scope {
        let client_controller = self.clone();
        web::scope(MAIN_PATH)
            .route(NEW_CLIENT_PATH, web::post().to(map_create_new_client))
            .route(NEW_CREDIT_TRANSACTION_PATH, web::post().to(map_create_new_credit_transaction))
            .route(NEW_DEBIT_TRANSACTION_PATH, web::post().to(map_create_new_debit_transaction))
            .route(STORE_BALANCE_PATH, web::post().to(map_create_balance_files))
            .route(CLIENT_BALANCE_PATH, web::get().to(map_get_client_balance))
    }
}

/// Maps new client end-point
async fn map_create_new_client(
    service: web::Data<DynClientService>,
    new_client: web::Json<NewClient>,
) -> impl Responder {
    match service.create_new_client(new_client.into_inner()).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(error) => match error {
            CommonError::Forbiden => HttpResponse::Forbidden().body("The document number already exists"),
            _ => HttpResponse::InternalServerError().body("Error creating new client. Try again later."),
        },
    }
}

/// Maps new credit transaction end-point
async fn map_create_new_credit_transaction(
    service: web::Data<DynClientService>,
    new_credit: web::Json<NewCreditTransaction>,
) -> impl Responder {
    match service.create_new_credit_transaction(new_credit.into_inner()).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => match error {
            CommonError::NotFound => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError().body("Error creating new client. Try again later."),
        },
    }
}
/// Maps new debit transaction end-point
async fn map_create_new_debit_transaction(
    service: web::Data<DynClientService>,
    new_debit: web::Json<NewDebitTransaction>,
) -> impl Responder {
    match service.create_new_debit_transaction(new_debit.into_inner()).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => match error {
            CommonError::NotFound => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError().body("Error creating new client. Try again later."),
        },
    }
}
/// Maps create balance files
async fn map_create_balance_files(
    service: web::Data<DynClientService>,    
) -> impl Responder {
    match service.generate_file_with_all_clients_balances().await {
        Ok(_) => HttpResponse::Ok().body("File created successfully"),
        Err(error) => {
            let message = match error {
                CommonError::FolderCreationFailed => "Error when creating folder",
                CommonError::FolderReadFailed => "Error when reading folder",
                CommonError::LockReadFailed => "Error when reading app_state",
                CommonError::LockWriteFailed => "Error when writing app_state",
                CommonError::FileCreationFailed => "Error when creating file",
                CommonError::FileWriteFailed => "Error when writing to the file",
                _ => "An unexpected error occurred",
            };
            HttpResponse::InternalServerError().body(message)
        },
    }
}
/// Maps get client balance end-point
async fn map_get_client_balance(
    service: web::Data<DynClientService>,
    client_id: web::Path<i32>,
) -> impl Responder {
    match service.get_client_balance(client_id.into_inner()).await {
        Ok(client_info) => HttpResponse::Ok().json(client_info),
        Err(error) => match error {
            CommonError::NotFound => HttpResponse::NotFound().body("Client not found"),
            CommonError::LockReadFailed => HttpResponse::NotFound().body("Error when reading app_state"),
            _ => HttpResponse::InternalServerError().body("An unexpected error occurred"),
        },
    }
}
