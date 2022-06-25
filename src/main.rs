mod print;
mod variables;
mod types;
mod conditionals;

pub fn main() {
    print::display();
    variables::vars();

    types::run();
    conditionals::run_conditions();
}
