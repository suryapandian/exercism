/*
Abstraction With Selective Exposure

Rust can hide the inner workings of objects.

By default fields and methods are accessible only to the module they belong to.

The pub keyword exposes struct fields and methods outside of the module.
*/

struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}
