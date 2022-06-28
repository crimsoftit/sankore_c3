pub fn run_greeting () {
    //greeting("manu");

    let n_str = show_name("manu");
    println!("{}", n_str);

    greeting("mzito");
}

fn greeting (name: &str) {
    println!("unasemaje {}", name);
}

fn show_name(name: &str) -> String {
    let mut sal = String::from("aje ");
    sal.push_str(name);

    // return statement doesn't end with a semicolon
    sal
}