Rusty TSP
=========

Rusty TSP is an implementation of 2-opt for solving the traveling salesman problem. I wanted to work on a simple proof of concept for WebAssembly + Rust, which opens up very interesting possibilities in the realm of web apps.

It is a wasm-pack project, along with an index.html at the root, referencing JavaScript, CSS and assets located in the demo folder. To build it, run:

```
wasm-pack build -t no-modules
```

It cannot be built as a ES6 module because it is run in a Web Worker, and module worker support is limited to Chrome at the time of writing.

Check out [a live demo of this repo](https://rustytsp.carlos.marchal.page).