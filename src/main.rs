extern crate ssid;
use ssid::SSID;

fn main() {
  let ssid = SSID::new_query();
  println!("{:?}", ssid);
}
