rust-assimp [![Build Status](https://travis-ci.org/jemcroft/rust-assimp.svg?branch=master)](https://travis-ci.org/jemcroft/rust-assimp)
===========

[Documentation](http://www.rust-ci.org/jemcroft/rust-assimp/doc/assimp/)

## Notice
Original repo vanished a while ago. This one is just an attempt to get
a working build on a stable version of the compiler.

## Building

## Examles

### Simple import example
This example sets up logging, loads a model and prints all its vertices to
stdout.

```rust
extern crate assimp;

use assimp as ai;

fn main() {
    // Log to stdout and a file `log.txt`
    ai::log::add_log_stream(ai::log::Stdout);
    ai::log::add_log_stream(ai::log::File("log.txt"));
    ai::log::enable_verbose_logging(true);

    let importer = ai::Importer::new();

    // The file to import
    let scene = importer.import("examples/assets/cube.dae").unwrap();

    // Print all the vertices in all the meshes
    for mesh in scene.get_meshes().iter() {
        for vert in mesh.get_vertices().iter() {
            println!("{:?}", vert);
        }
    }
}
```
