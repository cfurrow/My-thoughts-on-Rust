import io::println;

// Some examples show that you can use "args: ~[~str]"
//   but I got nothing but errors with that syntax.
fn main(args:[str]){
  // Having to define vars as muteable will get old quick.
  let mut i: int = 0;
  let mut max: int = 10;
  let mut sum: int = 0;
  let mut last: int = 0;
  let mut curr: int = 1;
  // Cannot check against just "1" because type of .len() is unsigned
  if( args.len() > 1u){
    // Well this is weird.
    //  int::from_str returns core::option::option<int>
    //  to get the value, have to use options::get() 
    max = option::get(int::from_str(args[1]));
  }

  while(i < max){
   // println cannot directly print an int, so must cast to str
   println(int::str(sum));
   sum = last + curr;
   last = curr;
   curr = sum;

   // no i++ unary operator
   i += 1;
  }
}
