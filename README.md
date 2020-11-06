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
* `cargo run` runs it.
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


* 
