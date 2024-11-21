use std::time;

pub struct Timer {
    name: String,
    start: time::Instant,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        Timer {
            name: name.to_string(),
            start: time::Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let took = time::Instant::now().duration_since(self.start);

        println!("{}: {:?}", self.name, took);
    }
}
