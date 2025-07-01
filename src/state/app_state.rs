use std::collections::HashMap;
use std::sync::RwLock;
use crate::model::client_model::Client;
use std::sync::atomic::AtomicI32;


pub struct AppState{
    pub clients: RwLock<HashMap<i32,Client>>,
    pub client_id_unique: AtomicI32,
}