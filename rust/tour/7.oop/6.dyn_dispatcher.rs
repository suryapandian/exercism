/*
Dynamic vs Static Dispatch

Methods are executed in two ways:

    static dispatch - When the instance type is known, we have direct knowledge of what function to call.
    dynamic dispatch - When an instance type is not known, we must find out some way of calling the correct function.

Trait types &dyn MyTrait give us the ability to work with instances of objects indirectly using dynamic dispatch.

When dynamic dispatch is used, Rust will encourage you to put dyn before your trait type so people are aware.

Memory details:

    Dynamic dispatch is slightly slower because of the pointer chasing to find the real function call.


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

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn static_make_noise(creature: &SeaCreature) {
    // we know the real type
    creature.make_noise();
}

fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    // we don't know the real type
    noise_maker.make_noise();
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    static_make_noise(&creature);
    dynamic_make_noise(&creature);
}
