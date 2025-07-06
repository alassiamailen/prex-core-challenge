use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicI32;
use prex_core_challenge::state::app_state::AppState;
use actix_web::{test, web, App};
use rust_decimal::Decimal;
use num_traits::Zero;
use prex_core_challenge::service::client_service::{DynClientService, ClientService};
use prex_core_challenge::controller::client_controller::*;
use prex_core_challenge::stub::client_info_stub::stub::{create_client_info_stub, CLIENT_ID};
use prex_core_challenge::stub::new_client_stub::stub::create_new_client_stub;
use prex_core_challenge::constants::constants::{CLIENT_BALANCE_PATH, MAIN_PATH, NEW_CLIENT_PATH, NEW_CREDIT_TRANSACTION_PATH, NEW_DEBIT_TRANSACTION_PATH};
use prex_core_challenge::dto::new_client_dto::NewClient;
use prex_core_challenge::model::client_model::Client;
use prex_core_challenge::stub::new_credit_transaction_stub::stub::create_new_credit_transaction_stub;
use prex_core_challenge::stub::new_debit_transaction_stub::stub::create_new_debit_transaction_stub;

const MOCK_CLIENT_ID: i32 = 3;
const MOCK_CLIENT_BALANCE_PATH : &str= "/client_balance/";

/// Scenario:
/// Execute map_create_new_client when [NewClient] is valid
/// Expectation:
/// A client id should be return and insert into AppState
#[actix_web::test]
async fn when_map_create_new_client_should_insert_into_app_state() {
    let client_stub= create_new_client_stub();

    let  new_client= NewClient {
        client_name: client_stub.client_name,
        birth_date: client_stub.birth_date,
        document_number: client_stub.document_number.clone(),
        country: client_stub.country,
    };

    let app_state = Arc::new(AppState {
        clients: Arc::new(RwLock::new(HashMap::new())),
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let path= format!("{}{}",MAIN_PATH,NEW_CLIENT_PATH);

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_client)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: i32 = test::read_body_json(resp).await;
    assert_eq!(body, CLIENT_ID);

    let client_in_app_state = app_state.clients.read().unwrap();
    let client = client_in_app_state.get(&CLIENT_ID).expect("client should exist");
    assert_eq!(client.document_number, client_stub.document_number);

}
/// Scenario:
/// Execute map_create_new_client when [NewClient] with this document number all ready exists
/// Expectation:
/// A [HttpResponse::Forbidden] should be returned
#[actix_web::test]
async fn when_map_create_new_client_and_this_all_ready_exists_should_return_common_error() {
    let client_stub= create_new_client_stub();
    let client= create_client_info_stub();

    let client_exists = Client{
        client_id: CLIENT_ID,
        client_name: client.client_name,
        birth_date: client.birth_date,
        document_number: client.document_number,
        country: client.country,
        balance: client.balance,
    };

    let  new_client= NewClient {
        client_name: client_stub.client_name,
        birth_date: client_stub.birth_date,
        document_number: client_stub.document_number,
        country: client_stub.country,
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(CLIENT_ID, client_exists);

    let app_state = Arc::new(AppState {
        clients: Arc::new(RwLock::new(hashmap)),
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_CLIENT_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_client)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
}

/// Scenario:
/// Execute map_create_new_client when [NewClient] is valid and read AppState failed
/// Expectation:
/// A [HttpResponse::InternalServerError] should be returned
#[actix_web::test]
async fn when_map_create_new_client_and_read_app_state_failed_should_return_common_error() {
    let client_stub= create_new_client_stub();

    let  new_client= NewClient {
        client_name: client_stub.client_name,
        birth_date: client_stub.birth_date,
        document_number: client_stub.document_number,
        country: client_stub.country,
    };

    let client = Arc::new(RwLock::new(HashMap::<i32, Client>::new()));
    let clients_ref = Arc::clone(&client);

    {
        let _ = std::thread::spawn(move || {
            let _guard = clients_ref.write().unwrap();
            panic!("error trying write");
        }).join();
    }

    let app_state = Arc::new(AppState {
        clients: client,
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_CLIENT_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_client)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
}
/// Scenario:
/// Execute map_create_new_credit_transaction when [NewCreditTransaction] is valid
/// Expectation:
/// Updates the balance and returns the current balance sheet value
#[actix_web::test]
async fn when_map_create_new_credit_transaction_is_valid_should_update_app_state() {
    let new_credit= create_new_credit_transaction_stub();
    let client_stub= create_new_client_stub();
    let expected_balance = Decimal::new(100,2);

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

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_CREDIT_TRANSACTION_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_credit)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let client_in_app_state = app_state.clients.read().unwrap();
    let client = client_in_app_state.get(&CLIENT_ID).expect("error searching client");
    assert_eq!(client.balance, expected_balance);

}

/// Scenario:
/// Execute map_create_new_credit_transaction when [NewCreditTransaction] is valid and read AppState failed
/// Expectation:
/// A [HttpResponse::InternalServerError] should be returned
#[actix_web::test]
async fn when_map_create_new_credit_transaction_and_read_app_state_failed_should_return_common_error() {
    let new_credit= create_new_credit_transaction_stub();

    let client = Arc::new(RwLock::new(HashMap::<i32, Client>::new()));

    let clients_ref = Arc::clone(&client);

    {
        let _ = std::thread::spawn(move || {
            let _guard = clients_ref.write().unwrap();
            panic!("error trying write");
        }).join();
    }

    let app_state = Arc::new(AppState {
        clients: client,
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_CREDIT_TRANSACTION_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_credit)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

}

/// Scenario:
/// Execute map_create_new_credit_transaction when [NewCreditTransaction] has an invalid client id
/// Expectation:
/// A [HttpResponse::NotFound] should be returned
#[actix_web::test]
async fn when_map_create_new_credit_transaction_and_client_id_does_not_exist_should_return_common_error() {
    let mut new_credit= create_new_credit_transaction_stub();
    let client= create_client_info_stub();

    let client_exists = Client{
        client_id: CLIENT_ID,
        client_name: client.client_name,
        birth_date: client.birth_date,
        document_number: client.document_number,
        country: client.country,
        balance: client.balance,
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(CLIENT_ID, client_exists);

    let app_state = Arc::new(AppState {
        clients: Arc::new(RwLock::new(hashmap)),
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_CREDIT_TRANSACTION_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    // modify client id
    new_credit.client_id = MOCK_CLIENT_ID;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_credit)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

}

/// Scenario:
/// Execute map_create_new_debit_transaction when [NewDebitTransaction] is valid
/// Expectation:
/// Updates the balance and returns the current balance value
#[actix_web::test]
async fn when_map_create_new_debit_transaction_is_valid_should_update_app_state() {
    let new_debit= create_new_debit_transaction_stub();
    let client_stub= create_new_client_stub();

    let client = Client{
        client_id:new_debit.client_id,
        client_name: client_stub.client_name,
        birth_date: client_stub.birth_date,
        document_number: client_stub.document_number,
        country: client_stub.country,
        balance: new_debit.debit_amount,
    };
    let mut hashmap = HashMap::new();
    hashmap.insert(new_debit.client_id, client);

    let app_state = Arc::new(AppState{
        clients: Arc::new(RwLock::new(hashmap)),
        client_id_unique: AtomicI32::new(new_debit.client_id),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_DEBIT_TRANSACTION_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_debit)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let client_in_app_state = app_state.clients.read().unwrap();
    let client = client_in_app_state.get(&CLIENT_ID).expect("error searching client");
    assert_eq!(client.balance, Decimal::zero());

}
/// Scenario:
/// Execute map_create_new_debit_transaction when [NewDebitTransaction] is valid and read AppState failed
/// Expectation:
/// A [HttpResponse::InternalServerError] should be returned
#[actix_web::test]
async fn when_map_create_new_debit_transaction_and_read_app_state_failed_should_return_common_error() {
    let new_debit= create_new_debit_transaction_stub();

    let client = Arc::new(RwLock::new(HashMap::<i32, Client>::new()));

    let clients_ref = Arc::clone(&client);

    {
        let _ = std::thread::spawn(move || {
            let _guard = clients_ref.write().unwrap();
            panic!("error trying write");
        }).join();
    }

    let app_state = Arc::new(AppState {
        clients: client,
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH,NEW_DEBIT_TRANSACTION_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_debit)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

}

/// Scenario:
/// Execute map_create_new_credit_transaction when [NewCreditTransaction] has an invalid client id
/// Expectation:
/// A [HttpResponse::NotFound] should be returned
#[actix_web::test]
async fn when_map_create_new_debit_transaction_and_client_id_does_not_exist_should_return_common_error() {
    let mut new_debit= create_new_debit_transaction_stub();
    let client= create_client_info_stub();

    let client_exists = Client{
        client_id: CLIENT_ID,
        client_name: client.client_name,
        birth_date: client.birth_date,
        document_number: client.document_number,
        country: client.country,
        balance: client.balance,
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(CLIENT_ID, client_exists);

    let app_state = Arc::new(AppState {
        clients: Arc::new(RwLock::new(hashmap)),
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let path= format!("{}{}",MAIN_PATH, NEW_DEBIT_TRANSACTION_PATH);

    let client_controller= ClientController::new(dyn_client_service.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .service(client_controller.create_routes())
    ).await;

    // modify client id
    new_debit.client_id = MOCK_CLIENT_ID;

    let req = test::TestRequest::post()
        .uri(&path)
        .set_json(&new_debit)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

}

/// Scenario:
/// Execute map_get_client_balance when client id is valid
/// Expectation:
/// A [ClientInfo] should be returned
#[actix_web::test]
async fn when_map_get_client_balance_should_get_client() {
    let client_info= create_client_info_stub();

    let client_exists = Client{
        client_id: CLIENT_ID,
        client_name: client_info.client_name,
        birth_date: client_info.birth_date,
        document_number: client_info.document_number.clone(),
        country: client_info.country,
        balance: client_info.balance,
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(CLIENT_ID, client_exists);

    let app_state = Arc::new(AppState {
        clients: Arc::new(RwLock::new(hashmap)),
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let route_pattern = format!("{}{}{{id}}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .route(&route_pattern, web::post().to(map_get_client_balance))
    ).await;

    let path= format!("{}{}{}",MAIN_PATH, MOCK_CLIENT_BALANCE_PATH,CLIENT_ID);

    let req = test::TestRequest::post()
        .uri(&path)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let client_in_app_state = app_state.clients.read().unwrap();
    let client = client_in_app_state.get(&CLIENT_ID).expect("error searching client");

    assert_eq!(client.client_id, client_info.client_id);
    assert_eq!(client.document_number, client_info.document_number);
}
/// Scenario:
/// Execute map_get_client_balance when client id is invalid
/// Expectation:
/// A [HttpResponse::NotFound] should be returned
#[actix_web::test]
async fn when_map_get_client_balance_and_client_id_is_invalid_should_common_error() {
    let client_info= create_client_info_stub();

    let client_exists = Client{
        client_id: CLIENT_ID,
        client_name: client_info.client_name,
        birth_date: client_info.birth_date,
        document_number: client_info.document_number.clone(),
        country: client_info.country,
        balance: client_info.balance,
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(CLIENT_ID, client_exists);

    let app_state = Arc::new(AppState {
        clients: Arc::new(RwLock::new(hashmap)),
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let route_pattern = format!("{}{}{{id}}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .route(&route_pattern, web::post().to(map_get_client_balance))
    ).await;

    let path= format!("{}{}{}",MAIN_PATH, MOCK_CLIENT_BALANCE_PATH,MOCK_CLIENT_ID);

    let req = test::TestRequest::post()
        .uri(&path)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(),http::StatusCode::NOT_FOUND);
}

/// Scenario:
/// Execute map_get_client_balance when client id is valid and read AppState failed
/// Expectation:
/// A [HttpResponse::NotFound] should be returned
#[actix_web::test]
async fn when_map_get_client_balance_and_read_app_state_failed_should_common_error() {
    let client_info= create_client_info_stub();

    let client = Arc::new(RwLock::new(HashMap::<i32, Client>::new()));

    let clients_ref = Arc::clone(&client);

    {
        let _ = std::thread::spawn(move || {
            let _guard = clients_ref.write().unwrap();
            panic!("error trying write");
        }).join();
    }

    let app_state = Arc::new(AppState {
        clients: client,
        client_id_unique: AtomicI32::new(CLIENT_ID),
    });

    let client_service = ClientService {
        app_state: Arc::clone(&app_state),
    };

    let dyn_client_service : DynClientService= Arc::new(client_service);
    let route_pattern = format!("{}{}{{id}}", MAIN_PATH, MOCK_CLIENT_BALANCE_PATH);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(dyn_client_service))
            .route(&route_pattern, web::post().to(map_get_client_balance))
    ).await;

    let path= format!("{}{}{}",MAIN_PATH, MOCK_CLIENT_BALANCE_PATH,CLIENT_ID);

    let req = test::TestRequest::post()
        .uri(&path)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(),http::StatusCode::INTERNAL_SERVER_ERROR);
}






