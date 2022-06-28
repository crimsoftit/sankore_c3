use std::vec;

pub fn run_refs(){
    let levels = vec![1,2,3,4,5];
    let level = 2;

    let mut e_levels = levels;
    e_levels = vec![1,2,3,4];
    let mut c_level = level;
    c_level = 4;

    println!("startup level - {} \n employee current level- {}", level, c_level);

    println!("employee current level - {} \n achieved levels- {:?}", c_level, e_levels);
}