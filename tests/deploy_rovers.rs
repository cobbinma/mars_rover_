use mars_rover;
use mars_rover::rover::{Bearing, Rover};
use mars_rover::Config;

#[test]
fn it_deploys_rover() {
    let args = vec![
        "test".to_string(),
        "5".to_string(),
        "5".to_string(),
        "1".to_string(),
        "2".to_string(),
        "N".to_string(),
        "M".to_string(),
        "3".to_string(),
        "3".to_string(),
        "E".to_string(),
        "MMRMMRMRRM".to_string(),
    ];

    let first_expected = Rover::new(1, 3, Bearing::North);
    let second_expected = Rover::new(5, 1, Bearing::East);

    let config = Config::new(&args).expect("should create config");

    let rovers = mars_rover::deploy_rovers(config).expect("should deploy rovers");

    assert_eq!(rovers[0], first_expected);

    assert_eq!(rovers[1], second_expected);
}
