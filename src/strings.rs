pub fn run() {

    // fixed-length strings
    let name: &str = "manu";

    // growing strings
    let mut salutation: String = String::from("aje ");
    salutation.push_str(name);

    println!("{}", salutation);

    println!("{}", salutation.len());

    println!("{}", salutation.is_empty());

    println!("{}", salutation.contains("aje"));
}