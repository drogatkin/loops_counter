# A performance language selection

## Preface

I usually do not look in performance numbers because I know that a performance of the final product will depend on a programmer skills and also app surroundings.
However, a friend of mine read a book about Rust and asked me to run some test [case](https://github.com/drogatkin/loops_counter/blob/master/rust/counter.rs). He has 
a relatively fast machine, but mine is 8 years old. However when I run the program, surprisingly I got numbers not very far from my friends numbers.

> RustBee (rb) v 1.08.04-nightly © 2024 D. Rogatkin
> Compiling counter ...
> Would you like to run count? [Y|n]  
> Hello, time!
> count: 42,493,431

One of theory why my numbers are similar to his that *Instant::now()* is actually an OS level call and slows down execution speed. So we decided to eliminate 
the call replacing it by a version with a [sleeping thread](https://github.com/drogatkin/loops_counter/blob/master/rust/counter3.rs).

> RustBee (rb) v 1.08.04-nightly © 2024 D. Rogatkin
> Would you like to run count? [Y|n]  
> Hello, time!
> SystemTime { tv_sec: 1715216904, tv_nsec: 814051492 }
> SystemTime { tv_sec: 1715216905, tv_nsec: 814528333 }
> count: 3,219,570,640

As you can see, the performance number got improved mostly in hundred times.This test pushed curiosity to check other popular languages as C++ and Java.
First rewriting the program in C++ issued an infinity loop. Sure, you can't access a shared copy of data without a special construction, as AtomicBool in Rust. However,  
nothing prevents you doing so in C++. Adding volatile memory access qualifier solved the problem. The C++ code you can see [there](https://github.com/drogatkin/loops_counter/blob/master/C%2B%2B/counter4.cpp).

> Compile and run....
> Elapsed: 1000
> count: 3,219,722,309

As you can see, C+++ provides a similar performance to Rust. The tests were executed on a Linux machine equipped with Intel i7 of 7th generation.
Remembering one billion rows challenge, I decided to test Java too. There is also no forcing for different threads to access shared data using synchronized, atomic or volatile data. However having C++ experience,
volatile was added. Java [code](https://github.com/drogatkin/loops_counter/blob/master/java/code/Counter.java) showed the following numbers:

Java 21

> Running...
> start: 1715218272720
>   end: 1715218273721
> count: 2,152,594,501

Java 8

> Running...
> start: 1715218279747
>   end: 1715218280747
> count: 2,652,172,171

As you can see, Java appeared 30% slower than compiled languages. Rust used *opt-level=3* compilation option and C++ - *O3*.
You can also see that the latest Java is a bit slower than Java 8.


