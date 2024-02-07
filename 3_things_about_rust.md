---
marp: true
author: Alec Di Vito <alecdivito>
footer: Learning Rust
theme: alec-custom
transition: fade
---

<!-- paginate: skip -->

# Learning to love the Rust Propaganda

How I learnt to love the borrow checker and why you should to.

Alec Di Vito

February 2024

---

<!-- paginate: true -->

# Who am I?

- **Alec Di Vito**
- **Job**: ~~Senior~~ Staff Cloud Developer
- **Experience in:** Python, Go, Javascript, Java and Jenkinfiles
- **Loves:** Rust 😍

Get in Touch

- Slack
- Email

![bg right:40% vertical](./assets/alec_1.jpg)

---

## What we'll cover today

- What is Rust
- Who likes Rust
- History of Rust
- My favorite 3 features of Rust
- The Rust tools and ecosystem

## What we won't be doing

- Learning how to program in Rust

---

## What is this Rust?

- Rust is a general purpose programming language
- Popular in systems programming
- Rust's features
  - Performance
  - Type Safety
  - Memory Safety
  - Fearless Concurrency

![bg right:33% w:90% vertical](./assets/rust_mascot.svg?image=3)
![bg w:90%](./assets/rust_logo.svg?image=4)

---

## Where is Rust used? 

- [The Linux Kernel](https://www.kernel.org/doc/html/next/rust/index.html)
- [Amazon Web Services built firecracker in Rust](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)
- [Microsoft is spending $10 million to make it a 1st class language](https://twitter.com/dwizzzleMSFT/status/1720134540822520268?s=20)
- [Cloudflare](https://blog.cloudflare.com/tag/rust), [Figma](https://www.figma.com/blog/rust-in-production-at-figma/), [Meta](https://github.com/facebook/sapling), [and many more](https://github.com/omarabid/rust-companies?tab=readme-ov-file)
- Probably some Web 3 Project you or I haven't heard of
- Some teams at RBC 👀

> 🚀 **Congratulations**
> It's 2023 and Rust is the most admired programming language for 8 years in a row
> ‐ [2023 Stackoverflow Survey](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages)

---

## The story behind the project

- Started as a personal project in 2006 by Graydon Hoare
- He was a 29 year old developer working at **Mozilla**
- The elevator wasn't working again after work; Software had crashed
- While walking up **21 floors** he thought about how software developers couldn't make an elevator work without crashing
- Rust was born, **"over-engineered for survival"**

---


## History of Rust

- Mozilla research sponsored the project in 2009
  - It was used to develop the servo browser engine
- Over the years Rust changed a lot
- In 2015 it had it's 1.0 release
- Ever sense adoption has been growing

![bg right:33% w:60% vertical](./assets/mozilla.webp)
![bg w:70% invert](./assets/servo.png)


---

## It's all about memory safety

- By spending more time up front, you could just skip programming memory bugs
- Browsers are often an attack vector for hackers
- What if the program we used could protect us from most attacks?

> 🔔 **info**
> Microsoft estimates that 70% of the vulnerabilities in its code are due to memory errors from code written in these languages.

---

## Rust Influences

Rust is not an original language, instead it's borrowed design elements from a wide variety of languages.

- **OCaml** for algebraic data types and pattern matching
- **C++** for references, smart pointers, move semantics, and more
- **Scheme** for hygienic macros
- **Swift** for optional bindings
- **Newsqueak, Alef, Limbo** for channels, concurrency
- [And More](https://doc.rust-lang.org/reference/influences.html)

---

# Introducing Rust

---

## Key words in rust _for this presentation_

Every language has reserved keywords that the language uses. These are some in which we'll be coving today.

| Construct | Description |
| -------- | ---------------------------------------- |
| `struct` | Where you store all of your objects data |
| `enum`   | When an object could be in different states (example: `Some` or `None`) |
| `fn`     | A function that you can pass arguments to |
| `trait`s | Rust versions of interfaces from languages like Java |
| Generics | Ability for a type to implement different actions |
| `mut`ability | Declaring if a variable is mutable |

---

# 3 Features you wish your language had

These are the 3 features that keep me coming back to Rust

---

## 1. Strong Typing

- Rust main feature (imo) is it's strong type system
- Types are like small tests all over your program
- Protect your team by requiring you to think of all possible errors during development
- Explicit typing to make variables `mut`able and `pub`lic

### Why are these strengths?

- Rust uses the type system to create stronger features like:
  - Tracking memory safety
  - Tracking variable lifetimes
  - Validating there is only ever one writer to a variable

---

## A strong type system makes issues seen in languages like javascript impossible

For example take the following Javascript example

```javascript
assert(1 + "2" == 12)
```

<br />

In Rust, the string type needs to be converted to a number before doing the addition. A Rust developer may program the same test like so.

```rust
assert_eq!(1 + "2".parse::<u32>().unwrap(), 3)
```

---

## Strong Types enforce error handling practices

However, what are the issues with the below code.

```rust
assert_eq!(1 + "2".parse::<u32>().unwrap(), 3)
```

Well the `.unwrap` call will stop the program if `2` wasn't a number. In a real Rust program, a developer would want to handle the error which they could do with a `match` expression

```rust
match "2".parse::<u32>() {
    Ok(num) => assert_eq!(1 + num, 3),
    Err(err) => { ... } // handle the error
}
```

---

## Benefits of a strong type system

1. Can't convert variables between types easily
2. Strict validation that a type implements a behavior (interface) at compile time

---

## 2. The borrow checker

Rust answer to how to keep memory in your application safe. It's made up of 3 components:

1. Tracking who has **Ownership** of a variables
2. Checking where the variable has been **Borrowed**
3. Validating if the variable **lifetime** is long enough

### If a program can't pass the borrow checker, it doesn't compile

---

## Understanding the borrow checker

Rust wants all code to have only **1 writer** and **multiple readers** at all times. The borrow checker enforces this idea on a program.

Readers share a read-only references to the ownership of an object.

If the ownership of an object were to fall out of scope while a reference to the object is being borrowed, one could say that the reference **out lived** the variable.

### This protects objects from being used after being free'd

---

## Example of passing ownership

In the example below, the program does not compile because it's trying to be used after the ownership of the variable has changed.

```rust
fn passing_ownership(obj: ExampleStruct) {}

fn main() {
    let example = ExampleStruct {}
    passing_ownership(example) // <-- Move of `example` variable happens
    print!("{}", example)      // ERROR! `example` is now owned by the function
}
```

---

## What people have to say about the Borrow checker

Users of Rust struggle with the borrow checker the most. This has required the language to develop more advanced tutorials so that users would get a better understanding.

![bg contain right:40%](./assets/stabby_mascot.png)

---

## 3. Fearless concurrency

**This is technically my first favorite feature!** But fearless concurrency requires that the strong type system and borrow checker exist to work. Imagine having multiple threads in a program, the borrow checker validates that

- Only 1 variable has mutable access at a time
- That the variable lives as long as the thread
- That there is only one owner of the data


---

# Lighting round


Because there was just even more features I wanted to talk about that I never got the time to cover!

---

## Testing in documentation

Create example code in your comments which are also tested when building a library!

```rust
/// Adds two numbers together
///
/// ```
/// /// Validate that add_one works!
/// use lib::add;
/// assert_eq!(add(41, 1), 42) // <---- this code runs on `cargo test`!!!
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

---

## Rust Macros!

Quickly create a CLI application by marking up a struct with macros using the [`clap` package](https://docs.rs/clap/latest/clap/)!

```rust
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
```

---

## CLI in action

Just with a bit of macro markup we have a beautiful help

```bash
➜  example git:(main) ✗ cargo run -- --help
Simple program to greet a person

Usage: example [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

---

## More Rust Macros

How about validating SQL at compile time?
**YUP RUST CAN DO THAT** thanks to the [crate `sqlx`](https://docs.rs/sqlx/latest/sqlx/)!

```rust
sqlx::query!(r#"SELECT id, description, done FROM todos ORDER BY id"#)
    .fetch_all(pool)
    .await?;
```

Those are the main ones I've used but theres more like:

- Need to work with JSON, XML, YAML, ect? Use the popular [`serde` crate](https://crates.io/crates/serde)
- Create types for `graphql` queries thanks to [`graphql_client`](https://crates.io/crates/graphql_client)
- Love the builder pattern? Theres a macro for that! [`derive_builder`](https://crates.io/crates/derive_builder)

---

## MORE MACROS WTF!!

WAIT YOU WANT TO PROGRAM YOUR NODE AND PYTHON PACKAGES IN RUST TOO!!

Well let me tell you...**RUST.CAN.DO.THAT.TO**

- Don't like using that startup library called `react` 🤢? Use the [crate `yew`](https://crates.io/crates/yew) instead and program your frontend in Rust 🤩!
- Need to go fast but you use Javascript 🤦‍♂️? No worries, [use `wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/introduction.html) to deploy Rust code as an npm module 🤯!
- Need a fast language but your stuck with Python 😭. Gain some _wheels_ ♿️ by developing a native python extension module using [`pyo3`](https://github.com/PyO3/pyo3)

---

# Rust tools

TODO :) 

---

# Where does rust stand with other languages

In my eyes, Rust stand's upon all

![bg cover right:40%](image.png)

> This is my first attempt at using DALL-E 🦀 DON'T JUDGE ME!!

---

# Where can I learn more?

I understand if this presentation has made you interested in learning why Rust is such a loved language. To help support you on this journey, Try and review the following:

- [**Rust book**](https://doc.rust-lang.org/): Learn the basics of the language
- [**Learn Rust with entirely too many linked lists**](https://rust-unofficial.github.io/too-many-lists/): More advanced rust
- [**Jon Gjengset**](https://www.youtube.com/c/jongjengset): Youtube channel covering advanced projects and material in rust
- [**Proc Macro workshop**](https://github.com/dtolnay/proc-macro-workshop): Workshop example on how to make your own rust macros

---

# How can I stay in the loop?

- [**This week in Rust**](https://this-week-in-rust.org) - Weekly articles about Rust made by the community
- [**Rustacean Station**](https://rustacean-station.org) - Podcasts on rust the language and it's projects
- [**Rust**](https://www.youtube.com/@RustVideos) - Youtube channel with talks and projects on rust

Other forms I use is also following matiners

- [**Niko Matsakis Blog**](https://smallcultfollowing.com/babysteps/) - About the language and it's development

To get more involved, I would recommend subscribing to this week in rust. They have a section on contibuting to open source projects that need help.

---

# More "Consider Rust" Presentations

- [Considering Rust](https://www.youtube.com/watch?v=DnT-LUQgc7s) by Jon Gjengset
- [Unlocking Rust's power through mentorship and knowledge spreading, with Tim McNamara](https://open.spotify.com/episode/32rkz55Jm4MRsO1AEmH9UT?si=b1e1791c40ff47be) who lead Rust education across Amazon

---

# Special Thanks

- Joh Gjegset - For creating amazing Rust content

---

# Thanks for listening

![width:700px height:270px](./assets/twitter.png)

<!-- https://twitter.com/p1xelHer0/status/1754011970158669855 -->