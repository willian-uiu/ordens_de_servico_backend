use domain::models::{FichaTecnica, NovaFichaTecnica};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_ficha(ficha: Json<NovaFichaTecnica>) -> Created<String> {
    use domain::schema::fichas_tecnica;

    let ficha = ficha.into_inner();

    match diesel::insert_into(fichas_tecnica::table).values(&ficha).get_result::<FichaTecnica>(&mut establish_connection()) {
        Ok(ficha) => {
            let response = Response { body: ResponseBody::FichaTecnica(ficha) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}