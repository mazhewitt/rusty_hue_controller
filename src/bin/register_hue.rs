// This file is part of the Rusty Hue Controller
use rusty_hue_controller::hue_controller::*;

fn main() {
    // promt user to press the hue button on te command line, wait until they enter yes, then continue
    println!("Have you pressed the hue button? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        let bridge_info = register_hue_client().unwrap();
        assert!(bridge_info.ip_addr.len() > 0);
        assert!(bridge_info.client_token.len() > 0);
        write_bridge_info_to_file("bridge_info.json", bridge_info).unwrap();
    }
}