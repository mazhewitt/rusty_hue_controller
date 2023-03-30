use std::fs::File;
use std::fs;
use std::io;
use std::io::{Error, Write};
use mac_address::get_mac_address;
use hueclient;
use hueclient::{CommandLight, HueError};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct BridgeInfo {
    ip_addr: String,
    client_token: String,
}


// A function that takes a file path and returns a Result<String>
pub fn read_bridge_info_from_file(path: &str) -> io::Result<BridgeInfo> {
    // Use fs::read_to_string to read the file contents into a String
    let content = fs::read_to_string(path)?;
    //deserialize the bridge info
    let bridge_info: BridgeInfo = serde_json::from_str(&content).unwrap();
    Ok(bridge_info)
}

// A function that takes a file path and a string and returns a Result<()>
pub fn write_bridge_info_to_file(path: &str, bridge_info: BridgeInfo) -> Result<(), Error> {
    // Use File::create to create or truncate a file
    let mut file = File::create(path)?;
    //serialize the bridge info
    let content = serde_json::to_string(&bridge_info).unwrap();
    // Use write! macro to write the content into the file
    write!(file, "{}", content)
}

// write me a function that initializes the bridge using the devices mac address and stores the client token in a file, Return a Result<Bridge>

pub fn initialize_bridge(bridge_info: &BridgeInfo) -> Result<hueclient::Bridge, HueError> {
    let bridge = hueclient::Bridge::for_ip(std::net::IpAddr::V4(bridge_info.ip_addr.parse().unwrap()));
    Ok(bridge.with_user(bridge_info.client_token.clone()))
}

pub fn register_hue_client() -> Result<BridgeInfo, HueError> {
    let mac_addr = get_mac_address().expect("Could not get mac address");
    let mac_addr_str = mac_addr.unwrap().to_string();
    let bridge = hueclient::Bridge::discover_required()
        .register_user(&mac_addr_str)?; // Press the bridge before running this
    let token = bridge.username.clone();
    let ip_addr = bridge.ip.to_string();
    let bridge_info = BridgeInfo { ip_addr, client_token: token };
    println!("the username was {}, IP was {}", bridge_info.client_token, bridge_info.ip_addr );
    Ok(bridge_info)
}

// a function that takes a bridge and the name of a group and toggeles the light state of that group
pub fn toggle_group(bridge: hueclient::Bridge, group_name: &str) -> Result<(), HueError> {
    let groups = bridge.get_all_groups()?;
    for i_group in groups {
        let group = i_group.group;
        if group.name == group_name {
            let light_state = group.state.clone();
            let command = match light_state.all_on {
                true => CommandLight::off(Default::default()),
                false => CommandLight::on(Default::default()),
            };
            bridge.set_group_state(i_group.id, &command)?;
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    // a unit test to test toggle_group
    #[test]
    fn can_toggle_group() {
        let bridge_info = read_bridge_info_from_file("bridge_info.json").unwrap();
        let bridge = initialize_bridge(&bridge_info).unwrap();
        let result = toggle_group(bridge, "Study");
        assert!(result.is_ok());
    }


    // a unit test to test register_hue_client // have to press the hue button before running this test
    #[test]
    fn can_register_hue_client() {
        let bridge_info = register_hue_client().unwrap();
        assert!(bridge_info.ip_addr.len() > 0);
        assert!(bridge_info.client_token.len() > 0);
        write_bridge_info_to_file("bridge_info.json", bridge_info).unwrap();
    }

    #[test]
    fn can_discover_the_bridge() {
        let maybe_bridge = hueclient::Bridge::discover();
        let found_bridge = match maybe_bridge{
            None => false,
            Some(_b) => true
        };
        assert!(found_bridge)
    }

    #[test]
    fn can_get_mac_address() {
        let maybe_mac_string = get_mac_address();
        let found_mac_addr = match maybe_mac_string {
            Ok(Some(_ma)) => {
                println!("{}", _ma.to_string());
                true
            }
            Ok(None) => {
               false
            }
            Err(_e) => {
                false
            }
        };
        assert!(found_mac_addr)
    }


}