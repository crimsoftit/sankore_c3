// loops are computer instructions that iterate until a condition is met
pub fn run_loops() {
    
    // infinite loop
    println!("       *       ");
    println!("--- infinite loop ---");

    let mut counter: i32;
    counter = 0;
    loop {
        println!("{}", counter);

        if counter >= 20 {
            break;
        }

        counter += 1;
    }

    // while loop
    let mut w_counter;
    println!("      *        ");
    println!("--- while loop ---");
    w_counter = 0;

    while w_counter <= 10 {
        println!("{}", w_counter);

        if w_counter >= 10 {
            break;
        }
        w_counter += 1;
    }

    // for loop
    println!("      *        ");
    println!("--- for loop ---");

    for x in 0..5 {
        println!("{}", x);
    }

    // loop through an array
    println!("      *        ");
    println!("--- for loop (array) ---");

    let l_array: [i32; 10] = [2,4,6,8,10,12,14,16,18,20];
    for item in l_array.iter() {
        println!("{}", item);
    }
    
}