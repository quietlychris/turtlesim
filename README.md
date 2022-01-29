# turtlesim* [ 🐢 + 🦀 = 🙂 ] 
### it's turtles(im) all the way down!™ 
_*with [`bissel`](https://github.com/quietlychris/bissel), an experimental robotics-focused middleware written in Rust_

---

__*Ever wanted make an [animated turtle](https://docs.ros.org/en/rolling/Tutorials/Turtlesim/Introducing-Turtlesim.html) move around on screen? Now you can, but with Rust!*__

<p align="center"><img src="assets/turtlesim.gif" alt="screenshot" width="30%"/></p>

Turtlesim is a autonomy simulator made popular by [ROS/2](https://docs.ros.org/en/rolling/Tutorials/Turtlesim/Introducing-Turtlesim.html), which is described as

> Turtlesim is a lightweight simulator for learning ROS 2. It illustrates what ROS 2 does at the most basic level, to give you an idea of what you will do with a real robot or robot simulation later on.

This can be run using the command
```sh
# This will start the simulator and allow you to manually 
# move the turtle around using the arrow keys (and [ESC] to exit)
$ cargo run --bin turtlesim 
```
or script the turtle's motion using a Bissel program at 
```sh 
$ cargo run --bin move_turtle
```
## Background

ROS/2 is one of the most commonly-used robotics middleware platforms, although it exists alongside other messaging platforms like [MOOS-IvP](https://oceanai.mit.edu/moos-ivp/pmwiki/pmwiki.php?n=Main.HomePage),  [MQTT](https://mqtt.org/), and [ZeroMQ](https://zguide.zeromq.org/docs/chapter1/). However, each of these alternatives have something in common; they're all written in C++. 

This Turtlesim repository is a first step in the direction of testing out an experimental robotics middleware, called [Bissel](https://github.com/quietlychris/bissel). It is written in pure Rust, and integrated with [Bevy](https://bevyengine.org), a "refreshingly simple data-driven game engine", creating an extensible base for building more complex autonomy simulations. To be clear, Bissel is almost certainly the weakest link of in this chain, but regardless of the de/merits of the implementation itself, but this experience has made it clear that writing robotics software using Rust is both feasible and may offer some meaningful improvements over existing languages. 

Other benefits of using Rust in robotics projects:
- _Catch errors at compile-time_: Runtime errors are robots' monster under the bed. Autonomy software is often mission-critical, and errors at runtime potentially mean losing a vehicle or worse. The Rust compiler, with compile-time assurances around static typing, mutability, memory management, and concurrency helps prevent many classes of bugs before the code ever makes it to deployment. An accessible ecosystem of high-quality developer tools makes unit and integration tests, documentation checks, formatting lints, and more exceptionally easy. 
- _Low-overhead dependency management_: Working wth and building ROS/2 packages has gotten much easier with the introduction of [`colcon`](https://design.ros2.org/articles/build_tool.html) (see bottom of the page), but is still not painless; developers [need](https://docs.ros.org/en/rolling/Tutorials/Writing-A-Simple-Cpp-Publisher-And-Subscriber.html) to competent with CMake, `package.xml`, [`rosdep`](http://docs.ros.org/en/rolling/Installation/Ubuntu-Development-Setup.html?highlight=rosdep#install-dependencies-using-rosdep), and `ament`-related structures. Comparatively, Rust projects use the standard `cargo` package/build system for all aspects of dependency management. 
- _Easy cross-compilation_: Compare the cross-compilation of [ROS2](http://docs.ros.org.ros.informatik.uni-freiburg.de/en/rolling/How-To-Guides/Cross-compilation.html) with Rust's [`cross`](https://github.com/cross-rs/cross) tool, which supports virtually all Rust compilation targets, including popular SBCs like the Raspberry Pi.
- _Limited environment configuration_: Rust uses static linking to create binaries, which drastically cuts down on the number of system-level dependencies required on either development or deployment machines. As standard Rust packages, operating system compatibility tracks the upstream ecosystem--no more worrying about which particular Ubuntu distros are currently supported. 

Rust is language with a [mission](https://www.rust-lang.org/) of "empowering everyone to write reliable and efficient software." This is especially important in the robotics space, where reliable autonomy software is often the foundational aspect of successful projects. In short, the goal of Bissel is to make writing mission-critical robotics software more accessible. Developers don't need to be experts in how class member access operators So don't worry about getting your system configuration perfect, and jump right into developing for robots (just worry about the number of pre-1.0 software packages being used instead)!

### Development

By default, Turtleim is configured to do [fast-compile](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional) using the Bevy dynamic linking feature. This can be turned off by modifying the `Cargo.toml` to, 
```toml
# bevy = {version = "0.6", features = ["dynamic"]}
bevy = "0.6
```
or built manually using this feature using 
```sh
$ cargo run --bin turtlesim --features bevy/dynamic
```

### Additional Resources

Also, check out the following:
- [Bevy](https://github.com/bevyengine/bevy): The Bevy game engines ECS represents an exciting new path for writing high-quality robotics simulations
- [`turtle`](https://github.com/sunjay/turtle): Another way to move a turtle-y thing around
- [rust-mq](https://github.com/gridgentoo/rust-mq): A ZeroMQ message client library written in Rust
- [MOOS-IvP](https://oceanai.mit.edu/moos-ivp/pmwiki/pmwiki.php?n=Main.HomePage): Marine robotics-focused middleware
- [ROS2](https://docs.ros.org/en/rolling/): ROS2 Rolling documentation

