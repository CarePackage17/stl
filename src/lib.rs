#[macro_use]
extern crate nom;

mod data;
pub mod ascii;
pub mod binary;

pub use data::{Vertex, Facet};
