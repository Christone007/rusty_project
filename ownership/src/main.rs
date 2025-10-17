fn main() {
    let s1 = String::from("Emeka");
    let s2 = 12;

    my_fun1(s1); // ownership of the string is moved from s1 to my_func
    my_fun2(7);
   
    println!("trying to access s1:- {}", s1);
    println!("trying to access s2:- {}", s2);

}

fn my_fun1(x: String) {
    println!("my_fun1 prints -> {x}");
}

fn my_fun2(x: i32) {
    println!("my_fun2 prints -> {x}");
}
