use crate::dtos::{login_dto::LoginDto, register_dto::RegisterDto};
pub struct AuthService{

}
 impl AuthService {
    pub fn login(&self, user: LoginDto) -> String {
        let result = format!("Login token {:?}", user.email);
        result
    }
    pub fn register(&self, user: RegisterDto) -> String {
        let result = format!("Register token {:?}", user.name);
        result
    }
}
