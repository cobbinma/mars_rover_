extern crate mars_rover;

pub use mars_rover::plateau;
pub use mars_rover::rover;
pub use mars_rover::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(val) => val,
        Err(e) => panic!("could not create config : {}", e),
    };

    let rovers = match mars_rover::deploy_rovers(config) {
        Ok(val) => val,
        Err(e) => panic!("could not deploy rovers : {}", e),
    };

    for rover in rovers {
        print!("{} ", rover.print());
    }
}
