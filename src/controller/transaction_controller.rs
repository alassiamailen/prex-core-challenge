use crate::constants::constants::{
    CLIENT_TRANSACTIONS_BY_PERIOD_PATH, CLIENT_TRANSACTIONS_BY_TYPE_PATH, CLIENT_TRANSACTIONS_PATH,
    CLIENT_TRANSACTION_SUMMARY_PATH, TRANSACTION_MAIN_PATH,
};
use crate::controller::request::user_extractor::extract_user_from_headers;
use crate::service::transaction_service::DynTransactionService;
use actix_web::{self, web, HttpRequest, HttpResponse, Responder, Scope};

/// Transaction controller
#[derive(Clone)]
pub struct TransactionController;

/// Implementation Transaction controller
impl TransactionController {
    /// Initialization
    pub fn new() -> Self {
        Self
    }

    /// Configure declared endpoints for this controller
    pub fn create_routes(&self) -> Scope {
        web::scope(TRANSACTION_MAIN_PATH)
            .route(
                CLIENT_TRANSACTIONS_PATH,
                web::get().to(map_get_client_transactions),
            )
            .route(
                CLIENT_TRANSACTIONS_BY_TYPE_PATH,
                web::get().to(map_get_client_transactions_by_type),
            )
            .route(
                CLIENT_TRANSACTIONS_BY_PERIOD_PATH,
                web::get().to(map_get_client_transactions_by_period),
            )
            .route(
                CLIENT_TRANSACTION_SUMMARY_PATH,
                web::get().to(map_get_transaction_summary),
            )
    }
}

/// Maps get all transactions from a client
async fn map_get_client_transactions(
    request: HttpRequest,
    service: web::Data<DynTransactionService>,
    client_id: web::Path<i32>,
) -> impl Responder {
    let user = match extract_user_from_headers(&request) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    match service
        .get_transactions(&user, client_id.into_inner())
        .await
    {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(error) => match error {
            crate::errors::common_error::CommonError::Forbiden => {
                HttpResponse::Forbidden().finish()
            }
            crate::errors::common_error::CommonError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
/// Get transactions by type from a client
async fn map_get_client_transactions_by_type(
    request: HttpRequest,
    service: web::Data<DynTransactionService>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    let user = match extract_user_from_headers(&request) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let (client_id, transaction_type) = path.into_inner();

    let transaction_type = match transaction_type.as_str() {
        "Credit" => crate::model::transaction_model::TransactionType::Credit,
        "Debit" => crate::model::transaction_model::TransactionType::Debit,
        _ => return HttpResponse::BadRequest().finish(),
    };

    match service
        .get_transactions_by_type(&user, client_id, transaction_type)
        .await
    {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(error) => match error {
            crate::errors::common_error::CommonError::Forbiden => {
                HttpResponse::Forbidden().finish()
            }
            crate::errors::common_error::CommonError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
/// Get transactions by period from a client
async fn map_get_client_transactions_by_period(
    request: HttpRequest,
    service: web::Data<DynTransactionService>,
    path: web::Path<i32>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let user = match extract_user_from_headers(&request) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let client_id = path.into_inner();

    let from = match query.get("from") {
        Some(value) => match value.parse::<chrono::DateTime<chrono::Local>>() {
            Ok(date) => date,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::BadRequest().finish(),
    };

    let to = match query.get("to") {
        Some(value) => match value.parse::<chrono::DateTime<chrono::Local>>() {
            Ok(date) => date,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::BadRequest().finish(),
    };
    // Validate that the start date is not after the end date.
    if from > to {
        return HttpResponse::BadRequest().finish();
    }

    match service
        .get_transactions_by_period(&user, client_id, from, to)
        .await
    {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(error) => match error {
            crate::errors::common_error::CommonError::Forbiden => {
                HttpResponse::Forbidden().finish()
            }
            crate::errors::common_error::CommonError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
/// Get transaction summary from a client
async fn map_get_transaction_summary(
    request: HttpRequest,
    service: web::Data<DynTransactionService>,
    client_id: web::Path<i32>,
) -> impl Responder {
    let user = match extract_user_from_headers(&request) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    match service
        .get_transaction_summary(&user, client_id.into_inner())
        .await
    {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(error) => match error {
            crate::errors::common_error::CommonError::Forbiden => {
                HttpResponse::Forbidden().finish()
            }
            crate::errors::common_error::CommonError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::constants::constants::{
        CLIENT_TRANSACTIONS_BY_PERIOD_PATH, CLIENT_TRANSACTIONS_BY_TYPE_PATH,
        CLIENT_TRANSACTIONS_PATH, CLIENT_TRANSACTION_SUMMARY_PATH, TRANSACTION_MAIN_PATH,
    };
    use crate::controller::transaction_controller::map_get_client_transactions_by_period;
    use crate::controller::transaction_controller::map_get_client_transactions_by_type;
    use crate::controller::transaction_controller::{
        map_get_client_transactions, map_get_transaction_summary,
    };
    use crate::dto::transaction_summary_dto::TransactionSummary;
    use crate::model::transaction_model::{Transaction, TransactionType};

    use crate::errors::common_error::CommonError;
    use crate::service::transaction_service::{DynTransactionService, MockTransactionServiceTrait};
    use actix_web::{test, web, App};
    use chrono::Local;
    use http::StatusCode;
    use rust_decimal::Decimal;
    use std::future;
    use std::sync::Arc;

    const MOCK_CLIENT_ID: i32 = 1;
    const MOCK_USER_ID: i32 = 1;

    /// Scenario:
    /// Executes map_get_client_transactions endpoint flow
    /// Expectation:
    /// HTTP Status 200 and the proper transactions should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_is_valid_should_return_ok_status() {
        let transaction = Transaction {
            transaction_id: 1,
            client_id: MOCK_CLIENT_ID,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(100, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(100, 0),
        };

        let expected_transactions = vec![transaction.clone()];

        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transactions()
            .return_once(move |_user, _client_id| {
                Box::pin(future::ready(Ok(expected_transactions)))
            });

        let route_pattern = format!("{}{}", TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_client_transactions)),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: Vec<Transaction> = test::read_body_json(resp).await;

        assert_eq!(body.len(), 1);
        assert_eq!(body[0], transaction);
    }

    /// Scenario:
    /// Executes map_get_client_transactions when service returns NotFound
    /// Expectation:
    /// HTTP Status 404 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_and_client_id_is_invalid_should_return_http_error() {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transactions()
            .return_once(move |_user, _client_id| {
                Box::pin(future::ready(Err(CommonError::NotFound)))
            });

        let route_pattern = format!("{}{}", TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_client_transactions)),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    /// Scenario:
    /// Executes map_get_client_transactions without valid user headers
    /// Expectation:
    /// HTTP Status 401 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_without_valid_user_should_return_unauthorized() {
        let mock_service = MockTransactionServiceTrait::new();

        let route_pattern = format!("{}{}", TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_client_transactions)),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    /// Scenario:
    /// Executes map_get_client_transactions when service returns Forbidden
    /// Expectation:
    /// HTTP Status 403 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_without_authorization_should_return_forbidden() {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transactions()
            .return_once(move |_user, _client_id| {
                Box::pin(future::ready(Err(CommonError::Forbiden)))
            });

        let route_pattern = format!("{}{}", TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_PATH);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_client_transactions)),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID + 1)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
    /// Scenario:
    /// Executes map_get_client_transactions_by_type endpoint flow with Credit transactions
    /// Expectation:
    /// HTTP Status 200 and only Credit transactions should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_type_with_credit_should_return_ok_status() {
        let transaction = Transaction {
            transaction_id: 1,
            client_id: MOCK_CLIENT_ID,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(500, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(500, 0),
        };

        let expected_transactions = vec![transaction.clone()];

        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service.expect_get_transactions_by_type().return_once(
            move |_user, _client_id, _transaction_type| {
                Box::pin(future::ready(Ok(expected_transactions)))
            },
        );

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_TYPE_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_type),
                ),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}/type/Credit", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: Vec<Transaction> = test::read_body_json(resp).await;

        assert_eq!(body.len(), 1);
        assert_eq!(body[0], transaction);
    }

    /// Scenario:
    /// Executes map_get_client_transactions_by_type when service returns NotFound
    /// Expectation:
    /// HTTP Status 404 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_type_and_client_id_is_invalid_should_return_http_error(
    ) {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service.expect_get_transactions_by_type().return_once(
            move |_user, _client_id, _transaction_type| {
                Box::pin(future::ready(Err(CommonError::NotFound)))
            },
        );

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_TYPE_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_type),
                ),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}/type/Credit", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    /// Scenario:
    /// Executes map_get_client_transactions_by_type without valid user headers
    /// Expectation:
    /// HTTP Status 401 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_type_without_valid_user_should_return_unauthorized(
    ) {
        let mock_service = MockTransactionServiceTrait::new();

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_TYPE_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_type),
                ),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}/type/Credit", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    /// Scenario:
    /// Executes map_get_client_transactions_by_type when service returns Forbidden
    /// Expectation:
    /// HTTP Status 403 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_type_without_authorization_should_return_forbidden(
    ) {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service.expect_get_transactions_by_type().return_once(
            move |_user, _client_id, _transaction_type| {
                Box::pin(future::ready(Err(CommonError::Forbiden)))
            },
        );

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_TYPE_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_type),
                ),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}/type/Credit", MOCK_CLIENT_ID + 1)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    /// Scenario:
    /// Executes map_get_client_transactions_by_period endpoint flow with valid period
    /// Expectation:
    /// HTTP Status 200 and the proper transactions should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_period_with_valid_period_should_return_ok_status()
    {
        let transaction = Transaction {
            transaction_id: 1,
            client_id: MOCK_CLIENT_ID,
            transaction_type: TransactionType::Credit,
            amount: Decimal::new(500, 0),
            timestamp: Local::now(),
            balance_after: Decimal::new(500, 0),
        };

        let expected_transactions = vec![transaction.clone()];

        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transactions_by_period()
            .return_once(move |_user, _client_id, _from, _to| {
                Box::pin(future::ready(Ok(expected_transactions)))
            });

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_PERIOD_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_period),
                ),
        )
        .await;

        let path = format!(
            "{}{}/period?from=2026-10-01T00:00:00-03:00&to=2026-10-31T23:59:59-03:00",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: Vec<Transaction> = test::read_body_json(resp).await;

        assert_eq!(body.len(), 1);
        assert_eq!(body[0], transaction);
    }
    /// Scenario:
    /// Executes map_get_client_transactions_by_period when service returns NotFound
    /// Expectation:
    /// HTTP Status 404 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_period_and_client_id_is_invalid_should_return_http_error(
    ) {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transactions_by_period()
            .return_once(move |_user, _client_id, _from, _to| {
                Box::pin(future::ready(Err(CommonError::NotFound)))
            });

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_PERIOD_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_period),
                ),
        )
        .await;

        let path = format!(
            "{}{}?from=2026-10-01T00:00:00-03:00&to=2026-10-31T23:59:59-03:00",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    /// Scenario:
    /// Executes map_get_client_transactions_by_period without valid user headers
    /// Expectation:
    /// HTTP Status 401 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_period_without_valid_user_should_return_unauthorized(
    ) {
        let mock_service = MockTransactionServiceTrait::new();

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_PERIOD_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_period),
                ),
        )
        .await;

        let path = format!(
            "{}/{}/period?from=2026-10-01T00:00:00-03:00&to=2026-10-31T23:59:59-03:00",
            TRANSACTION_MAIN_PATH.trim_end_matches('/'),
            MOCK_CLIENT_ID + 1
        );

        let req = test::TestRequest::get().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    /// Scenario:
    /// Executes map_get_client_transactions_by_period when service returns FORBIDDEN
    /// Expectation:
    /// HTTP Status 403 should be returned
    #[actix_web::test]
    async fn when_map_get_client_transactions_by_period_and_service_returns_forbidden_should_return_forbidden(
    ) {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transactions_by_period()
            .return_once(move |_user, _client_id, _from, _to| {
                Box::pin(future::ready(Err(CommonError::Forbiden)))
            });

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_PERIOD_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_period),
                ),
        )
        .await;

        let path = format!(
            "{}/{}/period?from=2026-10-01T00:00:00-03:00&to=2026-10-31T23:59:59-03:00",
            TRANSACTION_MAIN_PATH.trim_end_matches('/'),
            MOCK_CLIENT_ID + 1
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[actix_web::test]
    async fn when_map_get_client_transactions_by_period_with_from_after_to_should_return_bad_request(
    ) {
        let mock_service = MockTransactionServiceTrait::new();

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTIONS_BY_PERIOD_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(
                    &route_pattern,
                    web::get().to(map_get_client_transactions_by_period),
                ),
        )
        .await;

        let path = format!(
            "{}/{}/period?from=2026-10-31T23:59:59-03:00&to=2026-10-01T00:00:00-03:00",
            TRANSACTION_MAIN_PATH.trim_end_matches('/'),
            MOCK_CLIENT_ID
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Scenario:
    /// Executes map_get_transaction_summary endpoint flow with valid client
    /// Expectation:
    /// HTTP Status 200 and the proper transaction summary should be returned
    #[actix_web::test]
    async fn when_map_get_transaction_summary_with_valid_client_should_return_ok_status() {
        let expected_summary = TransactionSummary {
            total_credits: Decimal::new(50012, 2),
            total_debits: Decimal::new(100, 0),
            transaction_count: 2,
        };

        let expected_summary_clone = expected_summary.clone();

        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transaction_summary()
            .return_once(move |_user, _client_id| {
                Box::pin(future::ready(Ok(expected_summary_clone)))
            });

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTION_SUMMARY_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_transaction_summary)),
        )
        .await;

        let path = format!(
            "{}{}/summary",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: TransactionSummary = test::read_body_json(resp).await;

        assert_eq!(body, expected_summary);
    }
    /// Scenario:
    /// Executes map_get_transaction_summary when service returns NotFound
    /// Expectation:
    /// HTTP Status 404 should be returned
    #[actix_web::test]
    async fn when_map_get_transaction_summary_and_client_id_is_invalid_should_return_http_error() {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transaction_summary()
            .return_once(move |_user, _client_id| {
                Box::pin(future::ready(Err(CommonError::NotFound)))
            });

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTION_SUMMARY_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_transaction_summary)),
        )
        .await;

        let path = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    /// Scenario:
    /// Executes map_get_transaction_summary without valid user headers
    /// Expectation:
    /// HTTP Status 401 should be returned
    #[actix_web::test]
    async fn when_map_get_transaction_summary_without_valid_user_should_return_unauthorized() {
        let mock_service = MockTransactionServiceTrait::new();

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTION_SUMMARY_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_transaction_summary)),
        )
        .await;

        let path = format!(
            "{}{}/summary",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get().uri(&path).to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    /// Scenario:
    /// Executes map_get_transaction_summary when service returns Forbidden
    /// Expectation:
    /// HTTP Status 403 should be returned
    #[actix_web::test]
    async fn when_map_get_transaction_summary_without_authorization_should_return_forbidden() {
        let mut mock_service = MockTransactionServiceTrait::new();

        mock_service
            .expect_get_transaction_summary()
            .return_once(move |_user, _client_id| {
                Box::pin(future::ready(Err(CommonError::Forbiden)))
            });

        let route_pattern = format!(
            "{}{}",
            TRANSACTION_MAIN_PATH, CLIENT_TRANSACTION_SUMMARY_PATH
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(mock_service) as DynTransactionService
                ))
                .route(&route_pattern, web::get().to(map_get_transaction_summary)),
        )
        .await;

        let path = format!(
            "{}{}/summary",
            TRANSACTION_MAIN_PATH,
            format!("/{}", MOCK_CLIENT_ID)
        );

        let req = test::TestRequest::get()
            .uri(&path)
            .insert_header(("X-User-Id", MOCK_USER_ID.to_string()))
            .insert_header(("X-Client-Id", MOCK_CLIENT_ID.to_string()))
            .insert_header(("X-Role", "User"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
}
