//! Rather than running all tests in separate processes (by keeping them in
//! separate files in deps/tests), we group all the tests in each section of the
//! book in the same crate.
//!
//! All examples in this folder MUST be listed as module below or they won't be
//! discovered / run.

mod allow_dead_code;
mod associated_types;
mod async_main;
mod attributes_debug;
mod attributes_deprecated;
mod attributes_must_use;
mod attributes_production;
mod borrowing;
mod borrowing_mutable;
mod clone;
mod closures;
mod closures_as_input_parameters;
mod closures_move;
mod closures_with_type_annotations;
mod conditional_compilation;
mod const_in_traits;
mod copy;
mod destructuring;
mod diverging_functions;
mod enums;
mod for1;
mod function_pointers;
mod functions;
mod generic_functions;
mod generic_functions2;
mod generic_lifetime;
mod generic_structs;
mod generic_traits;
mod if_else;
mod if_let;
mod iterators;
mod lifetime;
mod loop1;
mod macros;
mod main_test;
mod match1;
mod match2;
mod modules;
mod modules2;
mod modules3;
mod newtype;
mod ownership;
mod ownership2;
mod rpit;
mod shadowing;
mod slices;
mod static_lifetime;
mod structs;
mod structs2;
mod structs3;
mod structs4;
mod trait_bounds;
mod trait_objects;
mod traits;
mod traits2;
mod traits3;
mod traits4;
mod traits5;
mod traits_as_parameters;
mod vars_and_consts;
mod while1;
mod while_let;
