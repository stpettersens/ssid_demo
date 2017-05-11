extern crate ssid;
use ssid::SSID;

fn main() {
  let ssid = SSID::new();
  println!("{:#?}", ssid);
  println!("SSID id = {} (state: {})", 
  ssid.get_id(), ssid.get_state());
}
