pub fn run_arrays () {
    let mut num_array: [i32; 4] = [1,2,3,4];
    num_array[3] = 31;

    println!("{:?}", num_array);
    println!("{:?}", num_array.len());
    println!("{}", num_array[1]);
}