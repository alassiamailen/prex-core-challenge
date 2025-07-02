mod mapper;
mod constants;
mod dto;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use crate::state::AppState;
use std::sync::atomic::AtomicI32;
use crate::controller::client_controller::{ClientController,ClientControllerTrait};
use crate::service::client_service::{ClientService, DynClientService};

#[actix_web::main]

async fn main() -> std::io::Result<()>{
    let app_state= AppState{
        clients: RwLock::new(HashMap::new()),
        client_id_unique: AtomicI32::new(1),
    };
    
    let share_state = Arc::new(app_state);

    // Crear el servicio (si tu controller lo necesita)
    let client_service: DynClientService = Arc::new(ClientService::new(share_state.clone()));

    // Crear el controller
    let client_controller = ClientController::new(client_service);
    // Initialize server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(share_state.clone()))
            .service(client_controller.config_endpoints())            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
 