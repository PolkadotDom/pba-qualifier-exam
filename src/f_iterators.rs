//! This portion of the exam tests your abilities to work with iterators and their functional-style
//! methods.
//!
//! Throughout this portion of the test, you may refer to <https://doc.rust-lang.org/std/iter/trait.Iterator.html>
//! and other docs about iterators. You may NOT look up specific implementations for these problems
//! in Rust or any other Functional language.
//!
//! If you find that you simply cannot write these methods in the functional style using iterator
//! methods, writing them in the imperative style with loops will still earn partial credit.

/// This function takes an iterator of u32 values, squares each value, and returns the sum of the
/// squares. You may assume that no individual square, nor the entire sum, overflows the u32 type.
pub fn sum_of_squares(vals: impl Iterator<Item = u32>) -> u32 {
	vals.map(|x| x*x).sum()
}

/// This function takes an iterator of i32 values, calculates the absolute value of each, and throws
/// away any values that are greater than 100. The remaining positive values are returned as an
/// iterator of u32s.
pub fn bounded_absolute_values(vals: impl Iterator<Item = i32>) -> impl Iterator<Item = u32> {
	//could you filter map but I like the one liner
	vals.map(|x| x.abs() as u32).filter(|x| *x <= 100)
}

// We allow `unused_mut` only so that there is no build warning on the starter code.
// You should remove this line once you have completed the following function

/// This function takes an iterator of u32 values. The first value in the iterator, call it n, is
/// special: it represents the maximum count of the resultant iterator. Once n is known, create an
/// iterator that yields the first n even values from the remainder of the input iterator.
///
/// If the input iterator is empty, return None
/// If there are fewer than n even values left in the input, return as many as possible
pub fn first_n_even(mut vals: impl Iterator<Item = u32>) -> Option<impl Iterator<Item = u32>> {
	let n = vals.next()?;
	let evens = vals.filter(|x| x % 2 == 0).take(n as usize);
	Some(evens)
}

/// Return an "infinite" iterator that yields the squares of the whole numbers.
/// For example, the first few values should be 0, 1, 4, 9, 16, 25, ...
///
/// The iterator should be bounded only by the u32 type, not by your code
pub fn square_whole_numbers() -> impl Iterator<Item = u32> {
	let mut ind: u32 = 0;
	std::iter::from_fn(move || {
		let sq = ind.checked_pow(2);
		match sq {
			Some(res) => {
				ind = ind + 1; //no need to check, as u32 overflow will happen first
				Some(res)
			},
			_ => None
		}
	})
}

/// An iterator that generates the Fibonacci sequence.
#[derive(Default)]
pub struct Fibonacci {
	/// The most recent value this iterator has yielded
	prev: Option<u32>,
	/// The second most recent value that this iterator has yielded
	prev_prev: Option<u32>,
}

impl Iterator for Fibonacci {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		//get next value
		let next_val = match (self.prev, self.prev_prev) {
			(Some(prev), Some(prev_prev)) => prev.checked_add(prev_prev),
			(Some(prev), None) => Some(1),
			_ => Some(0),
		};
		//slide values over
		if let Some(val) = next_val {
			self.prev_prev = self.prev;
			self.prev = Some(val);
		}
		next_val
	}
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	200
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
	fn sum_of_squares_1() {
		let initial = [1u32, 2, 3].into_iter();

		assert_eq!(14, sum_of_squares(initial));
	}

	#[test]
	fn bounded_absolute_values_1() {
		let initial = [1, 5, -5, 101, -200, 9, 0].into_iter();
		let expected = vec![1u32, 5, 5, 9, 0];

		assert_eq!(
			expected,
			bounded_absolute_values(initial).collect::<Vec<_>>()
		);
	}

	#[test]
	fn first_n_even_1() {
		let initial = [3u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
		let expected = vec![2u32, 4, 6];

		assert_eq!(expected, first_n_even(initial).unwrap().collect::<Vec<_>>());
	}

	#[test]
	fn first_n_even_2() {
		let initial = [3u32, 1, 3, 5, 7, 9].into_iter();

		assert_eq!(
			Vec::<u32>::new(),
			first_n_even(initial).unwrap().collect::<Vec<_>>()
		);
	}

	#[test]
	fn square_whole_numbers_1() {
		assert_eq!(
			vec![0, 1, 4, 9, 16, 25],
			square_whole_numbers().take(6).collect::<Vec<_>>()
		);
	}

	#[test]
	fn fibonacci_1() {
		let fib = Fibonacci::default();
		let expected = vec![0u32, 1, 1, 2, 3, 5, 8];

		assert_eq!(expected, fib.take(7).collect::<Vec<_>>());
	}
}
