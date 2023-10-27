//! # Wrapping the Modelica language in Rust
//! 
//! This crate is a wrapper for the Modelica language, used primarily for
//! modeling physical systems. It is **not** ready for use by anyone for any reason.


/// The component level of the Modelica language
pub struct ModelicaComponent {
    pub name: String,
}