/*
Its definition is like a blueprint for a compiler on how to layout the fields in memory nearby each other.
*/

struct SeaCreature {
    // String is a struct
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

/*
Rust programs have 3 memory regions where data is stored:

    data memory - For data that is fixed in size and static (i.e. always available through life of program). Consider the text in your program (e.g. "Hello World!"): This text's bytes are only ever read from one place and therefore can be stored in this region. Compilers make lots of optimizations with this kind of data, and they are generally considered very fast to use since locations are known and fixed.
    stack memory - For data that is declared as variables within a function. The location of this memory never changes for the duration of a function call; because of this compilers can optimize code so stack data is very fast to access.
    heap memory - For data that is created while the application is running. Data in this region may be added, moved, removed, resized, etc. Because of its dynamic nature it's generally considered slower to use, but it allows for much more creative usages of memory. When data is added to this region we call it an allocation. When data is removed from this section we call it a deallocation.
    */

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("brain"),
    };

//short hand operator to copy value.

   let ashley = SeaCreature{
       name: String::from("Ashley"),
       ..sarah
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}
