// tuples: groups values of various data types
pub fn run_tuples () {
    let employee: (&str, u32, f32) = ("manu", 31, 2000.50);
    println!("{} {}", employee.0, employee.1);

    // print the entire tuple
    println!("{:?}", &employee);
}