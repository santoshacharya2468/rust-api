use actix_web::web;
use crate::controllers::auth_controller::{login,register};
use crate::services::auth_service::AuthService;

pub  fn routes(router: &mut web::ServiceConfig) {
   
    let auth_service = 
        AuthService {
           
        };
    
    router
         .service(web::scope("/auth")
         .app_data(web::Data::new(auth_service))
         .route("/login", web::post().to(login))
         .route("/register", web::post().to(register))  
);
}