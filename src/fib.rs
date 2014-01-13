// Needed to access from_str()
use std::from_str::from_str;
use std::os::args;

fn main() {
  // in 0.6 and forward you don't supply main() with args
  let args = args();
  
  if(args.len() > 1){
    match from_str::<uint>(args[1]){
      // println!() is a Rust macro which allows for formatting comparable to C's printf, with the added benefit of compile-time type-checking.
      // If I tried println!("{:s}",non_rec_fib(x)) (note the :s) for example, this program wouldn't compile, as non_rec_fib doesn't return a string.
      // Furthermore, the type-checking is done on Traits, not on bare types, which means that if you create a class that implement the correct trait,
      // you'll be able to use println! as is.
      // For the complete doc on how to write format strings in Rust, see http://static.rust-lang.org/doc/master/std/fmt/index.html 
      Some(x) => println!("{:u}",non_rec_fib(x)),
      None    => fail!("Argument supplied is not a positive number")
    };
  } else {
    fail!("You must provide a number as an argument")
  }
}

// This is a good way to show-off Rust's pattern matching capabilities, but it has very poor performances
fn fib(n: uint) -> uint{
  match n {
    1 => 1,
    2 => 1,
    _ => fib(n-1) + fib(n-2)
  }
}

fn non_rec_fib(n: uint) -> uint{
  let mut previous = 1;
  let mut current = 1;

  for i in range(3, n+1) {
    let temp = current;
    current += previous;
    previous = temp; 
  }

  current
}
