use crate::constants::constants::{MAIN_PATH, NEW_CLIENT_PATH, NEW_CREDIT_TRANSACTION_PATH, NEW_DEBIT_TRANSACTION_PATH};
use crate::dto::new_client_dto::NewClientDto;
use crate::dto::new_credit_transaction::NewCreditTransactionDto;
use crate::dto::new_debit_transaction::NewDebitTransactionDto;
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
    }
}

/// Maps new client end-point
async fn map_create_new_client(
    service: web::Data<DynClientService>,
    new_client: web::Json<NewClientDto>,
) -> impl Responder {
    match service.create_new_client(new_client.into_inner()).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(error) => match error {
            CommonError::FORBIDEN => HttpResponse::Forbidden().body("The document number already exists"),
            _ => HttpResponse::InternalServerError().body("Error creating new client. Try again later."),
        },
    }
}

/// Maps new credit transaction end-point
async fn map_create_new_credit_transaction(
    service: web::Data<DynClientService>,
    new_credit: web::Json<NewCreditTransactionDto>,
) -> impl Responder {
    match service.create_new_credit_transaction(new_credit.into_inner()).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => match error {
            CommonError::NOT_FOUND => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError().body("Error creating new client. Try again later."),
        },
    }
}
/// Maps new debit transaction end-point
async fn map_create_new_debit_transaction(
    service: web::Data<DynClientService>,
    new_debit: web::Json<NewDebitTransactionDto>,
) -> impl Responder {
    match service.create_new_debit_transaction(new_debit.into_inner()).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => match error {
            CommonError::NOT_FOUND => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError().body("Error creating new client. Try again later."),
        },
    }
}
