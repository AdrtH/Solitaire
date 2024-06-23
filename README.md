# Solitaire
A Klondike game, written in [Rust](https://www.rust-lang.org/), using [Raylib](https://www.raylib.com/) and its [rust binding](https://github.com/deltaphc/raylib-rs).

# How to build it
You will need [Cargo](https://doc.rust-lang.org/cargo/) and [Raylib](https://www.raylib.com/) installed on your machine, I won't detail how to install neither of them, go to their dedicated website, both are well documented.

First, clone the repository:
```
git clone https://github.com/AdrtH/Solitaire
```

Then, go to that directory, for example, using Linux:
```
cd Solitaire
```

After that, you can use cargo to run/build/install, I recommend installing it, using this command:
```
cargo install --path .
```

# How to use it
After running it, you'll be directly on a game, you can start a new game using R and Escape to quit.
Currently, this does not support drag and drop for moving, so you need to click to move.

# Legal things
No modifications were made to neither Raylib nor its rust binding.
