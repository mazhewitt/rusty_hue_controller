//a rest interface for controlling hue lights

use crate::hue_controller;
use rocket::get;

// a get method which received http requests and returns a response to the client
// this is the entry point for your webserver
// the path is the path that the webserver will listen on
// the body of the function is the code that will be executed when a request is received
#[get("/toggle/<group_name>")]
pub fn toggle_group(group_name: String) -> String {
    let bridge_info = hue_controller::read_bridge_info_from_file("bridge_info.json").unwrap();
    let bridge = hue_controller::initialize_bridge(&bridge_info).unwrap();
    hue_controller::toggle_group(bridge, &group_name).unwrap();
    format!("Toggled group {}", group_name)
}





