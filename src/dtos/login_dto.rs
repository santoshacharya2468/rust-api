use serde_derive::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct LoginDto{
   pub  email:String,
   pub password:String
}