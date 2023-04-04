use rusty_hue_controller::hue_web_controller::*;
use rocket::*;
use rusty_hue_controller::*;
use std::path::Path;


#[rocket::main]
async fn main() {
    while !Path::new("bridge_info.json").exists() {
        println!("Press the hue button");
        //sleep for 2 seconds
        std::thread::sleep(std::time::Duration::from_secs(2));
        match hue_controller::register_hue_client(){
            Ok(bi) => {
                hue_controller::write_bridge_info_to_file("bridge_info.json", bi).unwrap();
                // break out of the loop
                break;
            }
            Err(_e) => {
                println!("Failed to register hue client");
                continue;
            }
        };
    }



    //if bridge_info.json exists then read it, otherwise register a new client and write it to file
    let bridge_info = match hue_controller::read_bridge_info_from_file("bridge_info.json") {
        Ok(b) => b,
        Err(_e) => {
            println!("Press the hue button and then restart the server");
            std::process::exit(1);
        }
    };

    let bridge = hue_controller::initialize_bridge(&bridge_info).unwrap();

    let rocket = rocket::build().mount("/", routes![toggle_group]).manage(bridge);

    rocket.launch().await.expect("Failed to launch server");
}