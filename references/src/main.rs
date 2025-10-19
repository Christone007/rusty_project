fn main() {
    let s1 = String::from("Hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("r1 = {}\nr2 = {}",r1, r2);
}
