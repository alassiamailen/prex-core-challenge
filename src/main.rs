use actix_web::{web, App, HttpServer};
use env_logger;
use prex_core_challenge::controller::client_controller::ClientController;
use prex_core_challenge::service::client_service::{ClientService, DynClientService};
use prex_core_challenge::state::app_state::AppState;
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
        clients: Arc::new(RwLock::new(HashMap::new())),
        client_id_unique: AtomicI32::new(1),
    };
    
    let share_state = Arc::new(app_state);

    // create service
    let client_service: DynClientService = Arc::new(ClientService::new(share_state.clone()));

    // create controller
    let client_controller = ClientController::new(client_service.clone());
    // Initialize server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client_service.clone()))
            .service(client_controller.create_routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
