#My (uneducated) thoughts on Mozilla's Rust language

As I was poking around github, I wondered onto [Mozilla's repos](http://github.com/mozilla). It was at that time I was reminded about [Rust](http://github.com/mozilla/rust), their programming language.

I hadn't given it a go yet, so I thought I'd try my hand with printing out a [Fibonacci sequence](http://en.wikipedia.org/wiki/Fibonacci_number). Figured I'd then move on to a [FizzBuzz](http://c2.com/cgi/wiki?FizzBuzzTest).

After writing fib.rs, I had had enough for now. It was a bad experience for me. I read over their [tutorials](http://dl.rust-lang.org/doc/tutorial.html#introduction) but it was pretty lacking on some basics I needed:

- how to structure a main()
- how to convert a string to an int

I had to dive into the source of Rust, which is meta-written in Rust, to determine what methods were available to me for int, str, etc. That's where I found that to get an int from a str, you need to do this:

      some_int = from_str::<int>(some_str).get();
      
Verbose!

Additionally, all local variables are immutable by default, and in order to make them mutable, you have to add three more letters to your declaration:

    let some_immutable_int: int = 10;
    let mut some_mutable_int: int = 42;

That got old fast. And this does not work (has to be mutable):

    let some_immutable_int: int;
    // .. some code
    if( condition ){
      some_immutable_int = 10;
    }
    else{
      some_immutable_int = 42;
    }
    
## It's not a big deal.
I wasn't planning on getting deep-into Rust to develop my next app, but I was curious how their language-design decisions affected my own developer inclinations. In the end, I just don't like Rust. Maybe that will change over time as they change the language, or as I find a need for it. But since it's so close to C/C++, I may just run with them and worry about null-pointers myself (that's one of Rust's core-features, no null or while pointers).
    
