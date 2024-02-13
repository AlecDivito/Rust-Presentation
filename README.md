# Rust presentation

This is my rust presentation that gives a general overview of the language along
with the 3 features that I think Rust does better then any other programming language.

I'm sorry about the state of this repo, but i did not have time to clean it up
before getting ready for the presentation.

There are 2 versions of this presentation. `presentation.md` contains an old presentation
that isn't polished and was my first attempt at making it. `3_things_about_rust.md`
is a polished 20+ minute presentation that gives a rough overview of the rust
programming language.

## Development

This is markdown that uses the tool `marp` to convert it into a presentation. Development
of the presentation can use the `marp` extension inside of VSCode to give a live
preview of the slides. Building the slides can be done by downloading the `marp`
command line tool.

For macos, install the tool with brew

```bash
brew install marp-cli
```

## Build

Build the presentation into a power point with the following command

```bash
marp --pptx --allow-local-files ./3_things_about_rust.md
```

## Host

Host the website on a server with the following command

```bash
marp --server
```

This will allow you to access the presentation through the browser.
