mod mapper;

use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use crate::state::AppState;
use std::sync::atomic::AtomicI32;
#[tokio::main]

async fn main() -> std::io::Result<()>{
    let app_state= AppState{
        clients: RwLock::new(HashMap::new()),
        client_id_unique: AtomicI32::new(1),
    };
    let share_state = Arc::new(app_state);
    
    // Initialize server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(share_state.clone()))
            //.service(web::resource("/clients").route(web::get().to(get_clients)))            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
 