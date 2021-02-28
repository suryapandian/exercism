struct SeaCreature {
    noise: String,
}

/*

Methods are defined within an implementation block with keyword impl:

&self - Immutable reference to the instance.
&mut self - Mutable reference to the instance.

*/

impl SeaCreature {
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

fn main() {
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}
