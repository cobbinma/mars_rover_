use mars_rover;
use mars_rover::Config;
use mars_rover::rover::{Rover, Bearing};

#[test]
fn it_deploys_rover() {
    let args = vec!["test", "5", "5", "1", "2", "N", "M", "3", "3", "E", "MMRMMRMRRM"];

    let first_expected = Rover::new(1, 3, Bearing::North);
    let second_expected = Rover::new(5, 1, Bearing::East);

    let config = Config::new(args).expect("should create config");

    let rovers = mars_rover::deploy_rovers(config)
        .expect("should deploy rovers");

    assert_eq!(
        rovers[0],
        first_expected
    );

    assert_eq!(
        rovers[1],
        second_expected
    );
}