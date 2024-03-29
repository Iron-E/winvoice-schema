//! This crate provides definitions for the information which is managed by CLInvoice. The data is
//! represented as it would be after all `JOIN`s are performed on a database (e.g. an
//! [`Organization`] in a database would likely reference [`Location`] by [`Id`], rather than
//! aggregating it).
//!
//! # Features
//!
//! * `serde` adds support for the [`serde`] crate.
//!
//! # Re-exports
//!
//! The crate provides access to the following elements of other crates:
//!
//! * Elements of the [`money2`] which are required to instantiate data (e.g. [`Money`]).
//! * The entire [`chrono`] crate, as almost all of it is required to instantiate certain data.

#![allow(clippy::drop_non_drop)]
#![forbid(unsafe_code)]
#![warn(
	missing_docs,
	clippy::alloc_instead_of_core,
	clippy::allow_attributes_without_reason,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::cast_lossless,
	clippy::checked_conversions,
	clippy::cloned_instead_of_copied,
	clippy::dbg_macro,
	clippy::debug_assert_with_mut_call,
	clippy::doc_link_with_quotes,
	clippy::doc_markdown,
	clippy::empty_line_after_outer_attr,
	clippy::empty_structs_with_brackets,
	clippy::enum_glob_use,
	clippy::equatable_if_let,
	clippy::exit,
	clippy::explicit_into_iter_loop,
	clippy::explicit_iter_loop,
	clippy::fallible_impl_from,
	clippy::filetype_is_file,
	clippy::filter_map_next,
	clippy::flat_map_option,
	clippy::fn_to_numeric_cast_any,
	clippy::format_push_string,
	clippy::from_iter_instead_of_collect,
	clippy::get_unwrap,
	clippy::implicit_clone,
	clippy::inefficient_to_string,
	clippy::items_after_statements,
	clippy::manual_assert,
	clippy::manual_ok_or,
	clippy::map_unwrap_or,
	clippy::match_same_arms,
	clippy::missing_const_for_fn,
	clippy::missing_panics_doc,
	clippy::multiple_inherent_impl,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::option_if_let_else,
	clippy::option_option,
	clippy::range_minus_one,
	clippy::range_plus_one,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_binding_to_reference,
	clippy::ref_option_ref,
	clippy::same_functions_in_if_condition,
	clippy::single_char_lifetime_names,
	clippy::std_instead_of_core,
	clippy::str_to_string,
	clippy::string_add,
	clippy::string_add_assign,
	clippy::string_to_string,
	clippy::try_err,
	clippy::unnecessary_join,
	clippy::unnecessary_wraps,
	clippy::use_self,
	clippy::used_underscore_binding,
	clippy::wildcard_imports
)]

mod contact;
mod employee;
mod expense;
mod id;
mod invoice;
mod invoice_date;
mod job;
mod location;
mod organization;
mod restorable_serde;
mod restore_error;
mod timesheet;

pub use chrono;
pub use contact::{Contact, ContactKind};
pub use employee::Employee;
pub use expense::Expense;
pub use id::Id;
pub use invoice::Invoice;
pub use invoice_date::InvoiceDate;
pub use job::Job;
pub use location::Location;
pub use money2::{Currency, Decimal, Money};
pub use organization::Organization;
pub use restorable_serde::RestorableSerde;
pub use restore_error::{RestoreError, RestoreResult};
pub use timesheet::Timesheet;
