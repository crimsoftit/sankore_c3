pub fn display () {
    let sal = "manu";
    let hobby = "programming";
    println!("manu, {}", sal);

    // positional formatting
    println!("{1} loves {0}", hobby, sal);

    // named argument formatting
    println!("{name} prefers {ln1} over {ln2}", name=sal, ln1="RUST", ln2="AssemblyScript");

    // print tuples
    println!("{:?}", ("20", "25", "15", "10"));
}