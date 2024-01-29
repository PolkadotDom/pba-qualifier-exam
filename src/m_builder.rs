//! In this module, we will explore the "builder" and "type-state" patterns in Rust, both of which
//! are extensively used in Substrate.
//!
//! There are ample resources about both of these online, so we will not go into too much detail
//! here. Here's one of the favorites of one of the instructors ;):
//! <https://www.youtube.com/watch?v=bnnacleqg6k>
//!
//! We will reuse the types from `e_common_traits` module and create a builder for the [`Employee`]
//! type.

use crate::e_common_traits::Employee;

/// First, let's build a naive builder. This builder should allow you to build an [`Employee`],
/// where the `name` and `uid` must be initialized, but the `experience` and `wage` can be left at
/// their default values, 0.
///
/// The final `fn build` return `Err(())` if either of `name` or `id` are not specified. See the
/// example section below.
///
/// > PS. Did you now know that the code snippets in your rust docs also compile, and are tested?
/// > now you do! :) `cargo test --doc` will run the tests.
///
/// ## Example
///
/// ```
/// # use pba_qualifier_exam::m_builder::EmployeeBuilder;
///
/// # fn main() {
/// let success = EmployeeBuilder::default().name("John".to_string()).uid(42).build();
/// assert!(success.is_ok());
///
/// let fail = EmployeeBuilder::default().name("John".to_string()).build();
/// assert!(fail.is_err());
///
/// let fail = EmployeeBuilder::default().uid(42).build();
/// assert!(fail.is_err());
/// # }
/// ```
#[derive(Clone)]
pub struct EmployeeBuilder {
	name: Option<String>,
	uid: Option<u32>,
	experience: u32,
	wage: u32,
}

impl Default for EmployeeBuilder {
	fn default() -> Self {
		Self {
			name: None,
			uid: None,
			wage: 0,
			experience: 0,
		}
	}
}

//won't change the signatures but I would make self mutable instead of cloning
impl EmployeeBuilder {
	pub fn name(self, name: String) -> Self {
		let mut self_mut = self.clone();
		self_mut.name = Some(name);
		self_mut
	}

	pub fn uid(self, uid: u32) -> Self {
		let mut self_mut = self.clone();
		self_mut.uid = Some(uid);
		self_mut
	}

	pub fn experience(self, experience: u32) -> Self {
		let mut self_mut = self.clone();
		self_mut.experience = experience;
		self_mut
	}

	pub fn wage(self, wage: u32) -> Self {
		let mut self_mut = self.clone();
		self_mut.wage = wage;
		self_mut
	}

	pub fn build(self) -> Result<Employee, ()> {
		let res = match (self.name, self.uid) {
			(Some(name), Some(uid)) => Ok(Employee {
				name: name,
				experience: self.experience,
				wage: self.wage,
				uid: uid,
			}),
			_ => Err(()),
		};
		res
	}
}

// Okay, that was good, but the unfortunate thing about the previous approach is that we will have
// no way to notify the user about their potential failure to set the name or uid, until they call
// `build` at runtime. Isn't Rust all about using the type system to move runtime errors to compile
// time?
//
// > "Rust is a language that gives you compile-time errors instead of runtime errors. It's like
// > having a spell checker for your code." - Steve Klabnik
//
// With this mindset in mind, we will introduce a new pattern called "type-state" to help us achieve
// that.

/// A unique type explicitly representing an employee that has been named.
pub struct Named {
	name: String,
}
/// A unique type explicitly representing an employee that NOT has been named.
pub struct NotNamed;

/// A unique type explicitly representing an employee that has been identified.
pub struct Identified {
	uid: u32,
}
/// A unique type explicitly representing an employee that has NOT been identified.
pub struct UnIdentified;

/// A new builder that uses the "type-state" pattern to ensure that the user has set the name and
/// uid. The main trick here is that instead of having `name` be represented by `Option<String>`, we
/// have two unique types mimicking the `Option<_>`: `Named { .. }` is analogous to `Some(_)` and
/// `UnNamed` is analogous to `None`. But, `Option<_>` is jus ONE type, but `Named` and `UnNamed`
/// are two different types.
///
/// What's the benefit of that? we can make sure that the `fn build` is only implemented if both the
/// `Name` and `Id` generics are set to `Named` and `Identified`.
///
/// > Did you know that not only you can write tests in your rust-docs, as we did in
/// > [`EmployeeBuilder`], you can also write snippets of code that MUST FAIL TO COMPILE? Cool, eh?
/// > See: <https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html>
///
/// ## Example
///
/// ```
/// use pba_qualifier_exam::m_builder::TypedEmployeeBuilder;
///
/// # fn main() {
/// // This is not a result anymore, because we guarantee at compile time that it has name and uid.
/// 	let employee =
/// 	TypedEmployeeBuilder::default().name("John".to_string()).uid(42).wage(77).build();
/// assert_eq!(employee.name, "John");
/// assert_eq!(employee.wage, 77);
/// assert_eq!(employee.uid, 42);
/// # }
/// ```
///
/// This code will simply fail to compile:
///
/// ```compile_fail
/// use pba_qualifier_exam::m_builder::TypedEmployeeBuilder;
///
/// # fn main() {
/// 	let success = TypedEmployeeBuilder::default().uid(42).build();
/// # }
/// ```
pub struct TypedEmployeeBuilder<Name, Id> {
	experience: u32,
	wage: u32,
	name: Name,
	uid: Id,
}

impl Default for TypedEmployeeBuilder<NotNamed, UnIdentified> {
	fn default() -> Self {
		Self {
			experience: 0,
			wage: 0,
			name: NotNamed,
			uid: UnIdentified,
		}
	}
}

impl<Name, Id> TypedEmployeeBuilder<Name, Id> {
	pub fn name(self, name: String) -> TypedEmployeeBuilder<Named, Id> {
		TypedEmployeeBuilder::<Named, Id> {
			experience: self.experience,
			wage: self.wage,
			name: Named { name },
			uid: self.uid,
		}
	}

	pub fn uid(self, uid: u32) -> TypedEmployeeBuilder<Name, Identified> {
		TypedEmployeeBuilder::<Name, Identified> {
			experience: self.experience,
			wage: self.wage,
			name: self.name,
			uid: Identified { uid },
		}
	}

	pub fn experience(self, experience: u32) -> Self {
		TypedEmployeeBuilder {
			experience: experience,
			wage: self.wage,
			name: self.name,
			uid: self.uid,
		}
	}

	pub fn wage(self, wage: u32) -> Self {
		TypedEmployeeBuilder {
			experience: self.experience,
			wage: wage,
			name: self.name,
			uid: self.uid,
		}
	}
}

impl TypedEmployeeBuilder<Named, Identified> {
	pub fn build(self) -> Employee {
		Employee {
			name: self.name.name,
			experience: self.experience,
			wage: self.wage,
			uid: self.uid.uid,
		}
	}
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	80
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	1
}
