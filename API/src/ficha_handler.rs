use shared::response_models::{Response, ResponseBody};
use application::ficha::{create, read, publish, delete};
use domain::models::{FichaTecnica, NovaFichaTecnica};
use rocket::{get, post};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

// Programação das rotas:

#[get("/")]
pub fn list_fichas_handler() -> String {
    let fichas: Vec<FichaTecnica> = read::list_fichas();
    let response = Response { body: ResponseBody::FichasTecnicas(fichas) };

    serde_json::to_string(&response).unwrap()
}

#[get("/ficha/<ficha_id>")]
pub fn list_ficha_handler(ficha_id: i32) -> Result<String, NotFound<String>> {
    let ficha = read::list_ficha(ficha_id)?;
    let response = Response { body: ResponseBody::FichaTecnica(ficha)};

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/nova_ficha", format = "application/json", data = "<ficha>")]
pub fn create_ficha_handler(ficha: Json<NovaFichaTecnica>) -> Created<String> {
    create::create_ficha(ficha)
}

#[get("/publish/<ficha_id>")]
pub fn publish_ficha_handler(ficha_id: i32) -> Result<String, NotFound<String>> {
    let ficha = publish::publish_ficha(ficha_id)?;
    let response = Response { body: ResponseBody::FichaTecnica(ficha) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/delete/<ficha_id>")]
pub fn delete_ficha_handler(ficha_id: i32) -> Result<String, NotFound<String>> {
    let fichas = delete::delete_ficha(ficha_id)?;
    let response = Response { body: ResponseBody::FichasTecnicas(fichas)};

    Ok(serde_json::to_string(&response).unwrap())
}