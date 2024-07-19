use domain::models::FichaTecnica;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use diesel::prelude::*;

// Essa rota faz CONLUIR A ORDEM DE SERVIÇO
pub fn publish_ficha(ficha_id: i32) -> Result<FichaTecnica, NotFound<String>> {
    use domain::schema::fichas_tecnica::dsl::*;

    match diesel::update(fichas_tecnica.find(ficha_id)).set(concluido.eq(true)).get_result::<FichaTecnica>(&mut establish_connection()) {
        Ok(ficha) => Ok(ficha),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing ficha with id {} - {}", ficha_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}