# wasm + react

A **brainfuck** interpreter using rust wasm + webpack + react.

## Project structure

The (rather messy) project structure

```txt
./
|-- app
|     App.tsx           # the react app
|-- cli
|     main.rs           # the cli app for native rust
|-- public
|     index.css         # index page css
|     index.html        # index page for ReactDOM render
|-- src
|     bf.rs             # bf interpreter (platform independent)
|     lib.rs            # module exposed to wasm
|     util.rs
|-- index.js
|-- Cargo.toml
|-- package.json
|-- webpack.config.js
<some other files>
```

## Building the project

You need cargo and `cargo` and `nodejs` to build the project (you probably need to do `npm install` first).

To run the preview server:

```sh
$ npm run serve
# some npm output, probably on http://localhost:8080/
```

To build the project (output will be in `/docs`):

```sh
$ npm run build
# some npm output
```

You can also compile the project to native and run the `cli` using:

```sh
$ cargo run cli
# some cargo output
prog> 
```
