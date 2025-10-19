fn main() {
    let mut s1 = String::from("Hello");

    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;

    println!("r1 = xx\nr2 = {}", r2);
}
