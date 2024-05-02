# Rust-ToDo
Rust CLI ToDo list project

Adapted from the guide at https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/

Added some printing features on top of their design to get used to working with the new tools they introduced. Will maybe add more functionality as I get more comfortable.

Update: I have been working in Rust for a few months now, and I wanted to reflect on my first Rust project:
  1) My solution, like the guide it is based off of, is not idiomatic with regards to naming, structure, or error handling. Were I to write this again I would utilitze pattern matching and result types for all fallible operations instead of expect and unwrap, both of which are poor form for a "finished" Rust project.
  2) Instead of handling user input with raw argument strings and a collection of if statements, I would probably rely on a crate like clap to define user input structs, and then use pattern matching against those structs to decide which functions to call and what operations to do.
  3) While there are messages printed to the screen that tell the user the result of their call, there is no logging or error handling beyond some print statements.
  4) There are no tests. While tests aren't strictly neccessary for a program that is 100 lines long, It is a good idea to occasionally test fallable methods, especially when handling raw user input like I do here.

I was coming from a mostly C++ background before this project, and it shows since I hardly leverage the robust type system, error handling capability, and functional aspects of the language. It is good to see that in these past moinths I have learned things about Rust, not only what crates are helpful for CLI programs, but also what approaches are idiomatic and how to leverage the unique strengths of a language like Rust. I plan to sit down and revise this project sometime soon to see how different it looks. I will update this repository when that happens.
