use std::fs::File;
use std::fs;
use std::io;
use std::io::{Error, Write};
use mac_address::get_mac_address;
use hueclient;
use hueclient::{CommandLight, HueError};
use serde::{Serialize, Deserialize};
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use std::{net::IpAddr, time::Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct BridgeInfo {
    pub ip_addr: String,
    pub client_token: String,
}

// a function that turns off all the lights in a group
pub fn turn_off_group(bridge: &hueclient::Bridge, group_name: &str) -> Result<(), HueError> {
    let groups = bridge.get_all_groups()?;
    for i_group in groups {
        let group = i_group.group;
        if group.name == group_name {
            let command = CommandLight::off(Default::default());
            bridge.set_group_state(i_group.id, &command)?;
        }
    }
    Ok(())
}

// a function that turns on all the lights in a group
pub fn turn_on_group(bridge: &hueclient::Bridge, group_name: &str) -> Result<(), HueError> {
    let groups = bridge.get_all_groups()?;
    for i_group in groups {
        let group = i_group.group;
        if group.name == group_name {
            let command = CommandLight::on(Default::default());
            bridge.set_group_state(i_group.id, &command)?;
        }
    }
    Ok(())
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
pub fn toggle_group(bridge: &hueclient::Bridge, group_name: &str) -> Result<(), HueError> {
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

// Define the service name for hue bridge
const SERVICE_NAME: &str = "_hue._tcp.local";

// Define a function that discovers a hue bridge using mDNS
pub async fn discover_hue_bridge() -> Result<IpAddr, mdns::Error> {
    // Iterate through responses from each hue bridge device, asking for new devices every 15s
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);
    while let Some(Ok(response)) = stream.next().await {
        // Get the IP address of the hue bridge device by looking up A / AAAA records
        let addr = response
            .records()
            .filter_map(to_ip_addr)
            .next();
        if let Some(addr) = addr {
            // Return the IP address of the first bridge if found
            return Ok(addr);
        }
    }
    // Return an error if no hue bridge device is found
    Err(mdns::Error::Io(io::Error::new(io::ErrorKind::NotFound, "No hue bridge found")))
}

// Define a helper function that converts a record to an IP address
fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    // a test to test the discover_hue_bridge function
    #[test]
    fn can_discover_hue_bridge_using_mdns() {
        let bridge_ftr = discover_hue_bridge();
        let bridge = block_on(bridge_ftr);
        println!("bridge is {:?}", bridge);
        assert!(bridge.is_ok());
    }


    // a unit test to test toggle_group
    #[test]
    fn can_toggle_group() {
        let bridge_info = read_bridge_info_from_file("bridge_info.json").unwrap();
        let bridge = initialize_bridge(&bridge_info).unwrap();
        let result = toggle_group(&bridge, "Study");
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
        let found_bridge = match maybe_bridge {
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

    // a test for turn_on_group
    #[test]
    fn can_turn_on_group() {
        let bridge_info = read_bridge_info_from_file("bridge_info.json").unwrap();
        let bridge = initialize_bridge(&bridge_info).unwrap();
        let result = turn_on_group(&bridge, "Study");
        assert!(result.is_ok());
    }

    // a test for turn_off_group
    #[test]
    fn can_turn_off_group() {
        let bridge_info = read_bridge_info_from_file("bridge_info.json").unwrap();
        let bridge = initialize_bridge(&bridge_info).unwrap();
        let result = turn_off_group(&bridge, "Study");
        assert!(result.is_ok());
    }
}


