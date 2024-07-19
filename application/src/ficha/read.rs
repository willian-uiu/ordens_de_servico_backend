use domain::models::FichaTecnica;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;


pub fn list_ficha(ficha_id: i32) -> Result<FichaTecnica, NotFound<String>> {
    use domain::schema::fichas_tecnica;

    match fichas_tecnica::table.find(ficha_id).first::<FichaTecnica>(&mut establish_connection()) {
        Ok(ficha) => Ok(ficha),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Erro ao selecionar a ficha com o id {} - {}", ficha_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_fichas() -> Vec<FichaTecnica> {
    use domain::schema::fichas_tecnica;

    match fichas_tecnica::table.select(fichas_tecnica::all_columns).load::<FichaTecnica>(&mut establish_connection()) {
        Ok(mut fichas_tecnica) => {
            fichas_tecnica.sort();
            fichas_tecnica
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}