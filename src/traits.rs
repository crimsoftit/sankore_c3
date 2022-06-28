pub fn run_traits () {
    let bike: Kawasaki = Kawasaki::new();
    bike.start_engine();
}

trait Bike {
    fn start_engine(&self);
}

struct Kawasaki {
    name: String,
}

impl Kawasaki {
    pub fn new() -> Self {
       Self { name: "ninja 1000".to_string() } 
    }
}

impl Bike for Kawasaki {
    fn start_engine(&self) {
        println!("{}'s engine started...", self.name);
    }
}