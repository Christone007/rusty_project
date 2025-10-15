fn main() {
    println!("Hello, world!");

    another_function(5.0);
    print_labeled_measurement(5, 'a');
    let sum = add_two_numbers(5, 6);
    println!("The sum of 5 and 6 is: {sum}");
}


fn another_function(x: f32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}
