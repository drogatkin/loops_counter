# A performance language selection

## Preface

I usually do not look in performance numbers because I know that a performance of the final product will depend on a programmer skills and also app surroundings.
However, a friend of mine read a book about Rust and asked me to run some test [case](https://github.com/drogatkin/loops_counter/blob/master/rust/counter.rs). He has 
a relatively fast machine, but mine is 8 years old. However when I run the program, surprisingly I got numbers not very far from my friends numbers.

```console
RustBee (rb) v 1.08.04-nightly © 2024 D. Rogatkin
Compiling counter ...
Would you like to run count? [Y|n]  
Hello, time!
count: 42,493,431
```

One of theory why my numbers are similar to his that *Instant::now()* is actually an OS level call and slows down execution speed. So we decided to eliminate 
the call replacing it by a version with a [sleeping thread](https://github.com/drogatkin/loops_counter/blob/master/rust/counter3.rs).

```console
RustBee (rb) v 1.08.04-nightly © 2024 D. Rogatkin
Would you like to run count? [Y|n]  
Hello, time!
SystemTime { tv_sec: 1715216904, tv_nsec: 814051492 }
SystemTime { tv_sec: 1715216905, tv_nsec: 814528333 }
count: 3,219,570,640
```

As you can see, the performance number got improved mostly in hundred times. This test pushed curiosity to check other popular languages as C++ and Java.
First rewriting the program in C++ issued an infinity loop. Sure, you can't access a shared copy of data without a special construction, as AtomicBool, or a Mutex in Rust. However,  
nothing prevents you doing so in C++. Adding a *volatile* memory access qualifier solved the problem. The C++ code you can see [there](https://github.com/drogatkin/loops_counter/blob/master/C%2B%2B/counter4.cpp).

```console
Compile and run....
Elapsed: 1000
count: 3,219,722,309
```

As you can see, C+++ provides a similar performance to Rust. 
Remembering one billion rows challenge, I decided to test Java too. There is also no forcing for different threads to access shared data using synchronized, atomic or volatile data. However a
*volatile* was added keeping in mind  C++ experience. Java [code](https://github.com/drogatkin/loops_counter/blob/master/java/code/Counter.java) showed the following numbers:

**Java 21**

```console
Running...
start: 1715218272720
 end: 1715218273721
count: 2,152,594,501
```

**Java 8**

```console
Running...
start: 1715218279747
 end: 1715218280747
count: 2,652,172,171
```

As you can see, Java appeared 30% slower than the compiled languages. Rust used *opt-level=3* compilation option, and C++ - *O3*.
You can also see that the latest Java is a bit slower than Java 8.

The above test showed mostly effectiveness of data sharing between threads in the popular languages. But what about a single thread performance? 
I selected a very simple calculation [task](https://github.com/drogatkin/loops_counter/blob/master/C%2B%2B/perfect.cpp) in C++.

```console
$ ./r perfect.cpp
Compile and run....
num: 8128/ sum:  8128
num: 496/ sum:  496
num: 28/ sum:  28
num: 6/ sum:  6

real	0m0.521s
user	0m0.514s
sys	0m0.004s
```


The test case was replicated in [Rust](https://github.com/drogatkin/loops_counter/blob/master/rust/perfect.rs) and [Java]( https://github.com/drogatkin/loops_counter/blob/master/java/code/Perfect.java).
Performance numbers appeared very similar with slight advantage of C++ over competitors.

Finally, I decided to check an effectiveness working with a memory allocation. The test will also challenge Java garbage collector. The test didn't bring
much surprises, but showed that Java starting losing more to competitors on memory operations. The source for the test is [Java](https://github.com/drogatkin/loops_counter/blob/master/java/code/StrTest.java),
 [Rust](https://github.com/drogatkin/loops_counter/blob/master/rust/str_test.rs), and [C++](https://github.com/drogatkin/loops_counter/blob/master/C%2B%2B/str_test.cpp).

## Some recap

The table below shows testing results for different programming languages :

| Rust | C++ | Java 8 | Java | Test # | In |
| :---------- | :------: | ----: | -------: | :----: |  :-------- |
| 1.78.0 | 11.4.0 | 1.8.0_401 | 21.0.3 |  |   |
| 3,222,942,950 | 3,232,063,904 | 2,586,544,161 | 2,157,080,054 | 1 | times |
| 0.601 | 0.521 | 0.608 | 0.620 | 2 | sec |
| 34.456 |  35.040 |  06:46.275 |   06:14.441 |  3 |  min:sec.ms |

All test were conducted on Intel i7 gen 7th processor, and OS Ubuntu 22.04. A test number was calculated as:

An average number of ten runs om the machine without any additional load. 

You can see that C++ keeps a leadership in all tests, however a position of Rust only sightly behind. So my view is the following:

1. A fast development with a good performance of a result product - Java will be the best choice
2. Dependable programming with outstanding performance, but slow in a development - Rust will be no mistake
3. Ultra fast performance with a relatively fast development, but not very reliable - C++ is your pick



