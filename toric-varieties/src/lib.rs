use general::*;

use std::fmt;
use std::ops::{Deref, DerefMut};

/// Rational Vector (vector with integer entries)
pub struct Vector(Vec<i8>);
impl MathClass for Vector {
	fn check(&self) -> Result<(), String> {
		Ok(())
	}
}
impl Vector {
	pub fn is_in(&self, c : Cone) -> bool {
		c.is_contain(self)
	}
}

impl Vector {
	pub fn dim(&self) -> usize {
		self.0.len()
	}

	pub fn new(dim : usize) -> Vector {
		Vector(vec![0 ; dim])
	}
	pub fn from(v : Vec<i8>) -> Vector {
		Vector(v)
	}
	pub fn from_slice(v : &[i8]) -> Vector {
		Vector(v.to_vec())
	}
}

/// Stringly Convex Ratioanl polyhedral cone
pub struct Cone {
	dim : usize,
	basis : Vec<Vector>
}
impl MathClass for Cone {
	fn check(&self) -> Result<(), String> {
		if self.basis.iter().all(|x| x.dim() == self.dim) {
			if self.is_strict_convex() {
				Ok(())
			} else {
				Err("Cone: This cone is not stricly convex".into())
			}
		} else {
			Err("Cone: The basis vectors should be in the same V".into())
		}
	}
}
impl Cone {
	fn is_contain(&self, v : &Vector) -> bool {
		if v.dim() == self.dim {
			unimplemented!()
		} else {
			panic!("Cone - contain: vector and cone are not in the same space")

		}
	}
}

impl Deref for Cone {
    type Target = Vec<Vector>;

    fn deref(&self) -> &Self::Target {
        &self.basis
    }
}
impl DerefMut for Cone {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.basis
    }
}
impl Cone {
	pub fn new(dim : usize) -> Cone {
		Cone {
			dim,
			basis : vec![Vector::new(dim)]
		}
	}
	pub fn from(dim : usize, vp : Vec<i8>) -> Cone {
		let mut c = Cone::new(dim);
		if vp.len() % dim == 0 {
			for v in vp.chunks(dim) {
				c.push(Vector::from_slice(v));
			}
			if c.is_strict_convex() {
				return c;
			}
		}
		panic!("Cone - from: invalid input")
	}

	fn is_strict_convex(&self) -> bool {
		unimplemented!()
	}
}
impl fmt::Debug for Cone {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// i hope it can output a basis of it
		write!(f, "a");
		unimplemented!()
    }
}

#[test] fn new_cone() {
	let c = Cone::new(2);
	assert!(c.check() == Ok(()));
}
