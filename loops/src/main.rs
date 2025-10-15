fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("value: {counter}");

        if counter == 10 {
            println!("desired value found");
            break counter;
        }
    };

    println!("The result is {result}");
}
