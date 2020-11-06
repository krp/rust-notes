# rust-notes
Notes while learning rust

## The Rust Programming Language (book) Notes: https://doc.rust-lang.org/book/title-page.html

## Chapter 1 - Getting Started

* Piping 'rustup' directly into a shell. Trendy but also questionable.
* Weird that macros require the `!` suffix. Are users going to have to depend on memory to know when to use a macro vs function?
* Cargo for creating projects seems much nicer than pip+virtualenv. It's more like npm at this point.


## Chapter 2 - Programming a Guessing Game

* Annoying that you have to create a new string with `String::new();` instead of `""` shorthand. Is it trying to be purely functional?
* use `std::io` and `io::stdin(`. Are there explicit imports/namespacing or do you have to depend on tooling to know which functions a library provides?
* So `read_line(&mut guess)` requires a reference. Why?
* Not 100% sure on 'the prelude' and `std::io::prelude` yet. https://doc.rust-lang.org/std/prelude/index.html
* All variables (defined by `let`) are immutable without a `mut` keyword.

* `String::new` calls the *associated function* (not method) on the String *type* (not class)
* `&` works like a reference, presumably like in C and C++
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
