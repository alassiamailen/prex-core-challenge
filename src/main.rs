use actix_web::{web, App, HttpServer};
use env_logger;
use prex_core_challenge::controller::client_controller::ClientController;
use prex_core_challenge::controller::transaction_controller::TransactionController;
use prex_core_challenge::service::authorization_service::AuthorizationService;
use prex_core_challenge::service::client_service::{ClientService, DynClientService};
use prex_core_challenge::service::transaction_service::{
    DynTransactionService, TransactionService,
};
use prex_core_challenge::state::app_state::{AppData, AppState};
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use std::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // declarate AppState
    let app_state = AppState {
        // application data
        data: Arc::new(RwLock::new(AppData {
            // hashmap of clients
            clients: HashMap::new(),
            // hashmap of transactions
            transactions: HashMap::new(),
        })),
        // client id unique
        client_id_unique: AtomicI32::new(1),
        // transaction id unique
        transaction_id_unique: AtomicI32::new(1),
    };

    let share_state = Arc::new(app_state);
    let authorization_service = Arc::new(AuthorizationService::new());

    // create service
    let client_service: DynClientService = Arc::new(ClientService::new(
        share_state.clone(),
        authorization_service.clone(),
    ));
    let transaction_service: DynTransactionService = Arc::new(TransactionService::new(
        share_state.clone(),
        authorization_service,
    ));

    // create controller
    let client_controller = ClientController::new();
    let transaction_controller = TransactionController::new();

    // Initialize server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client_service.clone()))
            .app_data(web::Data::new(transaction_service.clone()))
            .service(client_controller.create_routes())
            .service(transaction_controller.create_routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
