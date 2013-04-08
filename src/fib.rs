// Wow, what? I need this line just to call from_str ? Ugh.
use core::from_str::FromStr::from_str;

pub fn main() {
  // in 0.6 you don't supply main() with args
  let args = os::args();
  // Having to define vars as muteable will get old quick.
  let mut i    = 0;
  let mut max  = 10;
  let mut sum  = 0;
  let mut last = 0;
  let mut curr = 1;
  // Cannot check against just "1" because type of .len() is unsigned
  if( args.len() > 1u){
    // to get an int from a string:
    max = from_str::<int>(args[1]).get();
  }

  while(i < max){
   // println cannot directly print an int, so must cast to str
   println(int::to_str(sum));
   sum  = last + curr;
   last = curr;
   curr = sum;

   // no i++ unary operator
   i += 1;
  }
}
