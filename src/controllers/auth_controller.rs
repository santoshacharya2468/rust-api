use actix_web::{Responder,HttpResponse,web};
use crate::services::auth_service::AuthService;
use crate::dtos::{login_dto::LoginDto,register_dto::RegisterDto};
pub async  fn login(auth_service: web::Data<AuthService>,dto:web::Json<LoginDto>) -> impl Responder {
  let result = auth_service.login(dto.0);
  HttpResponse::Ok().body(result)
}

pub async fn register(auth_service: web::Data<AuthService>,dto:web::Json<RegisterDto>) -> impl Responder {
  let result = auth_service.register(dto.0);
  HttpResponse::Ok().body(result)
}
