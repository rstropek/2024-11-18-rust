#![allow(dead_code, unused)]

use std::fmt::Display;

struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl ConsultingWork {
    fn new(what: &str, hours: f32, rate: f32) -> Self {
        Self { what: what.to_string(), hours, rate }
    }
}

trait Billable {
    fn total(&self) -> f32;
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

fn give_me_something_billable() -> impl Billable {
    10.0
}

fn do_something_with_billable(billable: impl Billable) {
    println!("Total: {}", billable.total());
}

trait Pointworthy {
    fn points(&self) -> i32;
}

impl<T: Billable> Pointworthy for T {
    fn points(&self) -> i32 {
        (self.total() / 10.0) as i32
    }
}

fn do_something_with_pointworthy(pointworthy: impl Pointworthy) {
    println!("Points: {}", pointworthy.points());
}

enum PlumbingWork {
    Consultation(ConsultingWork),
    SpareSparts(f32),
}

impl Billable for PlumbingWork {
    fn total(&self) -> f32 {
        match self {
            PlumbingWork::Consultation(work) => work.total(),
            PlumbingWork::SpareSparts(price) => *price,
        }
    }
}

//impl<T: Billable, const C: usize> Billable for [T; C] {
//    fn total(&self) -> f32 {
//        self.iter().map(|item| item.total()).sum()
//    }
//}

impl<T: Pointworthy, const C: usize> Pointworthy for [T; C] {
    fn points(&self) -> i32 {
        self.iter().map(|item| item.points()).sum()
    }
}

fn main() {
    let work = ConsultingWork::new("Consulting", 10.0, 100.0);
    println!("Total: {}", work.total());
    //do_something_with_billable(work);
    do_something_with_pointworthy(work);

    let total = 100.0;
    println!("Total: {}", total.total());
    //do_something_with_billable(total);
    do_something_with_pointworthy(total);

    //do_something_with_billable([1.0, 2.0, 3.0]);
    do_something_with_pointworthy([1.0, 2.0, 3.0]);

    let some_work: Vec<Box<dyn Billable>> = vec![
        Box::new(100.0),
        Box::new(ConsultingWork::new("Consulting", 10.0, 100.0)),
    ];

    for item in some_work {
        println!("Total: {}", item.total());
    }
}
