use serde_derive::{Deserialize,Serialize};
use diesel::{Selectable,Queryable};
use crate::schema::users;

#[derive(Debug,Serialize,Deserialize,Selectable,Queryable)]
#[diesel(table_name=users)]
pub struct User{
   pub id:i32,
   pub name:String
}