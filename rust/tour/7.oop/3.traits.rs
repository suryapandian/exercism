/*Polymorphism With Traits

Rust supports polymorphism with traits. Traits allow us to associate a set of methods with a struct type.

We first define the signatures of methods of a trait within:

trait MyTrait {
    fn foo(&self);
    ...
}

When a struct implements a trait, it establishes a contract that allows us to indirectly interact with the struct through the trait type (e.g. &dyn MyTrait) without having to know the real type.

A struct's implemented traits methods are defined within an implementation block:

impl MyTrait for MyStruct {
    fn foo(&self) {
        ...
    }
    ...
}

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

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();
}
