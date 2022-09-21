// Convert Hex to base64
// Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing
extern crate base64;
use base64::encode;
fn main() {
    let a = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let encoded = hex_to_base64(a.to_string());
    println!("{}", encoded);
}
fn hex_to_base64(hex: String) -> String {
    let mut bytes = Vec::new();
    // println!("{:?}", hex.len());
    for i in 0..(hex.len() / 2) {
        // println!("{:?}", &hex[2 * i..2 * i + 2]);
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        // println!("{:?}", res);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    }
    // println!("{:?}", bytes);
    encode(&bytes)
}
