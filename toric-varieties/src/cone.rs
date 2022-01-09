use general::*;

use std::fmt;
use std::ops::{Deref, DerefMut};
// use std::rc::Rc;

/// Euclidean space of this dim
pub struct Space {
	dim : usize,
}
impl MathClass for Space {
	fn check(&self) -> Result<(), String> {
		Ok(())
	}
}

impl Space {
	pub fn new(dim : usize) -> Space {
		Space {
			dim,
		}
	}
}
impl Space { // for vector
	pub fn vector_from(&self, v : Vec<i8>) -> Vector {
		let v = Vector {
			dim : self.dim,
			inner : v
		};
		if let Err(s) = v.check() {
			panic!("Space: {}", s);
		}
		v
	}

	fn is_vectors_independent(&self, vs : &Vec<Vector>) -> bool {
		unimplemented!()
	}

	fn to_vectors_independent(&self, vs : Vec<Vector>) -> Vec<Vector> {
		let mut basis = Vec::new();
		for term in vs.into_iter() {
			basis.push(term);
			if !self.is_vectors_independent(&basis) {
				basis.pop();
			}
		}
		basis
	}
}
impl Space { // for cone
	fn cone_new(&self) -> Cone {
		Cone {
			dim : self.dim,
			basis : vec![]
		}
	}

	pub fn cone_zero(&self) -> Cone {
		self.cone_new()
	}
	
	fn cone_from_raw(&self, vp : Vec<i8>) -> Cone {
		let mut c = self.cone_new();
		if vp.len() % self.dim == 0 {
			for v in vp.chunks(self.dim) {
				c.push(self.vector_from(v.to_vec()));
			}
			if let Err(s) = c.check() {
				panic!("Space: cone_from_raw: {}", s)
			}
			c
		} else {
			panic!("Space: cone_from_raw: invalid input")
		}
	}
	fn to_cone_with_basis(&self, c : Cone) -> Cone {
		Cone {
			dim : self.dim,
			basis : self.to_vectors_independent(c.basis)
		}
	}
	pub fn cone_from(&self, vp : Vec<i8>) -> Cone {
		self.to_cone_with_basis(self.cone_from_raw(vp))
	}
}
#[test] fn cone_basis() {
	let s = Space::new(2);
	let c = s.cone_from(vec![
		1, 0,
		0, 1,
		1, 1
	]);
	assert!(c.basis.len() == 2);
}
impl Space { // for HalfSpace
	pub fn half_space_new(&self, dim : usize) -> Cone {
		Cone {
			dim,
			basis : vec![]
		}
	}
	
	pub fn half_space_from(&self, dim : usize, vp : Vec<i8>) -> Cone {
		let mut c = self.half_space_new(dim);
		if vp.len() % dim == 0 {
			for v in vp.chunks(dim) {
				c.push(self.vector_from(v.to_vec()));
			}
			if let Err(s) = c.check() {
				panic!("Space: half_space_from: {}", s)
			}
			c
		} else {
			panic!("Space: half_space_from: invalid input")
		}
	}
}

/// Rational Vector (vector with integer entries)
pub struct Vector {
	dim : usize,
	inner : Vec<i8>
}
impl MathClass for Vector {
	fn check(&self) -> Result<(), String> {
		if self.dim == self.inner.len() {
			Ok(())
		} else {
			Err("Vector: this vector has wrong dim".into())
		}
	}
}
impl Vector {
	pub fn is_nonzero(&self) -> bool {
		self.inner.iter().any(|x| *x != 0)
	}
	pub fn is_in(&self, c : Cone) -> bool {
		c.is_contain(self)
	}
}
#[test] fn zero_vector() {
	let v = Vector {
		dim : 3, inner : vec![0,1,0]
	};
	assert!(v.is_nonzero())
}

/// Half Space defined by the normal vector
pub struct HalfSpace {
	dim : usize,
	normal_vector : Vector
}
impl MathClass for HalfSpace {
	fn check(&self) -> Result<(), String> {
		if self.dim == self.normal_vector.dim {
			if self.normal_vector.is_nonzero() {
				Ok(())
			} else {
				Err("HalfSpace: normal vector cannot be zero".into())
			}
		} else {
			Err("HalfSpace: normal vector has wrong dim".into())
		}
	}
}

/// Stringly Convex Ratioanl polyhedral cone
pub struct Cone {
	dim : usize,
	basis : Vec<Vector>
}
impl MathClass for Cone {
	fn check(&self) -> Result<(), String> {
		if self.basis.iter().all(|x| x.dim == self.dim) {
			if self.is_convex() {
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
		if v.dim == self.dim {
			unimplemented!()
		} else {
			panic!("Cone - contain: vector and cone are not in the same space")

		}
	}
	pub fn is_convex(&self) -> bool {
		unimplemented!()
	}
	pub fn is_strictly_convex(&self) -> bool {
		unimplemented!()
	}
	
}
#[test] fn convex() {
	let s = Space::new(2);
	let c = s.cone_from_raw(vec![
		1, 4,
		3, 5,
		2, 4,
		-1, 1,
		1, 7
	]);
	assert!(c.check() == Ok(()));
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
impl fmt::Debug for Cone {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// i hope it can output a basis of it
		unimplemented!()
    }
}


