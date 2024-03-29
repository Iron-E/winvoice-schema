mod default;
mod display;
mod exchange;
mod restorable_serde;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use money2::{Decimal, Money};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Employee, Expense, Id, Job};

/// A [`Timesheet`] contains information about continuous periods of work on a [`Job`].
///
/// A [`Job`] may contain multiple [`Timesheet`]s, with different and/or duplicate
/// `employee`s.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timesheet
{
	/// The reference number of this [`Timesheet`], which is unique among all [`Timesheet`]s.
	///
	/// Should be generated by a database, and never altered once assigned.
	#[cfg_attr(feature = "serde", serde(skip))]
	pub id: Id,

	/// The [`Employee`] who performed this work. If multiple [`Employee`]s collaborated on the
	/// work, they should create separate [`Timesheet`]s.
	pub employee: Employee,

	/// Business-related, non-[hourly-rate](super::Invoice)-related [`Expense`]s which were
	/// incurred during this time.
	pub expenses: Vec<Expense>,

	/// The [`Job`] which was worked on.
	pub job: Job,

	/// The time which this period of work began.
	pub time_begin: DateTime<Utc>,

	/// The time which this period of work ended. If the period of work is ongoing, then this field
	/// will be [`None`].
	pub time_end: Option<DateTime<Utc>>,

	/// A summary of what work was performed.
	///
	/// Markup support (if any) is dependent on the CLInvoice frontend.
	pub work_notes: String,
}

impl Timesheet
{
	/// Get the amount of [`Money`] which is owed by the [client](crate::Organization) for work done
	/// on this [`Timesheet`].
	///
	/// `exchange_rates` may be [`None`] in the case that the [hourly rate](super::Invoice) is in
	/// the same [`Currency`] as _every one_ of the `expenses` on this [`Timesheet`].
	///
	/// # Panics
	///
	/// When [`Money::add_assign`] does.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_schema::{chrono::Utc, Currency, Expense, Money, Timesheet};
	/// # use pretty_assertions::assert_eq;
	///
	/// let timesheets = [
	///   Timesheet {
	///     time_begin: Utc::today().and_hms(2, 0, 0),
	///     time_end: Some(Utc::today().and_hms(2, 30, 0)),
	///     ..Default::default()
	///   },
	///   Timesheet {
	///     expenses: vec![Expense {
	///       cost: Money::new(20_00, 2, Currency::Usd),
	///       ..Default::default()
	///     }],
	///     time_begin: Utc::today().and_hms(3, 0, 0),
	///     time_end: Some(Utc::today().and_hms(3, 30, 0)),
	///     ..Default::default()
	///   },
	/// ];
	///
	/// assert_eq!(
	///   Timesheet::total_all(&timesheets, Money::new(20_00, 2, Currency::Usd)),
	///   Money::new(4000, 2, Currency::Usd),
	/// );
	/// ```
	///
	/// ```rust,should_panic
	/// # use clinvoice_schema::{chrono::Utc, Currency, Expense, Money, Timesheet};
	/// # use pretty_assertions::assert_eq;
	/// #
	/// # let timesheets = [
	/// #   Timesheet {
	/// #     time_begin: Utc::today().and_hms(2, 0, 0),
	/// #     time_end: Some(Utc::today().and_hms(2, 30, 0)),
	/// #     ..Default::default()
	/// #   },
	/// #   Timesheet {
	/// #     expenses: vec![Expense {
	/// #       cost: Money::new(20_00, 2, Currency::Usd),
	/// #       ..Default::default()
	/// #     }],
	/// #     time_begin: Utc::today().and_hms(3, 0, 0),
	/// #     time_end: Some(Utc::today().and_hms(3, 30, 0)),
	/// #     ..Default::default()
	/// #   },
	/// # ];
	/// let _ = Timesheet::total_all(&timesheets, Money::new(20_00, 2, Currency::Eur));
	/// ```
	pub fn total_all(timesheets: &[Self], hourly_rate: Money) -> Money
	{
		lazy_static! {
			static ref SECONDS_PER_HOUR: Decimal = 3600.into();
		}

		let mut total = Money::new(0, 0, hourly_rate.currency);
		timesheets.iter().filter(|timesheet| timesheet.time_end.is_some()).for_each(|timesheet| {
			total.amount += hourly_rate.amount *
				(Decimal::from(
					timesheet
						.time_end
						.expect("Filters should have assured that `Timesheet`s have an end time")
						.signed_duration_since(timesheet.time_begin)
						.num_seconds(),
				) / *SECONDS_PER_HOUR);

			timesheet.expenses.iter().for_each(|expense| total += expense.cost);
		});

		total.amount.rescale(2);
		total
	}
}
