pub fn run_enum() {

    let state = Status::None;
    match state {
        Status::DONE => println!("task completed..."),
        Status::UNDERWAY => println!("task is underway..."),
        Status::QUEUED => println!("task queued..."),
        _ => println!("ERROR!!")
    }
}


#[derive(Debug)]
enum Status {
    QUEUED,
    UNDERWAY,
    DONE,
    None
}