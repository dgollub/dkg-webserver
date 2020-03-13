# dkg-webserver - a simple web server written in Rust

This project implements a minimal web server.

It is mostly based on the [web server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) chapter
of the [The Rust Programming Language Book](https://doc.rust-lang.org/book/).

While the implementation follows the book for the most part, but I take some liberties
to implement certain things and parts differently.

The server can only handle `GET` requests right now.

# Building and Running

You need the Rust programming language installed ([installation guideline](https://doc.rust-lang.org/book/ch01-01-installation.html)).

Then simply run `cargo run` to run the web server in your terminal.

By default it will listen on `localhost` on port `8080`.
Easy access: [http://localhost:8080/](http://localhost:8080/)


# Copyright

Copyright 2020 (c) by Daniel Kurashige-Gollub <daniel@kurashige-gollub.de>


# License

[MIT](LICENSE)
