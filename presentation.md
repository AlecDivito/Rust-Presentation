---
marp: true
author: Alec Di Vito <alecdivito>
footer: Consider Rust
theme: gaia
transition: fade
---

<!-- paginate: skip -->

# Bring Rust to your organization

February 2024

<!--
Introduce rust and purpose of talk
- This is not me telling you to use rust
- This is me introducing you to why you should consider rust
- Why is rust worth your thinking
- What you should know about
-->

---

<!-- paginate: true -->

# Who am I?

- **Alec Di Vito** 
- **Job**: ~~Senior~~ Staff Cloud Developer
- **Experience in:** Python, Go, Javascript, Java and Jenkinfiles
- **Uses Rust:** Only in free time sadly 😭

Get in Touch

- Slack
- Email

---

## Goal for today


#### Get you From

> I already know python and it's fast enough
> `Current RBC Dev`

#### To

> I wish my code was "blazing fast" and I'll use Rust to get there
> `Future RBC Dev`

---

## What we'll cover today

- High level comparison to Python, Java and Go
- Rust's Major features
- Rust's Drawbacks
- Long-Term viability

## What we won't be doing

- Learning programming in Rust

---

## Why Rust

- Powerful type system
- Algebraic data types (Data types that can be added together to create strict)
- Safe concurrency
- Good support for meta-programming
- Strong package management system
- Built-in testing, document generation and benchmarking
- Good `async`/`await` support
- Portable between different compilation targets

<!--
Modern Language (nice to use)
- Nice Generics
- Algebraic data types + patterns
- Modern tooling


-->

---

## Powerful type system

```rust
fn show<D: std::fmt::Display>(d: D) {
    println!("{}", d)
}

fn main() {
    show("Example")
    show(32)
}
```

```bash
> cargo run
Example
32
```

---

## Powerful type system

```rust
struct Example {
    message: String
}

fn show<D: std::fmt::Display>(d: D) {
    println!("{}", d)
}

fn main() {
    show(Example { message: String::from("hello world")})
}
```

---

## Powerful type system
```zsh
➜  example git:(main) ✗ cargo build
   Compiling example v0.1.0 (/Users/divitoa/code/alecdivito/Rust-Presentation/example)
error[E0277]: `Example<'_>` doesn't implement `std::fmt::Display`
  --> src/main.rs:10:10
   |
10 |       show(Example {
   |  _____----_^
   | |     |
   | |     required by a bound introduced by this call
11 | |         message: "hello world",
12 | |     })
   | |_____^ `Example<'_>` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Example<'_>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `show`
  --> src/main.rs:5:12
   |
5  | fn show<D: std::fmt::Display>(d: D) {
   |            ^^^^^^^^^^^^^^^^^ required by this bound in `show`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `example` (bin "example") due to previous error
```

---

## Powerful type system

```rust
trait Find<P, T>
where
    P: Fn(&T) -> bool,
{
    fn new_find_func(&self, predicate: P) -> Option<&T>;
}

impl<P, T> Find<P, T> for Vec<T>
where
    P: Fn(&T) -> bool,
{
    fn new_find_func(&self, predicate: P) -> Option<&T> {
        self.iter().filter(|p| predicate(*p)).take(1).next()
    }
}
```

---

## Powerful type system

```rust
fn main() {
    let list = vec![1, 2, 3, 4, 5];
    println!("{:?}", list.new_find_func(|i| *i > 3));
    println!("{:?}", list.new_find_func(|i| *i == 10));
}
```

```zsh
➜  example git:(main) ✗ cargo run
   Compiling example v0.1.0 (/Users/divitoa/code/alecdivito/Rust-Presentation/example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/example`
Some(4)
None
```

---

## Algebraic data types

- Types that can be added together
- Can be matched on

```rust
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

---

## Algebraic data types

```rust
struct Version {
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.major, self.minor, self.patch) {
            (Some(major), Some(minor), Some(patch)) => write!(f, "{}.{}.{}", major, minor, patch),
            (Some(major), Some(minor), None) => write!(f, "{}.{}", major, minor),
            (Some(major), None, None) => write!(f, "{}", major),
            (None, _, _) | (Some(_), None, _) => unreachable!(),
        }
    }
}
```

---

## Error Handling

- panic
- Option & unwrap
- Result
- Error Types

---

## Life Times

- This is a part of the borrow checker
- Means that we can read the value

---

## Mutability

- Don't allow the same value to be used twice
- The following fails to compile

```rust
fn main() {
    let a = vec![10];
    let b = a;
    println!("{}", a);
}
```

```zsh
➜  example git:(main) ✗ cargo run
   Compiling example v0.1.0 (/Users/divitoa/code/alecdivito/Rust-Presentation/example)
warning: unused variable: `b`
 --> src/main.rs:6:9
  |
6 |     let b = a;
  |         ^ help: if this is intentional, prefix it with an underscore: `_b`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0382]: borrow of moved value: `a`
 --> src/main.rs:7:22
  |
5 |     let a = vec![10];
  |         - move occurs because `a` has type `Vec<i32>`, which does not implement the `Copy` trait
6 |     let b = a;
  |             - value moved here
7 |     println!("{:?}", a);
  |                      ^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
6 |     let b = a.clone();
  |              ++++++++
```

---

## Mutable Lifetimes

- function with mutable lifetime
- everything must be declared mutable
- immunitable by default

---

## Pointers

- Box
- Rc
- Arc

---

## Thread Safety

```rust
    let a = Rc::new(10);
    let b = a;

    std::thread::spawn(move || {
        println!("{}", b);
    });
```

```
➜  example git:(main) ✗ cargo run
   Compiling example v0.1.0 (/Users/divitoa/code/alecdivito/Rust-Presentation/example)
error[E0277]: `Rc<i32>` cannot be sent between threads safely
   --> src/main.rs:10:24
    |
10  |       std::thread::spawn(move || {
    |       ------------------ ^------
    |       |                  |
    |  _____|__________________within this `[closure@src/main.rs:10:24: 10:31]`
    | |     |
    | |     required by a bound introduced by this call
11  | |         println!("{}", b);
12  | |     });
    | |_____^ `Rc<i32>` cannot be sent between threads safely
```

---

## Thread safety

- Using `Arc` is safe
- Using `Rc` in multiple threaded code is unsafe

```rust
    let a = Arc::new(10);
    let b = a;

    std::thread::spawn(move || {
        println!("{}", b);
    });
```

- This is enforced by the compiler

---

## Unsafe Operations

- Unsafe operations

---

## Testing

- unit testing
- integeration testing
- Documentation testing

---

## Documentation Generation

- Generated sites
- Documentation
- Hosted for free

---

## Macros

- Quickly write out repeatable code
- Rust like syntax

---

## Procedural Macros

- Harder to write but can create more complicated code
- Able to iterate over a stream of tokens and have complicated logic

---

## Cargo

- Build tool for Rust
- Package manager

---

## Cargo.toml

- Manifest file for Rust
- Track package and project settings
- Track dependancy and verions
  - Download from git and cargo.io

---

## Build targets

- Quickly and easily target different compilers and architectures
- Easy compile to wasm
- Tools that generate bindings automatically for 
  - Web Assembly and Javascript
  - C/C++ Header files
  - Python

---

## Build Scripts

- For building more complicated projects

---

## Async / Await support

- Multiple runtimes to choose from
- Zero-cost abstractions state machines

---

## Overall result

reduced tail latency because no more GC calls

---

# Rust is better then Python because

- Its much faster
- And much more memory efficient
- Pattern matching
- Strict Static Typing

<!--
Some i didn't cover
- Algebraic data types
- Many fewer runtime
-->

> The amount of memory [pandas] needs to finish an operation ... 10 times the RAM size or 10 times data set size, and for polar this has been lower.

<!--

no interpreter
no runtime
easier to do multi threading
pattern matching, using types as a strength
running for a while then checking

GIL - Global Interpreture lock
 - locks variable value in a thread at a time

-->

---

# Rust is better then Java because

- No GVM overhead or pauses
- Lower memory use
- Safer concurrency <!-- data races -->
- Pattern matching
- Power type system
- Single build system

<!-- 
- No JVM overhead or GC pauses
- Much lower memory use
- No byte code
- Zero cost abstractions
  - Often times java includes metadata on the object of interfaces that are
    attached to the object at runtime. These abstractions add bloat to your code
  - This doesn't happen in rust because all of the abstractions like interfaces
    are compiled away into the binary
- Data Races (Safer concurrency)
  - Rust at compile time can validate that there are no data races and so these
  - concerns can go away
- Unified build system means there is only one
-->

---

# Rust is better then C/C++ because

- it won't Segfaults
- it can't use after free
- no null pointers
- more powerful type system
- has no complicated build steps
- package management system
- builds a static binary

---

# Rust is better the Jenkinfiles

- Powerful type system
- Ability to run on a local machine
- Better package management

---

# Rust is better then Go because

- No GC pauses
- lower memory use
- No `nil` pointers
- No fine grain error checking (nicer error checking)
- Concurrency bugs are easy to run into
- Easy to put stuff on the heap
- more powerful type system
- Zero-cost abstractions


---



# Why I love Rust

- nice and efficent generics
- Algebraic data types + patterns
- Modern tooling
- Compiler errors

---

```rust

trait Find<P> {
    find(p: P) -> Option<&Self>
}

impl<P> 


```

---

## Rust documentation

```rust
/// Returns one more than it's argument
///
/// ```
/// assert_eq!(one_more(42), 43)
/// ```
fn one_more(num: i32) -> i32 {
    num + 1
}
```

---

## Safety by construction

- Checking pointers at compile time
  - Every value in your program has a single owner
  - They are required for freeing that memory
  - There is only one owner of data
  - Can't have use after free, not possible
  - This is what is known as the borrow checker

--- 
- Mutability
  - Everything is immutable by default
  - This extends to methods
  - Even code that owns the value can't do this

---


## Thread Safety
- Rust types know when it's safe to cross thread boundaries
- Rc and Arc are pointer types (smart pointers in C++)
  - Rc uses normal integer to track pointers
  - Arc uses atomic integer
  - These pointers are for thread safety
- There can only be 1 writer or many readers
  - Can't have 2 mutable writers

---


## No Hidden states
- use the type system to check every state
- `Option` and `Result` type
- Any function that can fail will return one of these things
- You don't need to remember to put in null checks
- You are forced to deal with it
- Question mark syntax surger
- Really pleasant to handle errors

---

## Low level control

- No GC or Runtime
  - No grabage collection
  - No memory overhead
  - Can issue system calls
  - Can run on systems without an OS
  - Free FFI calls to other languages
- Allocation and dispatch
  - Opt into heap allocation using `Box`
  - Opt into dynamic dispatch (vtable) using `dyn Trait`
  - You can change your global allocator (for example, in Web assembly)
- Low level code
  - Using the unsafe key word
  - Escape hatch from using normal rust code
  - You know that the unsafe code at rest
  - just search for the keyword

Overall, this allows you to be compatible with other languages

- Zero-overhead FFI
  - Bindings to C languages  (bindgen, C-bindgen)
- WebAssembly support (wasm-bindgen)
  - Like the JVM but for browsers
  - Rust program and export as a npm package
- Works with traditional tools
  - Just uses LLVM so
    - perf
    - Security checking
    - Rust binaries == C++ binaries

<!--
In this example, we should spend more time talking about webassembly
Any language, low level native C-ABI that rust plugs into
-->

---

## Tooling

- Dependency management
  - Example: Showing cargo.toml file
    - Version and git dependency
  - Knows about versioning
  - Write build tools in Rust using a build.rs file
  - Can change the types of builds you want
- Standard tools includes
  - Shipped with Rust (developed within)
  - Cargo fmt
  - Cargo docs
  - Cargo clippy
  - rust-analyzer
  - When you upload a library, it will generate the documentation and create a public site.
  - Documentation experience is extremely good.
- Excellent macro support
  - Rust supports writing repetitive Rust code
  - Rust validates that the generated code is valid rust at compile time
  - Rust also has procedural macros
    - Write Rust code with Rust code
    - Talk about serde
    - Give me an implementation of serialize and deserialization
    - Take the types and develop your own serializing library
    - `derive` procedural macro
- Tracing?

---

## Asynchronous code

Don't block your executing code.
`async`/`await`

- Language support for writing async code
- Choose your own runtime!
  - Don't need to use the default runtime
    - Node
    - Python
    - Go
  - Were now going to be getting async traits soon

---

## Drawbacks

- Learning curve
  - The borrow checker is different
  - No object-oriented programming
    - used to thinking about class objects
    - interfaces act differently then traits
    - These decisions are made for good reasons
  - Spend more time in programming so that it works when it compiles

- Ecosystem
  - Smaller group
  - Integration with enterprise clients
  - Small group of people created the most popular package

- No Runtime
  - Has no reflection
  - Don't get runtime assisted debugging

- The Rust organization
  - Can't use the Rustatation, even though they didn't make it

- Slow-ish compile times
  - Large projects can compile slowly (+10-20 seconds)
  - No pre built libraries
  - However, this is changing

- Sometimes working with vendored packages can be a pain
  - You can write the glue, but it's not always the best

---

## Long-term viability

- Been the mosted loved language for years (stackoverflow)
  - Rust is the most admired language, more than 80% of developers that use it want to use it again next year. Compare this to the least admired language: MATLAB. Less than 20% of developers who used this language want to use it again next year.
  - https://survey.stackoverflow.co/2023/
  - Most admired language https://survey.stackoverflow.co/2023/#programming-scripting-and-markup-languages
- Adoption by big companies
  - Mozilla, Dropbox, CloudFlare, Microsoft, Google, Amazing, Facebook, Atlassian, NPM, deno, Next.js
- Great interoperability storey; easy incremental adoption
- Increased company involvement in Rust
- ~10 yearly conferences around the world

---

<!-- Rust code example finding an element in Vec<T>  -->

---

# Who is supporting Rust Developer?

1. Vice President OS Security and Enterprise at Microsoft
   > [$10 million to make it 1st class language in our engineering systems](https://twitter.com/dwizzzleMSFT/status/1720134540822520268)
   > - Nov 2, 2023
2. Niko Matsakis - Rust Programming language Tech lead
   > [How our AWS Rust team will contribute to Rust’s future successes](https://aws.amazon.com/blogs/opensource/how-our-aws-rust-team-will-contribute-to-rusts-future-successes/)
   > - March 2021

---

# Another Test

---

# How can I stay in the loop?

- **Jon Gjengset** - Youtube for advance Rust tutorials
- **This week in Rust** - Weekly articles about Rust made by the community
- **Rustacean Station** - Podcasts on rust the language and it's projects
- **Rust** - Youtube channel with talks and projects on rust
- **Niko Matsakis Blog** - 

To get more involved, I would recommend subscribing to this week in rust.

---

# More "Consider Rust" Presentations

- [Considering Rust](https://www.youtube.com/watch?v=DnT-LUQgc7s) By Jon Gjengset
- [Unlocking Rust's power through mentorship and knowledge spreading, with Tim McNamara](https://open.spotify.com/episode/32rkz55Jm4MRsO1AEmH9UT?si=b1e1791c40ff47be)
- [Tom Hank - Why Rust is a significant developer in dont include this](https://www.youtube.com/watch?v=IwjlCxwcuIc)

---

# Special Thanks

- Joh Gjegset - For creating amazing Rust content

---

# Thanks for listening

Any questions :) 