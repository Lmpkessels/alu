# ALU

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-red.svg)
![Status Slowed down](https://img.shields.io/badge/Status%20-Slowed-down-orange.svg)

**ALU** is a Rust playground for binary math, logic gates, and arithmetic operations.

Originally started as binarySeries, it grew into a 32-bit ALU simulation inspired by Nand2Tetris (Ch. 1–3).

## Features

- 32-bit binary add, subtract, multiply, divide
- Overflow/borrow flag support
- Bitwise gates (AND, OR, XOR, NAND)
- Integer ↔ bit converters
- Unit-tested per module

This project was built as a foundation exercise to deepen my understanding of binary arithmetic, flags, and low-level system design in Rust.

## Setup

```bash
git clone https://github.com/Lmpkessels/alu.git
cd alu
cargo test
```

Navigate through folders by topic.

## Folder Structure

```
src/
├── adders/      # Binary adders, subtractors, multipliers, dividers
├── alu/         # Arithmetic Logic Unit
├── gates/       # Logic gates (AND, OR, XOR, NAND, etc.)
├── operators/   # Integer–bit converters, transformations
```

> Each folder contains its own logic and tests.
> Run `cargo test` to validate any module.

## License

This project is licensed under the [MIT License](./LICENSE). <br/>
© 2025 Luuk Kessels

## Status

This project is currently in maintenance mode, but I may add new features in the future if they support my goals.

## Connect

- 📧 [l@lmpkessels.com](mailto:l@lmpkessels.com)
- 🐦 [@lmpkessels on X/Twitter](https://x.com/lmpkessels)
- 🛠️ [Open an issue](https://github.com/Lmpkessels/alu/issues/new)
