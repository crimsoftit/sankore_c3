mod print;
mod variables;
mod types;
mod conditionals;
mod strings;
mod arrays;
mod vectors;
mod tuples;
mod loops;
mod functions;
mod structs;
mod traits;
mod enums;
mod references_n_pointers;

pub fn main() {
    print::display();

    println!("              ");
    println!("*** VARIABLES ***");
    variables::vars();

    println!("              ");
    println!("*** DATA TYPES ***");
    types::run();

    println!("              ");
    println!("*** CONDITIONALS ***");
    conditionals::run_conditions();

    println!("              ");
    println!("*** STRINGS ***");
    strings::run();

    println!("              ");
    println!("*** ARRAYS ***");
    arrays::run_arrays();

    println!("              ");
    println!("*** VECTORS ***");
    vectors::run_vectors();

    println!("              ");
    println!("*** TUPLES ***");
    tuples::run_tuples();

    println!("              ");
    println!("*** LOOPS ***");
    loops::run_loops();

    println!("              ");
    println!("*** FUNCTIONS ***");
    functions::run_greeting();

    println!("              ");
    println!("*** STRUCTS ***");
    structs::run_struct();

    println!("              ");
    println!("*** TRAITS ***");
    traits::run_traits();

    println!("              ");
    println!("*** ENUMS ***");
    enums::run_enum();

    println!("              ");
    println!("*** REFERENCES 'N POINTERS ***");
    references_n_pointers::run_refs();
}
