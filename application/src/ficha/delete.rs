use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::FichaTecnica;

pub fn delete_ficha(ficha_id: i32) -> Result<Vec<FichaTecnica>, NotFound<String>> {
    use domain::schema::fichas_tecnica::dsl::*;
    use domain::schema::fichas_tecnica;

    let response: Response;

    let num_deleted = match diesel::delete(fichas_tecnica.filter(id.eq(ficha_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting post with id {} - {}", ficha_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    };

    if num_deleted > 0 {
        match fichas_tecnica::table.select(fichas_tecnica::all_columns).load::<FichaTecnica>(&mut establish_connection()) {
            Ok(mut fichas_tecnica_) => {
                fichas_tecnica_.sort();
                Ok(fichas_tecnica_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no ficha with id {}", ficha_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
    
}