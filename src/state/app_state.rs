use crate::model::client_model::Client;
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use std::sync::{Arc, RwLock};

/// AppState for save clients data
pub struct AppState {
    // hashmap of clients
    pub clients: Arc<RwLock<HashMap<i32, Client>>>,
    // client id unique
    pub client_id_unique: AtomicI32,
}
