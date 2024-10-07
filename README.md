# Rust-ToDo
Rust CLI ToDo list project

Originlly adapted from the guide at https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/ although they two now share very little in common...

Most recent update: I decided to se what I could do if I spent an hour or two refactoring this old project. Here are the main points:

  1. Error handling was something that was sorely lacking from the original program. It was littered with panics and expects, which are not neccesarily wrong, but could have potentially left the program in a more vulnerable state had an error occurred as copared to using error propogation with some toollike anyhow. This is a very lazy implementation of anyhow because this project is small and there just wasn't any need for 80% or the project's code to be error handling. Nevertheless the more frequent use of matching and returning detailed errors to propogate back to the control function is my preffered form of error handling for all projects.
     
  2. Input parsing was just done with variable and the .expect() keyword, which works, but is not very robust and does not handle errors well. Replacing that scheme with a proper one like clap makes the program much more ergonomic to work with. Especially the ability to include not always needed arguments like how a description isn't needed when using the print function.
  
  3. The control flow was handled with a jumble of if else statements instead of being inside of a dedicated function (Which wasn't strictly neccessary but is way more ergonomic for programs as they scale up) that uses pattern matching and error propogation to sucinctly call self contained functions. This works a whole lot better than defining whole processes within an if else block.
  
  4. Implementing the display trait for my custom struct (which was just a lazy hashmap with some automated methods) instead of doing some sort of jumbled if else printing nonsense. This was one thing I had no idea even existed bback then, the trait system. It is nice to be able to define how to pring your own data structures instead of relying on debug or some custom rolled method that only works outside of println! and it's alternatives. Since my struct is just a hashmap with helper methods it didn't really matter how I did it, but as programs scale up this becomes a more and more useful feature.
  
  5. The removal or excess code and code that was not very idomatic was also neccessary to make this program look much better and feel much better to work with. There were inconsistencies surrounding when to borrow or when to consume the input variable for a function without much thought given, and each function had it's own rules when it came to error handling and return types. This is not inherently a bad thing, but when these factors can be normalized across a whole project without much fuss it is really nice. In addition to that making sure that the code added is much closer to the Rust style guide interpretation ensures that I am not the only one who can possibly read this program.

In the end this program is so small in scope that none of these changes were neccessary, and truthfully I probably spent more time on this little exercisse than it is worth, but it is satisfying to see my code evolve to be more idiomatic, with more robust features and a more future conscious focus when building new projects.
