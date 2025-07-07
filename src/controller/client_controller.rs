use crate::constants::constants::{
    CLIENT_BALANCE_PATH, MAIN_PATH, NEW_CLIENT_PATH, NEW_CREDIT_TRANSACTION_PATH,
    NEW_DEBIT_TRANSACTION_PATH, STORE_BALANCE_PATH,
};
use crate::dto::new_client_dto::NewClient;
use crate::dto::new_credit_transaction::NewCreditTransaction;
use crate::dto::new_debit_transaction::NewDebitTransaction;
use crate::errors::common_error::CommonError;
use crate::service::client_service::DynClientService;

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
        web::scope(MAIN_PATH)
            .route(NEW_CLIENT_PATH, web::post().to(map_create_new_client))
            .route(
                NEW_CREDIT_TRANSACTION_PATH,
                web::post().to(map_create_new_credit_transaction),
            )
            .route(
                NEW_DEBIT_TRANSACTION_PATH,
                web::post().to(map_create_new_debit_transaction),
            )
            .route(STORE_BALANCE_PATH, web::post().to(map_create_balance_files))
            .route(CLIENT_BALANCE_PATH, web::get().to(map_get_client_balance))
    }
}

/// Maps new client end-point
pub async fn map_create_new_client(
    service: web::Data<DynClientService>,
    new_client: web::Json<NewClient>,
) -> impl Responder {
    match service.create_new_client(new_client.into_inner()).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(error) => match error {
            CommonError::Forbiden => {
                HttpResponse::Forbidden().body("The document number already exists")
            }
            _ => HttpResponse::InternalServerError()
                .body("Error creating new client. Try again later."),
        },
    }
}

/// Maps new credit transaction end-point
pub async fn map_create_new_credit_transaction(
    service: web::Data<DynClientService>,
    new_credit: web::Json<NewCreditTransaction>,
) -> impl Responder {
    match service
        .create_new_credit_transaction(new_credit.into_inner())
        .await
    {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => match error {
            CommonError::NotFound => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError()
                .body("Error creating new client. Try again later."),
        },
    }
}
/// Maps new debit transaction end-point
pub async fn map_create_new_debit_transaction(
    service: web::Data<DynClientService>,
    new_debit: web::Json<NewDebitTransaction>,
) -> impl Responder {
    match service
        .create_new_debit_transaction(new_debit.into_inner())
        .await
    {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(error) => match error {
            CommonError::NotFound => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError()
                .body("Error creating new client. Try again later."),
        },
    }
}
/// Maps create balance files
pub async fn map_create_balance_files(service: web::Data<DynClientService>) -> impl Responder {
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
        }
    }
}
/// Maps get client balance end-point
pub async fn map_get_client_balance(
    service: web::Data<DynClientService>,
    client_id: web::Path<i32>,
) -> impl Responder {
    match service.get_client_balance(client_id.into_inner()).await {
        Ok(client_info) => HttpResponse::Ok().json(client_info),
        Err(error) => match error {
            CommonError::NotFound => HttpResponse::NotFound().body("Client not found"),
            _ => HttpResponse::InternalServerError().body("An unexpected error occurred"),
        },
    }
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::constants::constants::{
        MAIN_PATH, NEW_CLIENT_PATH, NEW_CREDIT_TRANSACTION_PATH, NEW_DEBIT_TRANSACTION_PATH,
        STORE_BALANCE_PATH,
    };
    use crate::controller::client_controller::{
        map_create_balance_files, map_create_new_client, map_create_new_credit_transaction,
        map_create_new_debit_transaction, map_get_client_balance,
    };
    use crate::dto::client_info_dto::ClientInfo;
    use crate::errors::common_error::CommonError;
    use crate::service::client_service::{DynClientService, MockClientServiceTrait};
    use crate::stub::client_info_stub::stub::{create_client_info_stub, CLIENT_ID};
    use crate::stub::new_client_stub::stub::create_new_client_stub;
    use crate::stub::new_credit_transaction_stub::stub::create_new_credit_transaction_stub;
    use crate::stub::new_debit_transaction_stub::stub::create_new_debit_transaction_stub;
    use actix_web::{test, web, App};
    use http::StatusCode;
    use rust_decimal::Decimal;
    use std::future;
    use std::sync::Arc;

    const MOCK_CLIENT_BALANCE_PATH: &str = "/client_balance/";

    ///Scenario:
    /// Executes map_create_new_client endpoint flow
    /// A client id should be return
    #[actix_web::test]
    async fn when_create_new_client_should_return_ok_and_id() {
        let new_client = create_new_client_stub();

        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_create_new_client()
            .return_once(move |_p1| Box::pin(future::ready(Ok(CLIENT_ID))));

        let path = format!("{}{}", MAIN_PATH, NEW_CLIENT_PATH);
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_new_client)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&path)
            .set_json(&new_client)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: i32 = test::read_body_json(resp).await;
        assert_eq!(body, CLIENT_ID);
    }

    /// Scenario:
    /// Executes map_create_new_client when document number all ready exists
    /// A HTTP Status error should be returned
    #[actix_web::test]
    async fn when_create_new_client_and_document_number_all_ready_exists_should_return_http_error()
    {
        let new_client = create_new_client_stub();

        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_create_new_client()
            .return_once(move |_p1| Box::pin(future::ready(Err(CommonError::Forbiden))));

        let path = format!("{}{}", MAIN_PATH, NEW_CLIENT_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_new_client)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&path)
            .set_json(&new_client)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    /// Scenario:
    /// Executes map_create_new_credit_transaction endpoint flow
    /// HTTP Status 200 and the proper balance value should be returned
    #[actix_web::test]
    async fn when_map_create_new_credit_transaction_is_valid_should_return_ok_status() {
        let new_credit = create_new_credit_transaction_stub();

        let mut mock_service = MockClientServiceTrait::new();
        let expected_balance = Decimal::new(100, 2);

        mock_service
            .expect_create_new_credit_transaction()
            .return_once(move |_p1| Box::pin(future::ready(Ok(expected_balance))));

        let path = format!("{}{}", MAIN_PATH, NEW_CREDIT_TRANSACTION_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_new_credit_transaction)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&path)
            .set_json(&new_credit)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Decimal = test::read_body_json(resp).await;
        assert_eq!(body, expected_balance);
    }

    /// Scenario:
    /// Executes map_create_new_credit_transaction when service returns an error
    /// A HTTP Status error should be returned
    #[actix_web::test]
    async fn when_map_create_new_credit_transaction_but_client_id_is_invalid_should_return_http_error(
    ) {
        let new_credit = create_new_credit_transaction_stub();

        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_create_new_credit_transaction()
            .return_once(move |_p1| Box::pin(future::ready(Err(CommonError::NotFound))));

        let path = format!("{}{}", MAIN_PATH, NEW_CREDIT_TRANSACTION_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_new_credit_transaction)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&path)
            .set_json(&new_credit)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// Scenario:
    /// Executes map_create_new_debit_transaction endpoint flow
    /// HTTP Status 200 and the proper balance value should be returned
    #[actix_web::test]
    async fn when_map_create_new_debit_transaction_is_valid_should_return_ok_status() {
        let new_debit = create_new_debit_transaction_stub();

        let mut mock_service = MockClientServiceTrait::new();
        let expected_balance = Decimal::new(100, 2);

        mock_service
            .expect_create_new_debit_transaction()
            .return_once(move |_p1| Box::pin(future::ready(Ok(expected_balance))));

        let path = format!("{}{}", MAIN_PATH, NEW_DEBIT_TRANSACTION_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_new_debit_transaction)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&path)
            .set_json(&new_debit)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Decimal = test::read_body_json(resp).await;
        assert_eq!(body, expected_balance);
    }

    /// Scenario:
    /// Executes map_create_new_debit_transaction when service returns an error
    /// A HTTP Status error should be returned
    #[actix_web::test]
    async fn when_map_create_new_debit_transaction_should_return_http_error() {
        let new_debit = create_new_debit_transaction_stub();

        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_create_new_debit_transaction()
            .return_once(move |_p1| Box::pin(future::ready(Err(CommonError::NotFound))));

        let path = format!("{}{}", MAIN_PATH, NEW_DEBIT_TRANSACTION_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_new_debit_transaction)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&path)
            .set_json(&new_debit)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// Scenario:
    /// Executes map_get_client_balance endpoint flow
    /// HTTP Status 200 and the proper balance value should be returned
    #[actix_web::test]
    async fn when_map_get_client_balance_is_valid_should_return_ok_status() {
        let client = create_client_info_stub();

        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_get_client_balance()
            .return_once(move |_p1| Box::pin(future::ready(Ok(client))));

        let route_pattern = format!("{}{}{{id}}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&route_pattern, web::get().to(map_get_client_balance)),
        )
        .await;

        let path = format!("{}{}{}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH, CLIENT_ID);

        let req = test::TestRequest::get().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: ClientInfo = test::read_body_json(resp).await;
        assert_eq!(body.client_id, CLIENT_ID);
    }
    /// Scenario:
    /// Executes map_get_client_balance when service returns an error
    /// A HTTP Status error should be returned
    #[actix_web::test]
    async fn when_map_get_client_balance_should_return_http_error() {
        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_get_client_balance()
            .return_once(move |_p1| Box::pin(future::ready(Err(CommonError::NotFound))));

        let route_pattern = format!("{}{}{{id}}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&route_pattern, web::get().to(map_get_client_balance)),
        )
        .await;

        let path = format!("{}{}{}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH, CLIENT_ID);

        let req = test::TestRequest::get().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    /// Scenario:
    /// Executes map_create_balance_files endpoint flow
    /// A HTTP Status error should be returned
    #[actix_web::test]
    async fn when_map_create_balance_files_should_return_ok_status() {
        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_generate_file_with_all_clients_balances()
            .return_once(move || Box::pin(future::ready(Ok(()))));

        let path = format!("{}{}", MAIN_PATH, STORE_BALANCE_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_balance_files)),
        )
        .await;

        let req = test::TestRequest::post().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    /// Scenario:
    /// Executes map_create_balance_files when service returns an error
    /// A HTTP Status error should be returned
    #[actix_web::test]
    async fn when_map_create_balance_files_when_service_failed_should_return_return_http_error() {
        let mut mock_service = MockClientServiceTrait::new();

        mock_service
            .expect_generate_file_with_all_clients_balances()
            .return_once(move || Box::pin(future::ready(Err(CommonError::FolderCreationFailed))));

        let path = format!("{}{}", MAIN_PATH, STORE_BALANCE_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(mock_service) as DynClientService))
                .route(&path, web::post().to(map_create_balance_files)),
        )
        .await;

        let req = test::TestRequest::post().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
