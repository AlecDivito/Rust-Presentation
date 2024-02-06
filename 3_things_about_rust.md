---
marp: true
author: Alec Di Vito <alecdivito>
footer: Learning Rust
theme: alec-custom
transition: fade
---

<!-- paginate: skip -->

# Learning the Rust Propaganda

How I learnt to love the borrow checker and why you should to.

Alec Di Vito

February 2024

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

![bg right:40% vertical](./assets/alec_1.jpg)

---

## What we'll cover today

- What is Rust
- History of Rust
- Who's using Rust
- My favorite 3 features of Rust
- The Rust ecosystem

## What we won't be doing

- Learning programming in Rust in depth

---

## What is this "Rust"?

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

## Rust Usage! 

- [The Linux Kernel](https://www.kernel.org/doc/html/next/rust/index.html)
- [Amazon Web Services built firecracker in Rust](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)
- [Microsoft is spending $10 million to make it a 1st class language](https://twitter.com/dwizzzleMSFT/status/1720134540822520268?s=20)
- [Cloudflare](https://blog.cloudflare.com/tag/rust), [Figma](https://www.figma.com/blog/rust-in-production-at-figma/), [Meta](https://github.com/facebook/sapling), [and many more](https://github.com/omarabid/rust-companies?tab=readme-ov-file)
- Some teams at RBC 👀

> 🚀 **Congratulations**
> It's 2023 and Rust is the most admired programming language for 8 years in a row
> ‐ [2023 Stackoverflow Survey](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages)

---

## History of Rust

- Started as a personal project in 2006 by Graydon Hoare
- Mozilla research sponsored the project in 2009
  - It was used to develop the servo browser engine
- Over the years Rust changed a lot
- In 2015 it had it's 1.0 release
- Ever sense adoption has been growing

---

## The story behind the project

- Graydon Hoare was a 29 year old developer
- The elevator wasn't working again; Software had crashed
- While walking 21 floors he thought about how software developers couldn't make an elevator work without crashing
- Rust was born, "over-engineered for survival"

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

## What is a "Garbage collector"

Tracks all pointers in the program
Allocates memory for you to be used automatically
Cleans up memory when no longer in use

Used in popular languages like:
- Javascript
- Java
- Go

---

# 3 Features you wish your language had

These are the 3 features that keep me coming back to Rust

---

## 1. Fearless concurrency