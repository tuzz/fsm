#[macro_use] extern crate stdweb;
#[macro_use] extern crate stdweb_derive;
#[macro_use] extern crate serde_derive;

mod webpage;

use webpage::Webpage;

fn main() {
    let _ = Webpage::new("FSM");
}
