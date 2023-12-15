> Ultra beta version (e.g. simple POC of the algo)

# CSS Minifier

CSS Minifier is a simple Rust program that takes a CSS file as input and minifies it.

The objective of this algorithm is to produce a single minified string representing your stylesheet.

## Features

- Removes unnecessary semicolons, spaces, and line breaks.
- Eliminates all comments from the CSS file.

## To-Do List

- [ ] Create a separate library crate.
- [ ] Implement improved error handling.
- [ ] Add support for nesting in the CSS.
- [ ] Enhance testing procedures.
- [ ] Minimize variable names for a more compact output.

## Usage

To use the CSS Minifier, run the program with the path to your CSS file as an argument.

You can add a target path as second argument (optional) if you don't want to overwrite the og stylesheet.

```bash
cargo run -- style.css (target.css)
```
