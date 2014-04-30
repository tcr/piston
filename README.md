Piston
======

A user friendly game engine written in Rust

Built on top of
* [rust-opengles](https://github.com/bvssvni/rust-opengles)
* [glfw-rs](https://github.com/bvssvni/glfw-rs)
* [rust-graphics](https://github.com/bvssvni/rust-graphics)

Test project: [rust-snake](https://github.com/bvssvni/rust-snake)

## How to build Piston

Build the folliwing projects and put the '.rlib' files to '/target/cpu-vendor-os/lib/':

* [GLFW-RS](https://github.com/bjz/glfw-rs)
* [Rust-Graphics](https://github.com/bvssvni/rust-graphics)
* [Rust-OpenGLES](https://github.com/mozilla-servo/rust-opengles)

There are currently many breaking changes in Rust, so please help the maintainers to keep them up with master!  

In the Terminal window, navigate to the project folder and type:

```
make rlib
```

This gives you a new '.rlib' file in the '/target/cpu-vendor-os/lib/' folder to put in your project.

If you are starting a new project, [Rust-Empty](https://github.com/bvssvni/rust-empty) will automate the setup of directories for you.

## Goals

* Test the design and performance of Rust-Graphics
* Experiment with Rust-ish game design and collect feedback
* Actor/events AI modelling
* Make more people use Rust for game development
* Multi-platform game development
