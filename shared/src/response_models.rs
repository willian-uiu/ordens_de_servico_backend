use domain::models::FichaTecnica;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    FichaTecnica(FichaTecnica),
    FichasTecnicas(Vec<FichaTecnica>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}