fn main() {
    let mut body_temp = deg_to_far(37.0);
    println!("Normal body temperature of 37C is {body_temp}F");

    body_temp = far_to_deg(body_temp);
    println!("And this when converted back to Celcius is {body_temp}");
}

fn deg_to_far(temp: f32) -> f32 {
    temp * 9.0 / 5.0 + 32.0
}

fn far_to_deg(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}
