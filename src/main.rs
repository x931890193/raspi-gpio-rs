use sysfs_gpio::Pin;


fn main() {
    // get args from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: raspi-gpio-rs <pin-number>");
        std::process::exit(1);
    }
    println!("Hello, raspi-gpio-rs! use pin {}", args[1]);
    let pin23 = Pin::new(args[1].parse::<u64>().unwrap());
    // export the pin
    pin23.export().expect("Unable to export pin");
    // set the direction of the pin out
    pin23.set_direction(sysfs_gpio::Direction::Out).expect("Unable to set direction");
    // set the value of the pin to high
    pin23.set_value(1).expect("Unable to set value");
    // sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
    // get the value of the pin
    assert!(pin23.get_value() == Ok(1));
    // set low the value of the pin
    pin23.set_value(0).expect("Unable to set value");
    // unexport the pin
    pin23.unexport().expect("Unable to unexport pin");
}
