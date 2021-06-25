// inside lib.rs, only the following line should be in here
pub mod entrypoint;
pub mod instruction;
pub mod error;
pub mod processor;
pub mod state;



/*
.
├─ src
│  ├─ lib.rs -> registering modules
│  ├─ entrypoint.rs -> entrypoint to the program
│  ├─ instruction.rs -> program API, (de)serializing instruction data
│  ├─ processor.rs -> program logic
│  ├─ state.rs -> program objects, (de)serializing state
│  ├─ error.rs -> program specific errors
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ Xargo.toml


The flow of a program using this structure looks like this:

Someone calls the entrypoint
The entrypoint forwards the arguments to the processor
The processor asks instruction.rs to decode the instruction_data argument from the entrypoint function.
Using the decoded data, the processor will now decide which processing function to use to process the request.
The processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint.
As you can see,

instruction.rs defines the "API" of a program

While there is only one entrypoint, program execution
 can follow different paths depending on the given instruction data that is decoded inside instruction.


 */
