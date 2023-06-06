use core::fmt::{Display, Formatter, Result};

use super::Contact;

impl Display for Contact
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}: {}", self.label, self.kind)
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::Contact;
	use crate::{ContactKind, Location};

	#[test]
	fn display()
	{
		let earth_view = Location { name: "Earth".into(), ..Default::default() };

		let usa_view =
			Location { name: "USA".into(), outer: Some(earth_view.into()), ..Default::default() };

		let arizona_view =
			Location { name: "Arizona".into(), outer: Some(usa_view.into()), ..Default::default() };

		let phoenix_view = Location {
			name: "Phoenix".into(),
			outer: Some(arizona_view.into()),
			..Default::default()
		};

		let street_view = Location {
			name: "1337 Some Street".into(),
			outer: Some(phoenix_view.into()),
			..Default::default()
		};

		assert_eq!(
			Contact { kind: ContactKind::Address(street_view), label: "Office".into() }.to_string(),
			"Office: 1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
		assert_eq!(
			Contact { kind: ContactKind::Email("foo@bar.io".into()), label: "Email".into() }
				.to_string(),
			"Email: foo@bar.io"
		);
		assert_eq!(
			Contact {
				kind: ContactKind::Phone("1-603-555-5555".into()),
				label: "Cellphone".into(),
			}
			.to_string(),
			"Cellphone: 1-603-555-5555"
		);
	}
}
