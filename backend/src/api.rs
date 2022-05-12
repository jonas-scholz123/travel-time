use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
