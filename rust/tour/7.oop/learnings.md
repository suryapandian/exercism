### Trait Objects

 When we pass an instance of an object to a parameter of type &dyn MyTrait we pass what is called a trait object.

A trait object is what allows us to indirectly call the correct methods of an instance. A trait object is a struct that holds the pointer of our instance with a list of function pointers to our instance's methods.

Memory details:

   This list of functions is known in C++ as a vtable.


### Handling Unsized Data

Traits introduce an interesting challenge when we want to store them within another struct. Traits obfuscate the original struct thus it also obfuscates the original size. Unsized values being stored in structs are handled in two ways in Rust:

  generics - Using parameterized types effectively create struct/functions known types and thus known sizes.
  indirection - Putting instances on the heap gives us a level of indirection that allow us to not have to worry about the size of the actual type and just store a pointer to it. There are other ways as well!
