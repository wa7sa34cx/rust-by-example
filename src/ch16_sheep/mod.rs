trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name);
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "fuck!"
        } else {
            "what?"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn run() {
    let mut dolly = Sheep::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
