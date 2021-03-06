#My (uneducated) thoughts on Mozilla's Rust language

As I was poking around github, I wondered onto [Mozilla's repos](http://github.com/mozilla). It was at that time I was reminded about [Rust](http://github.com/mozilla/rust), their programming language.

I hadn't given it a go yet, so I thought I'd try my hand with printing out a [Fibonacci sequence](http://en.wikipedia.org/wiki/Fibonacci_number). Figured I'd then move on to a [FizzBuzz](http://c2.com/cgi/wiki?FizzBuzzTest).

After writing fib.rs, I had had enough for now. It was a bad experience for me. I read over their [tutorials](http://dl.rust-lang.org/doc/tutorial.html#introduction) but it was pretty lacking on some basics I needed:

- how to structure a main()
- how to convert a string to an int

After having a bad time with poorly written documentation, I found that to get an int from a str, you need to do this:

     use std::from_str::from_str;

     // This is verbose, but there is also a lot going on:
     // -> The type of some_int in constrained to int (could also be written : let some_int: int = match from_str(some_str) {...})
     //    from_str applies on generic types, which means you dont have to know a function/method per type, but it also means you have
     //    to provide Rust's compiler with a hint to guess which type you want the result to be.
     //
     // -> from_str() returns an Option, which is an enumerated type, which can either be Some(x) (with x the contained value), or None.
     //    This allows for simple, and enforced error-checking. You can bypass it by calling unwrap() on the Option, which will fail if the
     //    Option is None. Of course, this is strongly discouraged. Using this pattern forces you to think about what you want to do in case
     //    of failure. In general, using Options is the Rust way of dealing with function/method calls that may fail.
     let some_int = match from_str::<int>(some_str) {
         Some(x) => x,
         None    => fail!("The string doesn't contain an int")
     }

Additionally, all local variables are immutable by default, and in order to make them mutable, you have to add three more letters to your declaration:

    let some_immutable_int: int = 10;
    let mut some_mutable_int: int = 42;

This is a design decision by the Rust team, to enable easy message passing between tasks, but in contexts where you NEED mutable variables, it sucks. And this does not work (has to be mutable):

    let some_immutable_int: int;
    // .. some code
    if( condition ){
      some_immutable_int = 10;
    }
    else{
      some_immutable_int = 42;
    }
    
However, this works

      let some_immutable_int = if(condition){
            10
      else {
            42
      };
      
So we need to re-think how to do some common tasks.
    
## It's not a big deal.
I wasn't planning on getting deep-into Rust to develop my next app, but I was curious how their language-design decisions affected my own developer inclinations. In the end, I just don't like Rust. Maybe that will change over time as they change the language, or as I find a need for it. But since it's so close to C/C++, I may just run with them and worry about null-pointers myself (that's one of Rust's core-features, no null or while pointers).
    
