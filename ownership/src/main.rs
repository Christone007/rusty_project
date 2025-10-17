fn main() {
    let s1 = String::from("Emeka");
    let s2 = 12;

    my_fun1(s1.clone()); // ownership of the string is moved from s1 to my_func, hence value has to be cloned to avoid dropping s1
    my_fun2(s2); // no cloning needed as s2 does not really Own the value
   
    println!("trying to access s1:- {}", s1);
    println!("trying to access s2:- {}", s2);

}

fn my_fun1(x: String) {
    println!("my_fun1 prints -> {x}");
}

fn my_fun2(x: i32) {
    println!("my_fun2 prints -> {x}");
}
