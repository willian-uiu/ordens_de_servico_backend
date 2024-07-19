#[macro_use] extern crate rocket;
use API::ficha_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            ficha_handler::list_fichas_handler,
            ficha_handler::list_ficha_handler,
            ficha_handler::create_ficha_handler,
            ficha_handler::publish_ficha_handler,
            ficha_handler::delete_ficha_handler,
        ])
}