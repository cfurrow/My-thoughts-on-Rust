// Needed to access from_str()
use std::from_str::from_str;
use std::os::args;

fn main() {
  // in 0.6 and forward you don't supply main() with args
  let args = args();
  
  if(args.len() > 1){
    match from_str::<uint>(args[1]){
      Some(x) => println!("{}",non_rec_fib(x)), //or println(fib(x).to_str())
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
