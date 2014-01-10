// Needed to access from_str()
use std::from_str::from_str;
use std::os::args;

fn main() {
  // in 0.6 and forward you don't supply main() with args
  let args = args();
  
  if(args.len() > 1){
    match from_str::<uint>(args[1]){
      Some(x) => println!("{}",fib(x)), //or println(fib(x).to_str())
      None    => fail!("Argument supplied is not a positive number")
    };
  } else {
    fail!("You must provide a number as an argument")
  }
}

fn fib(n: uint) -> uint{
  match n {
    1 => 1,
    2 => 1,
    _ => fib(n-1) + fib(n-2)
  }
}


