use crate::service::client_service::{DynClientService, ClientService};
use actix_web::{web, Scope, HttpResponse, Responder, post};
use actix_web::web::route;
use crate::constants::constants::{MAIN_PATH, NEW_CLIENT_PATH};


/// Client controller trait
pub trait ClientControllerTrait{
    fn config_endpoints(&self) -> Scope;
}

/// Client controller implementation struct
pub struct ClientController{
    client_service: DynClientService,
}

/// Implementation
impl ClientController{
    /// Configure declared endpoints for this controller
   pub fn new(client_service: DynClientService) -> Self{
        Self{client_service} 
    }

    async fn map_create_new_client(
        &self,
        payload: web::Json<NewClientDto>,
    ) -> impl Responder{
        match self.client_service.create_new_client(payload.into_inner()).await{
            Ok(id) => HttpResponse::Ok().json(id),
            Err(_)=> HttpResponse::InternalServerError().body("Error creating new client")
        }
    }
    
   fn create_routes(&self) -> Scope{
       let client_controller = self.clone();       
       web::scope(NEW_CLIENT_PATH)
        .route(
           "",
           web::post().to(move|payload| client_controller.map_create_new_client(payload)),
       )
       
   }
}

impl ClientControllerTrait for ClientController{
    fn config_endpoints(&self) -> Scope{
        web::scope(MAIN_PATH)
            .service(self.create_routes())
    }
}





