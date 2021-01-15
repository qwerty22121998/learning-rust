struct Animal {
    name: String,
    hungry: bool,
    leg: u8,
}

impl Animal {
    fn want_to_eat(&self) -> bool {
        self.hungry
    }
}

trait Walk {
    fn walk(&self) -> bool {
        println!("not implemented");
        false
    }
}

impl Walk for Animal {
    fn walk(&self) -> bool {
        self.leg != 0
    }
}

struct Robot {
    name: String,
}

impl Walk for Robot {}

impl Robot {
    fn from_animal(a: Animal) -> Robot {
        Robot {
            name: a.name + " robot",
        }
    }
}

fn main() {
    let dog = Animal {
        name: "dog".to_string(),
        hungry: false,
        leg: 4,
    };

    let fish = Animal {
        name: "fish".to_string(),
        hungry: true,
        leg: 0,
    };

    {
        let animals = [&dog, &fish];

        for animal in &animals {
            println!("{} want to eat? : {}", animal.name, animal.want_to_eat());
            println!("{} can walk? : {}", animal.name, animal.walk());
        }
    }

    let robot = Robot {
        name: "robot".to_string(),
    };

    let dog_robot = Robot::from_animal(dog);

    println!("{} can walk?: {}", robot.name, robot.walk());
    println!("{} can walk?: {}", dog_robot.name, dog_robot.walk());
}
