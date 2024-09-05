//! This portion will test your familiarity with some of Rust's common traits and your ability to
//! implement those traits in interesting ways. You need to remove all of the `todo!()`s. Most of
//! them will need to be replaced by some code, but some may simply be deleted.

// NOTE: You will need to `use` something from the standard library to implement `Ord` and
// `PartialOrd` here.

/// A record of an employee at a particular company
#[derive(Debug)]
pub struct Employee {
	/// The name the person likes to be called. Doesn't have to be their "passport name"
	pub name: String,
	/// Amount of experience (in months) the person has working at this company
	pub experience: u32,
	/// Hourly wage paid to this employee
	pub wage: u32,
	/// Unique identifier for this employee
	pub uid: u32,
}

// We want to consider two employee instances equal iff they have the same `uid`.

impl PartialEq for Employee {
	fn eq(&self, other: &Self) -> bool {
		self.uid == other.uid
	}
}
impl Eq for Employee {}

// We want to sort employees. First and foremost, employees are equal if they have the same
// `uid`, as explained above. For employees who are not equal, we sort by the value they
// bring to the company. Value is defined as the quotient of the experience they've acquired
// at the company divided by their wage. Use integer division for the purpose of this calculation.
use std::cmp::*;
impl PartialOrd for Employee {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self == other { return Some(Ordering::Equal); }
		let val1 = self.experience/self.wage;
		let val2 = other.experience/other.wage;
		Some(val1.cmp(&val2))
	}
}

impl Ord for Employee {
	fn cmp(&self, other: &Self) -> Ordering {
		if self == other { return Ordering::Equal; }
		let val1 = self.experience/self.wage;
		let val2 = other.experience/other.wage;
		val1.cmp(&val2)
	}
}

// We want to parse employee information from a string data
// The string data should be comma separated. Here are a few examples:
//
// * "Billy, 4, 5, 345" - Billy has been working here for 4 months and earns 5 token per hour. She
//   is employee #345
// * "Jose, 12, 6, 1" - Jose has been working here for 1 year (12 months) and earns 6
// tokens per hour. He is employee #1
//
// Any strings with the wrong number of commas or numbers too big for `u32` should return `Err(_)`
// where the error message may be anything.

impl TryFrom<String> for Employee {
	type Error = &'static str;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		//split string
		let parts: Vec<&str> = value.split(',').map(str::trim).collect();
		//extract	
		match parts.as_slice() {
			[name, exp, wage, uid] => {
				return Ok(Employee {
					name: (*name).to_string(),
					experience: exp.parse::<u32>().map_err(|_| "string to u32 parse error")?,
					wage: wage.parse::<u32>().map_err(|_| "string to u32 parse error")?,
					uid: uid.parse::<u32>().map_err(|_| "string to u32 parse error")?
				})
			},
			_ => return Err("Invalid format"),
		};
	}
}

// We also want to convert employees back into strings in the same format as above.
impl From<Employee> for String {
	fn from(e: Employee) -> Self {
		format!("{}, {}, {}, {}", e.name, e.experience, e.wage, e.uid)
	}
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	177
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn employee_from_string_success() {
		let s = String::from("Billy, 4, 5, 345");
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};

		assert_eq!(billy, s.try_into().unwrap())
	}

	#[test]
	fn employee_to_string_success() {
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};

		assert_eq!(String::from("Billy, 4, 5, 345"), String::from(billy))
	}

	#[test]
	fn employee_ord() {
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};
		let susie = Employee {
			name: String::from("Susie"),
			experience: 5,
			wage: 5,
			uid: 347,
		};

		assert!(susie > billy);
	}

	#[test]
	fn employee_neq() {
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};
		let susie = Employee {
			name: String::from("Susie"),
			experience: 5,
			wage: 5,
			uid: 347,
		};

		assert!(susie != billy);
	}
}
