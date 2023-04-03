use rusty_hue_controller::hue_web_controller::*;
use rocket::*;
use rusty_hue_controller::hue_controller;

#[rocket::main]
async fn main() {
    let bridge_info = hue_controller::read_bridge_info_from_file("bridge_info.json").unwrap();
    let bridge = hue_controller::initialize_bridge(&bridge_info).unwrap();

    let rocket = rocket::build().mount("/", routes![toggle_group]).manage(bridge);

    rocket.launch().await.expect("Failed to launch server");
}