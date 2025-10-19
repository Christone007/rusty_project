fn main() {
    let mut s1 = String::from("Hello");

    change(&mut s1);

    println!("The s1 value is {s1}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
