pub fn run_conditions () {
    let age = 17;
    let mut emoji: char = '\u{1F600}';
    let gender = "female";

    if age >= 18 && gender == "female" {
        println!("umetosha mboga {}", emoji);
    } else if age >=15 && age >= 17 {
        emoji = '\u{1F928}';
    } else {
        emoji = '\u{1F621}';
        println!("too young to be me lover!! {}", emoji);
    }

    // match conditional statement
    match age {
        18..=200 => println!("umetosha mboga {}", emoji),
        15..=17 => println!("unakam!! {}", emoji),
        0..=14 => println!("too young to be me lover!! {}", emoji),
        _ => println!("too young to be me lover!! {}", emoji),
    }
}