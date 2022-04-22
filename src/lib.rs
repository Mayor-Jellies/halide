//#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]
//!This crate is an example of calling Halide generated code from Rust. 
//!This crate also has an example of IIR-blur halide app that get called from rust.
//! IIR blur takes a image input, goes through the full halide pipeline, and outputs a blurred image all using rust.  
//!

///This Halide generator build using GenBuilder 
///
///It follows (Compile->Run->make->rename->rename->bind)
pub mod build;

#[doc = include_str!("../README.md")] 
///tests for build
///
///Basic tests to check the status of (Compile->Run->make->rename->rename->bind)
#[cfg(test)]
mod build_tests;

///To make the Halide_buffer T easier to access
///
/// Halide_buffer T
pub mod runtime;

///tests for build
///
#[cfg(test)]
mod runtime_tests;

