#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

struct MyPreciousRing {
    engraving: String
}

impl MyPreciousRing {
    fn forge() -> Self {
        Self {
            engraving: "One Ring to rule them all...".to_string(),
        }
    }
}

impl Drop for MyPreciousRing {
    fn drop(&mut self) {
        println!("Dropping MyPreciousRing with engraving: {}", self.engraving);
    }
}

fn main() {
    let mut a_ring = MyPreciousRing::forge();
    a_ring.engraving.push('!');
    
    let saurons_ring = Rc::new(a_ring);
    //saurons_ring.engraving.push('!');
    println!("The ring has now {} owners", Rc::strong_count(&saurons_ring));
    
    let frodos_ring = Clone::clone(&saurons_ring);
    println!("The ring has now {} owners", Rc::strong_count(&frodos_ring));
    println!("Dropping frodos_ring");
    drop(frodos_ring);
    
    println!("The ring has now {} owners", Rc::strong_count(&saurons_ring));
    println!("Dropping saurons_ring");
    drop(saurons_ring);
    
    //println!("{}", a_ring.engraving);
    
    let a_ring = MyPreciousRing::forge();

    let saurons_ring = Rc::new(RefCell::new(a_ring));
    let frodos_ring = saurons_ring.borrow();
    println!("{}", frodos_ring.engraving);
    drop(frodos_ring);
    
    let mut frodos_ring = saurons_ring.borrow_mut();
    println!("{}", frodos_ring.engraving);
    frodos_ring.engraving.push('!');
    drop(frodos_ring);

    let mut samwise_ring = saurons_ring.borrow_mut();
    println!("{}", samwise_ring.engraving);
    samwise_ring.engraving.push('!');
    drop(samwise_ring);
}
