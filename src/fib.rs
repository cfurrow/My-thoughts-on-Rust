import io::println;

// Some examples show that you can use "args: ~[~str]"
//   but I got nothing but errors with that syntax.
fn main(args:~[~str]){
  // Having to define vars as muteable will get old quick.
  let mut i    = 0;
  let mut max  = 10;
  let mut sum  = 0;
  let mut last = 0;
  let mut curr = 1;
  // Cannot check against just "1" because type of .len() is unsigned
  if( args.len() > 1u){

    max = int::from_str(args[1]).get();
  }

  while(i < max){
   // println cannot directly print an int, so must cast to str
   println(int::str(sum));
   sum  = last + curr;
   last = curr;
   curr = sum;

   // no i++ unary operator
   i += 1;
  }
}
