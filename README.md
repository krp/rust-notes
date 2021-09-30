# rust-notes
Notes while learning rust

## The Rust Programming Language (book) Notes: https://doc.rust-lang.org/book/title-page.html

## Chapter 1 - Getting Started

* Piping 'rustup' directly into a shell. Trendy but also questionable.
* Weird that macros require the `!` suffix. Are users going to have to depend on memory to know when to use a macro vs function?
* Cargo for creating projects seems much nicer than pip+virtualenv. It's more like npm at this point.
* Create a new project using `cargo new hello_cargo`
* This creates a `Cargo.toml` file and a `src/` directory and initializes a `git` repository.

* What does the `edition` field in `Cargo.toml` correspond to? It defaults to "2018". Version of rust package configuration?
* Defaults to semantic versioning. Grabs your git config email address and inserts it.
* `cargo build` builds a file and places it in `target/debug` by default.
* `cargo run` automatically runs it, but it will also automatically detect other changes.
* A whole bunch of additional object files & others are created in the `target/debug` directory.

* Includes an `Info.plist` file and `Resources` directory.
* `hello_world` has large ELF-looking binary placed in `target/debug/deps/hello_cargo.dSYM/Contents/Resources/DWARF/hello_cargo` and it's **HUGE**. 961kB aka 1MB.
* What's the `.dSYM` file used for?
* Other `hello_world` binary is in `target/debug/deps` and is 309kB. Still huge.
* Contains other files in a `.fingerprint` directory. Also contains an `invoked_timestamp` file which has an `mtime` of when the build was started.

* Has a bunch of `.o` object files in `target/debug/incremental/hello_cargo-somehash/some-uuid-looking-thing`
* Has a `.lock` file in the same directory.
* Has some `dep-graph.bin`, `query-cache.bin`, and `work-products.bin` files in addition to the object files.
* `dep-graph.bin` has `RSIC 1.45.2 (d3fb005a3 2020-07-31)` header.
* `query-cache.bin` has the same but also has mention of `println` and other functions, as well as `rust_eh_personality`(?).
* `work-products.bin` has the same header but the file is tiny and doesn't have much else.

* `cargo check` does a dry-run without producing a binary. Faster than running `cargo build` to check if it builds or not.
* Maybe setup a vim script to autorun `cargo check` on file save.
* Cargo appears to search parent directories for a `Cargo.toml` file if you're in the `src` directory.
* `cargo build --release` to build a release version with optimizations.
* `cargo run` still runs the dev version. `cargo run --release` runs the release version.


## Chapter 2 - Programming a Guessing Game

* Annoying that you have to create a new string with `String::new();` instead of `""` shorthand. Is it trying to be purely functional?
* use `std::io` and `io::stdin(`. Are there explicit imports/namespacing or do you have to depend on tooling to know which functions a library provides?
* So `read_line(&mut guess)` requires a reference. Why?
* Not 100% sure on 'the prelude' and `std::io::prelude` yet. https://doc.rust-lang.org/std/prelude/index.html
* All variables (defined by `let`) are immutable without a `mut` keyword.

* `String::new` calls the *associated function* (not method) on the String *type* (not class)
* `&` works like a reference, presumably like in C and C++. Using `&mut` allows you to modify the reference.
* `.read_line` returns an `io::Result` which is an enum (has a fixed set of values). Result has variants `Ok` and `Err`.
* Ok is handled if operation was successful
* Err if it wasn't

* `.expect()` causes program to crash (fatally) and display the message you give it if an error occurs.
* Without calling `expect()` the program will run but it'll complain that you haven't handled the Result.
* println() accepts curly-braces like Python's .format strings (but not like f-strings). I'd prefer f-strings.
* Dependencies need to be manually added to Cargo.toml?
* Good that it only auto-upgrades patch versions when you run `cargo update`.

* Does Cargo.toml also get updated with dependency versions?
* Cargo fetches packages from the package registry - crates.io
* `cargo build` checks if your files have changed before recompiling, and doesn't do anything if they haven't changed
* Cargo.lock file is created (similar to yarn/npm locks) to try and ensure reproducible builds
* `use rand::Rng`. Rng is apparently a trait which defines methods. Must be in scope to use those methods. Ch10 for traits.

* `rnd::thread_rnd().gen_range(1, 101);`. thread_rnd and gen_range aren't great function names.
* `cargo doc --open` should build and open docs for what you currently have.
* `match` keyword appears to be like C's switch statement at first, but more powerful.
* Guido van Rossum has some good notes on it for an upcoming PEP (622): https://github.com/gvanrossum/patma/
* Passing a reference to `secret_number` in to `guess.cmp` seems verbose.

* Branches/cases instead a match are called **arms**
* `use std::cmp::Ordering` for access to `Ordering`
* `Ordering::Less`, `Ordering::Greater`, `Ordering::Equal` also seems like a super-verbose way to do this.
* `Ordering` is also an enum.
* Nice that they appear to support arrow functions

* Need to provide type annotation when defining `guess` as it's not able to figure it out on its own.
* `let guess: u32 = guess.trim().parse().expect("...")` all on one line looks horrible. `expect()` isn't returning a u32 is it? Behaviour is weird here.
* `let guess` rebinds the earlier `guess` of String type from above using **shadowing**. Shadowing seems like something implicit, whereas other parts of Rust are explicit.
* `loop`'s syntax is nice
* The `match` expression is interesting. Arrow functions allow appear to allow for really concise handling.

* `_` is used as a throwaway like in Python & JS. e.g. `Err(_) => continue`. Somewhat weird that `Ok(num) => num` implicitly returns `num` but `Err(_) => continue` triggers the `continue` statement.
* Switch from `expect` to `match` in order to handle errors.
* `Result` is apparently generic but it has specific subtypes, e.g. `io::Result`.
* Tried `cargo install rand` - connects to GitHub. Takes a **really** long time to update the crates.io index.
* Then received `error: specified package rand v0.7.3 has no binaries`.

* Adding to `Cargo.toml` then running `cargo build` worked. Weird that it would give an error for the other one though.
* `Cargo.lock` contains other dependencies and checksums.
* Removing it from `Cargo.toml` and rerunning `cargo build` removes it from `Cargo.lock` but doesn't say that it's removing it.
* Adding `rand = ""` installs the latest version.
* `cargo update` does nothing if you have the latest version. I wonder if it has to be continually updated for each new package that receives an update, and how frequently it has to be updated, as it took forever.

* Doesn't complain about camelCase variables.
* Calling `rand::thread_rng()` without `use rand::Rng;` gives "use of undeclared type or module "rand".
* Tried `use rand` and got "error: no `rand` external crate" while `rand` isn't installed.
* Installed it and it lets me implicitly call `rand::rand_thread` but then complains `rand::rngs::thread::ThreadRng cannot be formatted with the default formatter` when passing it to `println!()`.
* Tried `use rand::Rng` and then using `let secret_number = rand::Rng()` and got `error: not a function, tuple struct or tuple variant`.

* This kind of importing of `Rng` and using of `thread_rng` without any reference to `Rng` is confusing.
* Tried `let secret_number = rand::();` and got `expected value, found crate 'rand'`.
* Complains if I do `let secret_number = rand::random()` and needs a type hint. 
* Using `rand::random()` but the linter still complains that I'm not using `rand::Rng`. Strange.
* Tried using `let secret_number: rand::rngs::thread::ThreadRng = rand::thread_rng().random()` and it complains that `thread` is a private module. Cool that modules can have different access mechanisms.

* `thread_rng` is a Thread-local random number generator.
* `gen_range(1, 101)` behaves like Python's `range()` with the `101` being exclusive.
* `cargo doc --open` builds docs for each crate/module. Very cool. Python's needs to look better too.
* The docs generated by this look incredible.
* You can also directly browse the source from inside the browser with nice syntax highlighting. Python needs this.

* It highlights the specific function you've clicked too.
* Also previews the function declaration. This is awesome.
* `rand::distribution` docs mentions that all implementations are expected to be immutable, so you don't need to worry about thread safety.
* Some weird-ass syntax like `impl<'a, T, D: Distribution<T>> Distribution<T> for &'a D {` in rand's `mod.rs`. I guess I'll learn what these apostrophe's are later.
* `fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {` question mark prefix and `(*self).sample(rng)` are weird too. Is the `*` a pointer dereference?

* Tried `let secret_number = rand::thread_rng().gen_range()` and got `"the trait rand::distributions::uniform::SampleUniform is not implemented for std::string::String"` plus `"the trait bound std::string::String: rand::distributions::uniform::SampleUniform is not satisfied"`. All because of missing arguments to `gen_range()`.
* Wrote `let guess u32 = guess.parse()` and got `expected one of ':', ';', '=', '@', or '|'. Trying each.
* `use of undeclared typ or module Ordering` when trying to use them without `use`ing it.
* `let guess@ u32: guess.parse();` works.
* `guess.trim()` returns a string. Trims newlines.

* Just noticed that Rust doesn't (yet) have an equivalent to `input()` or a function that simultaneously prints a line and waits for input.
* Got an error when typing 123 as the input? It compiled fine then said `ParseIntError: { kind: InvalidDigit }`.
* `.parse()` required a type hint for type inference.
* Had to run with `RUST_BACKTRACE=1 cargo run` and it's got a huge backtrace which reminds me of C++ template debugging.
* Required `RUST_BACKTRACE=full` for full trace. Still no clearer.

* This is constantly panicking. No idea why right now as my code appears to be identical to the book.
* Was caused by not calling `.trim()` before calling `parse()`. **Rust should definitely provide a better error message if it finds a newline while trying to parse a number.**
* `match` syntax can be strange it seems. I guess it takes a while to get used to.
* `_` behaves like a catchall. I wonder what happens if you only specify a specific error type.
* Struggles with match keyword when trying to make `Err(_) => { ... print something and return a value }` instead.

* Also can't explicitly return `continue` from here in the event of an error. This must not have been getting triggered earlier.
* Getting another shitty confusing error. It doesn't like something about the code I've got and is saying `match` is an unexpected token, `expected one of '.', ';', '?', '}', or an operator`.
* Accidentally somehow deleted a `;` from a preceeding line and it was telling me where it should be. I'm an idiot.
* Printing `Err(x)` has `x` implicitly converted into the error string. Useful for debugging. Seems to be generic catchall for all errors. I wonder how multiple 'exception types' will be handled (as Rust doesn't appear to have Exceptions, only panic and Result)
* Combining two statements with `return continue;` gives a compiler warning: `Unreachable code` for the `return` statement and `any code following this expression is unreachable` for the continue. So it's finding and parsing the `continue` before it finds the `return` statement.


### Errors
* Try removing the `!` from println. - Suggests "use `!` to invoke the macro".
* Try removing the `mut` from `&mut guess`
* Try change the name of `guess`
* Try pass in just `read_line(guess)`. - Suggests mutably borrinwg using a reference.
* Try adding `char` type annotation to `guess` definition. - Smart enough to check types both when defined and when used later on.

* Try use the `\n` character when printing.
* Try the `print` statement instead of the `println!` macro.
* Try replace `String::new();` with a string literal - Empty string literal is &str
* Try replace it with an empty character literal - Empty character literal just says 'empty char'
* Try replace it with a character literal - expected `&mut std::string::String` but found `&mut char`

* Try put something in the curly-braces. - 'Positional arguments are zero-based' error. Kinda vague.
* `read_line` expects a `String::new()` reference (object?). Passing a mutable string-literal doesn't
* Has built-in linting for unused variables. Suggests using `_`underscore prefix for unused variables.
* Also catches when you change a variable but don't later use it.
* Suggests if a mutable variable is not mutated and tells you it doesn't need to be mutable. Very cool!

* `String::new() == ""` so it's strange that you can't use `let mut guess = "";` and pass it to `read_line`.
* `String::new("some string")` also doesn't work. This is strange behaviour for teaching beginners about strings.
* How should users know when to use `String::new` and `""`?
* Try enter a string instead of number. - Implicitly converts the input to a string.
* Try leave off `.expect()` call. - Warns about `Result` possibly being `Err` variant and it needing to be handled.

* Try leave off arguments or give `expect` an integer.



## Chapter 3 - Common Programming Concepts

### Data Types

* Mentions `mut` keyword and the error you see if you try to reassign an immutable variable.
* `const` keyword for defining constants. Can only be set to an expression (not the result of a function call). Type **must** be annotated.
* Can use `_` separator for numbers like with Python. e.g. `const MAX_POINTS: u32 = 100_000;`.
* Does Rust complain if your constants aren't ALL_CAPS? - It doesn't error but it warns. `convert: max_pOiNts to MAX_P_OI_NTS`.
* Separators also make no difference. `100_000 == 1_0_0_0_0_0`.

* Shadowing requires reusing the `let` keyword, and doesn't need the `mut` keyword. Also lets you change types. Be **very careful** when using it. Probably safest to keep variable declarations at the top of scope, like with C.
* "Rust is statically typed so it must know the types of all variables at compile time."
* Scalar types: i8/u8, i16/u16, i32/u32, i64/u64, i128/u128, isize/usize. The latter depend on architecture, e.g. 32 on 32-bit arch, 64 on 64-bit arch.
* Uses two's-complement like other languages.
* Integer literals: 98_222, 0xff, 0o77, 0b1111_0000, b'A' (Byte, u8 only).

* In debug-mode Rust checks for integer overflows at runtime. In release mode it doesn't, but it performs two's-complement wrapping.
* Float types: f32, f64.
* Operators: +, -, *, /, %
* Boolean types: bool for type annotation, true/false for values.
* Character types: 'x', supports Unicode.

* Compound types: Tuple: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
* Destructuring: `let (x, y, z) = tup;`.
* Can also use implicit types: `let tup = (500, 6.4, 1);`.
* Accessing via index: `x.0`, `x.1`, etc. I wonder how to loop through them by index?
* Array type: `let a = [1, 2, 3, 4, 5];` - Have fixed lengths. Allocates data on the stack rather than heap. Not as flexible as vector types.

* Can explicitly specify type and size: `let a: [i32; 5] = [1, 2, 3, 4, 5];`
* Can also specify default values: `let a = [3; 5];` generates `[3, 3, 3, 3, 3]`.
* Accessed via index. e.g. `a[3]`. Doesn't check for invalid index at compile-time but notices the invalid index at runtime..

### Functions

* `snake_case` for function and variable names
* Rust doesn't care where you define your functions
* What happens when you try to return from a function which you haven't specified a return type? - Says 'possibly return type missing here?'
* `statements` are instructions that perform some action that do not return a value
* `expressions` evaluate to a resulting value

* You can't assign a `let` statement to another variable. e.g. `let x = (let y = 6);`. - Says 'variable declaration using `let` is a statement'.
* Can't write `x = y = 6` in Rust.
* Can use them in curly braces if you use block-scope. e.g.
```rust
let y = {
    let x = 3;
    x + 1
};
```
* Function return types are annotated with `-> type`. e.g. `fn five() -> i32 { 5 }`
* Forgetting to return from a function gives 'expected `i32`, found `()`.


* Rustlings notes: Returning Ok(()) from a function looks weird.

### Comments

* Comments all begin with `//`. There are no multi-line or block comments. Just prefix everything with `//`


### Control Flow

* `if` expressions don't have surrounding parentheses. e.g. `if number < 5 { ... } else { ... }`
* Blocks of code associated with conditions are called **arms** in Rust.
* No *truthiness* in Rust. Conditions must evaluate to booleans. Says 'expected `bool`, found integer' otherwise.
* No implicit type conversion/coercion in conditional
* Can use `if/else` in a `let` statement. e.g. `let number = if condition { 5 } else { 6 };`.

* Values returned by different arms must all be the same type. - Get 'expected integr, found `&str` otherwise.
* Rust has 3 loop types. `loop`, `while`, and `for`.
* `loop` runs until you explicitly tell it to stop
* The `break` keyword supports returning values from a loop. e.g. `let result = loop { ... }` then `break counter * 2` inside the loop. Pretty cool!
* `while` loops work as expected. e.g. `while number != 0 { ... }`.

* `for` loops require creating an iterator. e.g. `let a = [1, 2, 3];` then `for element in a.iter() { ... }`.
* Sequences and reverse loops can be done with `for number in (1..4).rev() { ... }`


### Exercises
* Convert temperatue between Fahrenheit and Celsius
* Generate the nth Fibonacci number
* Print the lyrrics to the Christmas carol song "The Twelve Days of Christmas", taking advantage of the repetition of the song.



## Chapter 4 - Ownership


### 4.01 - Ownership

* `s.push_str(", world!"); // push_str() appends a literal to a String`. Why push_str? Can't it infer a string from the argument?

* Rust has 2 string types. String for heap-allocated strings and str for both literals and slices.

* Use String::from to create a String from a string literal.

* :: operator is used for namespacing.

* Copy trait can be used only if the Drop trait hasn't been used.


### 4.02 - References and Borrowing

* & for references

* 

Copy vs Clone types - https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone

Named arguments don't work for constructors? e.g. Point::new(x: 10, y: 10);

### Notes from Rustlings

* Move semantics 2
1st way: vec0.clone();, 2nd way: borrow (using &) then make clone and return new vec, 3rd way: pass in &mut vec0 to fill_vec and change args to make it a mutable reference. Make vec2 mutable, remove return value and change function signature, then finally make vec1 a clone of vec0 after filling vec0.

* Primitive types 2:
is_numeric() only exists on characters, not on strings.

* Stupid: Parsing a float in Rust: s = "3.14"; s.trim().parse::<f64>().is_ok()



## Chapter 5 - Structs
    
    
Notes from Rustlings: 3 struct types: C, tuple, and unit(? no docs on this, seems to just be like the empty set?).



## Chapter 6 - Enums and Pattern Matching
    
Rustlings notes: Enums are defined without using strings or defining names.
Two different syntaxes for defining enum types. struct-like with argument names & curly braces, and function-like without.
    


## Chapter 7 - Packages, Crates & Modules

## Chapter 8 - Common Collections

## Chapter 9 - Error Handling
    
## Chapter 10 - Generic Types, Traits, and Lifetimes
    
* Rustlings notes: AppendBar implementation uses self + "Bar". Isn't "Bar" a CString/str? If so shouldn't the compiler complain when trying to append it? self + String::from("Bar") fails yet the function signature returns a String and not an str.

* Kind of cool that traits can add methods to built-in types.
* Coherence/Orphan Rule (ch10.02) prevents overriding stdlib methods that are out of scope.
    
* Trait Bounds: Can a method not call methods on another type without specifying the traits they implement? e.g. with notify() in chapter 2.
    
* Multiple trait bounds using the *where* keyword.
    
   
## Chapter 11 - Writing Automated Tests
    
## Chapter 12 - An I/O Project: Building a Command Line Program
    
## Chapter 13 - Functional Language Features: Iterators and Closures

* Rustlings notes: *i *= 2. Interesting syntax for iter_mut in collections/vec2.rs.

## Chapter 14 - More about Cargo and Crates.io
   
## Chapter 15 - Smart Pointers
    
## Chapter 16 - Fearless Concurrency
    
## Chapter 17 - Object Oriented Programming Features of Rust

## Chapter 18 - Patterns and Matching
    
## Chapter 19 - Advanced Features
    
## Chapter 20 - Final Project: Building a Multithreaded Web Server
    
## Chapter 21 - Appendix
