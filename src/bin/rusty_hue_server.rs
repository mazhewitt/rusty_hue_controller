use rusty_hue_controller::hue_web_controller::*;
use rocket::*;


#[rocket::main]
async fn main() {
    let rocket = rocket::build().mount("/", routes![toggle_group]);
    rocket.launch().await.expect("Failed to launch server");
}