# axiom

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-red.svg)

**axiom** is a Rust playground for binary math, logic gates, and ALU design.

It started as _binarySeries_ and evolved into an implementation of an ALU (Arithmetic Logic Unit).

Inspired by the first chapters of _Nand2Tetris_ (Ch. 1–3), it serves as a lab for learning low-level systems, logic, and math.

## Setup

```bash
git clone https://github.com/Lmpkessels/axiom.git
cd axiom
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

## Connect

- 📧 [l@lmpkessels.com](mailto:l@lmpkessels.com)
- 🐦 [@lmpkessels on X/Twitter](https://x.com/lmpkessels)
- 🛠️ [Open an issue](https://github.com/Lmpkessels/axiom/issues/new)
