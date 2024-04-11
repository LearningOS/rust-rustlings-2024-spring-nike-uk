// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {}); // Step 1: Wrap Sun in Rc

    let mercury = Planet::Mercury(Rc::clone(&sun)); // Step 2: Use Rc::clone
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun)); // Step 2: Use Rc::clone
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun)); // Step 2: Use Rc::clone
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun)); // Step 2: Use Rc::clone
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun)); // Step 2: Use Rc::clone
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun)); // Step 3: Use Rc::clone
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun)); // Step 3: Use Rc::clone
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun)); // Step 3: Use Rc::clone
    neptune.details();

    println!("reference count = {}", Rc::strong_count(&sun)); // Check reference count

    // Dropping references to Planets
    drop(neptune);
    drop(uranus);
    drop(saturn);
    drop(jupiter);
    drop(mars);
    drop(earth);
    drop(venus);
    drop(mercury);

    println!("reference count = {}", Rc::strong_count(&sun)); // Check reference count
}
