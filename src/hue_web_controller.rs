//a rest interface for controlling hue lights

use hueclient::Bridge;
use crate::hue_controller;
use rocket::get;
use rocket::State;

// a get method which received http requests and returns a response to the client
// this is the entry point for your webserver
// the path is the path that the webserver will listen on
// the body of the function is the code that will be executed when a request is received
#[get("/toggle/<group_name>")]
pub fn toggle_group(bridge_state: &State<Bridge>, group_name: String) -> String {
    let bridge = bridge_state.inner();
    hue_controller::toggle_group(bridge, &group_name).unwrap();
    format!("Toggled group {}", group_name)
}





