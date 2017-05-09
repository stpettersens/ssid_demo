extern crate ssid;
extern crate os_type;
use ssid::SSID;

fn main() {
  let ssid = SSID::new_query();
  let os = os_type::current_platform();
  println!("{:?}", ssid);
  println!("OS = {:?}", os);
}
