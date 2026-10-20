use crate::model::client_model::Client;
use crate::model::transaction_model::Transaction;
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use std::sync::{Arc, RwLock};

/// AppData for save application data
pub struct AppData {
    // hashmap of clients
    pub clients: HashMap<i32, Client>,
    // hashmap of transactions
    pub transactions: HashMap<i32, Transaction>,
}

/// AppState for save application state
pub struct AppState {
    // application data
    pub data: Arc<RwLock<AppData>>,
    // client id unique
    pub client_id_unique: AtomicI32,
    // transaction id unique
    pub transaction_id_unique: AtomicI32,
}
