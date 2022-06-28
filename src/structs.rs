pub fn run_struct () {
    let employee: Employee = Employee::new("Manu", 31);
    let stmt = employee.employee_info();
    println!("{:?}", stmt);
}

struct Employee {
    name: String,
    age: u32
}

impl Employee {
    fn new (name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age
        }
    }

    fn employee_info (&self) {
        println!("{}'s age is {}", self.name, self.age);
    }
}