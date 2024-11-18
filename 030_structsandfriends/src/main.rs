#[derive(Debug)]
struct WallClock {
    hours: u8,
    minutes: u8,
}

impl WallClock {
    fn new(hours: u8, minutes: u8) -> Self {
        Self { hours, minutes }
    }

    fn add_by_copying(&self, minutes: u8) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }

    fn add(&mut self, minutes: u8) {
        self.minutes += minutes;
    }
}

struct Customer {
    name: String,
    age: u8,
}

struct Order<'a> {
    customer: &'a Customer,
    product: String,
    amount: f32,
}

#[derive(Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}

fn get_longer_vector<'a>(v1: &'a Vector2, v2: &'a Vector2) -> &'a Vector2 {
    if v1.x > v2.x {
        v1
    } else {
        v2
    }
}

fn main() {
    {
        let my_clock = WallClock {
            hours: 12,
            minutes: 30,
        };
        println!("{:?}", my_clock);

        let my_other_clock = my_clock;
        //println!("The time is {:02}:{:02}", my_clock.hours, my_clock.minutes);
        println!(
            "The time is {:02}:{:02}",
            my_other_clock.hours, my_other_clock.minutes
        );

        let my_clock = Box::new(WallClock {
            hours: 12,
            minutes: 30,
        });

        let my_other_clock = &my_clock;
        println!("The time is {:02}:{:02}", my_clock.hours, my_clock.minutes);
        println!(
            "The time is {:02}:{:02}",
            my_other_clock.hours, my_other_clock.minutes
        );

        let mut my_clock = WallClock::new(12, 30);
        println!("{:?}", my_clock);

        my_clock.add(10);

        {
            let my_clock = &my_clock;
            //my_clock.add(10);
        }

        let my_clock = WallClock::new(12, 30);
        let my_clocks = vec![my_clock];

        for clock in &my_clocks {
            println!("{:?}", clock);
        }
        for clock in my_clocks {
            println!("{:?}", clock);
        }

        //drop(my_clocks);
    }

    {
        let customer = Customer {
            name: "John".to_string(),
            age: 25,
        };

        let order = Order {
            customer: &customer,
            product: "Laptop".to_string(),
            amount: 1000.0,
        };

        
        let order2 = Order {
            customer: &customer,
            product: "Bike".to_string(),
            amount: 2000.0,
        };

        
        println!("{}", order.customer.name);
        drop(customer);
    }

    {
        let v1 = Vector2 { x: 1.0, y: 2.0 };
        let longer;
        let v2 = Vector2 { x: 100.0, y: 100.0 };
        longer = get_longer_vector(&v1, &v2);
        
        println!("{:?}", longer);
    }
}
