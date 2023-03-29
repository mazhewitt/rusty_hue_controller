use std::fs::File;
use std::fs;
use std::io;
use std::io::{Error, Write};
use mac_address::get_mac_address;
use hueclient;

// A function that takes a file path and returns a Result<String>
fn read_hue_token_from_file(path: &str) -> io::Result<String> {
    // Use fs::read_to_string to read the file contents into a String
    fs::read_to_string(path)
}

// A function that takes a file path and a string and returns a Result<()>
fn write_hue_token_to_file(path: &str, content: &str) -> Result<(), Error> {
    // Use File::create to create or truncate a file
    let mut file = File::create(path)?;
    // Use write! macro to write the content into the file
    write!(file, "{}", content)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_hue_token_to_from_file() {
        // Use a sample file and string for testing
        let path = "token_file.txt";
        let content = "HDJHJDHJKHDJWKSHJKDJKFDHEW";
        // Call the function and expect it to return Ok(())
        write_hue_token_to_file(path, content).expect("Should have been able to write to the file");
        // Call the read_string_from_file function and expect it to return Ok with the same content
        let result = read_hue_token_from_file(path).expect("Should have been able to read from the file");
        // Assert that the result equals the content
        assert_eq!(result, content);
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