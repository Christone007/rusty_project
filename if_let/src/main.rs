fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum  is configured to be {max}"),
        _ => (),
    }

    let config_min = Some(1u8);
    if let Some(min) = config_min {
        println!("The minimum value is {min}");
    }
}
