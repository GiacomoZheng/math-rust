use std::fmt;

/// n-simplex, up to homeomorphic
pub struct Simplex(usize);
impl fmt::Debug for Simplex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-simplex", self.0)
    }
}

mod tools;
use tools::VecTail;
use std::ops::Deref;
#[derive(PartialEq)]
pub struct Component { // formed by simplex
	inner : VecTail<usize>
}
impl Deref for Component {
	type Target = VecTail<usize>;

	fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Component {
	pub fn from(v: Vec<usize>) -> Component {
		Component {
			inner : VecTail::from(v, 0)
		}
	}
}
impl fmt::Debug for Component {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (dim, num) in self.inner.iter_finite().enumerate() {
			writeln!(f, "{:?}\t{}", Simplex(dim), num)?
		}
		write!(f, "")
    }
}
// impl fmt::Display for Component {

// }
#[test] fn print_component() {
	let c = Component::from(vec![1,2,3,4]);
	println!("{:?}", c);
}

/// Cell complex or CW complex
pub trait Complex {
	fn component(&self) -> Component;
	fn is_empty(&self) -> bool {
		self.component().is_empty()
	}
	fn dim(&self) -> Result<usize, ()> {
		if self.is_empty() {
			Err(())
		} else {
			Ok(self.component().len() - 1)
		}
	}
}
pub mod complexes;
pub use complexes::*;

// In this package, I'll regard simplex and complex as totally different things

