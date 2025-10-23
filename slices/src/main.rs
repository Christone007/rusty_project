fn main() {
    let s = String::from("firstWorldWar");
    let w = &s[0..5];
    let w2 = &s[5..10];
    let w3 = &s[8..];
    println!("{w} and {w2} and {w3}");
}
