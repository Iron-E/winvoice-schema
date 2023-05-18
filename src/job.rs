mod default;
mod display;
mod exchange;
mod restorable_serde;

use core::time::Duration;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Id, Invoice, Organization};

/// A request to complete some `objective` for some `client` [`Organization`].
///
/// # Notes
///
/// * It is assumed that the [`Organization`] working on completing the `objective` is the same as
///   the one using Winvoice. This is a setting, configured elsewhere and retrieved as needed.
/// * Work which is done for a [`Job`] is tracked by [`Timesheet`]s.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Job
{
	/// The [`Organization`] who the work is being performed for.
	pub client: Organization,

	/// The [date](DateTime) which the [`Organization`] using Winvoice stopped working on the
	/// [`Job`], or [`None`] if this [`Job`] is still being worked on.
	pub date_close: Option<DateTime<Utc>>,

	/// The [date](DateTime) upon which the [`Organization`] received the `client`'s request for
	/// the work.
	pub date_open: DateTime<Utc>,

	/// The reference number of this [`Job`], which is unique among all [`Job`]s.
	///
	/// Should be generated by a database, and never altered once assigned.
	#[cfg_attr(feature = "serde", serde(skip))]
	pub id: Id,

	/// The amount of time between updates to the `time_start` and `time_end` on a [`Timesheet`].
	/// For example, if `increment` is 15m:
	///
	/// * A real `time_start` 11:14 is rounded to 12:15.
	/// * A real `time_end` 13:34 is set to 13:30.
	#[cfg_attr(feature = "serde", serde(with = "humantime_serde"))]
	pub increment: Duration,

	/// Information about how this [`Job`] will be paid for.
	pub invoice: Invoice,

	/// Important things to know about the work that has been performed, which are useful for both
	/// summarizing the events of working on a [`Job`], or leaving reminders for when refering
	/// back.
	///
	/// Markup support (if any) is dependent on the Winvoice frontend.
	pub notes: String,

	/// Desired outcomes as a result of completing work on this [`Job`].
	///
	/// Markup support (if any) is dependent on the Winvoice frontend.
	pub objectives: String,
}
