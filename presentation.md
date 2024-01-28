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
- **Job**: Senior Cloud Developer
- **Experince in:** Python, Go, Javascript, Java and Jenkinfiles
- **Uses Rust:** In free time

Get in Touch

- Slack
- Email

---

# Why should I listen to you?

Good question

Often when I ask others about adopting Rust, they are strongly against it.

However i believe it can help the bank run more software faster. 

---

## What we'll cover today

- High level comparison to Python, Java and Go
- Rust's Major features
- Rust's Drawbacks
- Long-Term viability

## What we won't be doing

- Learning Rust

---

# Python vs Rust

---

# Java vs Rust

---

# Go vs Rust

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

---

# Special Thanks

- Joh Gjegset - For creating amazing Rust content

---

# Thanks for listening

Any questions :) 