# Combinatorial Algorithms in Rust

> A personal reconstruction of classic combinatorial algorithms — rebuilt,
> documented, tested, and understood through modern Rust.

---

## Motivation

This project began as a deep reading of **_Combinatorial Algorithms for Computers and Calculators_** by Nijenhuis & Wilf, with a personal journey of learning **_Rust_** the programming language in a practical **"learning-by-doing"** approach.

The original book is mathematically rich, but:
- Algorithms are presented in dense, imperative **FORTRAN** that is rarely used in today's computational world.
- Control flow obscures invariants, and it's extremely hard to follow with mind-tracing.
- Implementation details overwhelm conceptual clarity

Rather than translating code line-by-line, this project **reconstructs** the
algorithms from first principles.

The goal is not speed, nor novelty — but **understanding and implementation**.

---

## What This Project Is

✔ A systematic rewrite of classical combinatorial algorithms  
✔ Algorithms expressed in **clear abstract models** (CLRS-style)  
✔ Correct, idiomatic **Rust implementations**  
✔ Extensive unit tests and integration tests  
✔ Applied solutions to relevant **LeetCode medium and hard problems**  
✔ Visualizations (SVG / CLI / ASCII) where structure matters

---

## What This Project Is Not

✘ A production-grade combinatorics library  
✘ A “competitive programming” shortcut collection  
✘ A minimal or polished framework

This repository reflects **learning in motion**, including:
- many incorrect early attempts
- rounds of refactoring
- continuous fixes for failed tests
- algorithmic misconceptions that were later corrected

These are preserved deliberately for improving personal learning experiences.

---

## Design Principles

1. **Algorithm first, code second**
2. **State invariants over control flow**
3. **Correctness before optimization**
4. **Tests as executable explanations**
5. **Visualization when structure is non-linear**

---

## Status

All core modules:
- compile
- pass unit tests
- pass integration tests
- are cross-checked against known results

This project will continue to evolve slowly and deliberately.

---

## Acknowledgements

- Nijenhuis & Wilf — original algorithms
- CLRS — algorithmic clarity
- *The Rust Programming Language* — safety and expressiveness
- LeetCode — stress-testing correctness
- ChatGPT — interactive reasoning and debugging partner

---

## License

MIT — this work is intended to be read, reused, and learned from.
