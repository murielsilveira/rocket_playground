mod endpoints;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![endpoints::index])
}
