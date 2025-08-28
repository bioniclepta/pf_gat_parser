# Powerflow Grid Analysis Tool \[Parser\] (pf_gat_parser)
<!-- [![Docs.rs](https://docs.rs/pf_gat_parser/badge.svg)](https://docs.rs/pf_gat_parser) -->
[![CI](https://github.com/bioniclepta/pf_gat_parser/actions/workflows/rust.yml/badge.svg)](https://github.com/bioniclepta/pf_gat_parser/actions)

> A fast parser for PSS/E `.raw` power system model files, written in Rust.

pf_gat_parser is a foundational library for power systems analysis in the Rust ecosystem. It provides robust, efficient, and strongly-typed parsing for the PSS/E RAW data format, transforming complex grid models into accessible Rust structs. This project was built to serve as the backbone for modern grid analysis tools, prioritizing performance and safety.

## Features

- **Blazing Fast**: Built with Rust's zero-cost abstractions to handle large-scale models, like the full WECC or MISO systems, with ease.
- **Type-Safe**: Converts the raw text data into strongly-typed Rust structs, preventing entire classes of bugs at compile time.
- **Robust Error Handling (WIP)**: Provides clear, actionable error messages, pointing to the exact line and issue in the `.raw` file.

## Project Goals

The ultimate goal is for pf_gat to be a foundational crate for a new generation of power systems analysis tools — from fast decoupled power flow solvers to dynamic simulations and contingency analysis.

## Getting Started
WIP

## Disclaimer
This crate is not finished yet and will be constantly changing. It is useable in its current state, but there are still features and changes I wish to make.