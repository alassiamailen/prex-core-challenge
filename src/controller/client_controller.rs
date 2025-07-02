use crate::service::client_service::{DynClientService};
use actix_web::{web, Scope, self,HttpResponse, Responder};
use actix_web::web::route;
use crate::constants::constants::{MAIN_PATH, NEW_CLIENT_PATH};
use crate::dto::new_client_dto::NewClientDto;


/// Client controller
#[derive(Clone)]
pub struct ClientController{    
    client_service: DynClientService
}

/// Implementation Client controller
impl ClientController{
    
   pub fn new(service: DynClientService) -> Self{
        Self{client_service: service} 
    }
    /// Configure declared endpoints for this controller
    pub fn create_routes(&self) -> Scope{
        let client_controller= self.clone();
        web::scope(MAIN_PATH)
            .route(NEW_CLIENT_PATH,
                web::post().to(map_create_new_client),
            )
    }    
  
}

/// Maps new client end-point
async fn map_create_new_client(
    service: web::Data<DynClientService>,
    new_client: web::Json<NewClientDto>,
) -> impl Responder{
    match service.create_new_client(new_client.into_inner()).await{
        Ok(id) => HttpResponse::Ok().json(id),
        Err(_)=> HttpResponse::InternalServerError().body("Error creating new client")
    }
}








