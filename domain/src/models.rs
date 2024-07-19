use crate::schema::fichas_tecnica;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct FichaTecnica {
    pub id: i32,
    pub nome: String,
    pub sobrenome: String,
    pub numero: String,
    pub email: String,
    pub equipamento: String,
    pub defeito: String, 
    pub concluido: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = fichas_tecnica)]
pub struct NovaFichaTecnica {
    pub nome: String,
    pub sobrenome: String,
    pub numero: String,
    pub email: String,
    pub equipamento: String,
    pub defeito: String,
}