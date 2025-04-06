use std::collections::{HashMap, HashSet};
use std::ops::{Add, Deref, DerefMut, Div, Mul};
use std::fmt::Debug;
use alias::{BasicObject, BasicObjectCopy};

mod just_for_fun;

pub trait AddGroup : BasicObjectCopy + Add {
	fn zero() -> Self;
	fn is_zero(&self) -> bool {
		self == &Self::zero()
	}
}
pub trait Ring : AddGroup + Mul<Output = Self> {
	fn one() -> Self;
	fn is_one(&self) -> bool {
		self == &Self::one()
	}
	fn pow(&self, n : usize) -> Self {
		(0..n).map(|e| {eprintln!("pow: {:?}", e); e}).fold(Self::one(), |acc, _x| (acc * (*self)))
	}
}
pub trait Field : Ring + Div {}
pub trait ScaMul<R : Ring> {
    fn sca_mul(self, sca: R) -> Self;
}
pub trait Module<R : Ring> : AddGroup + ScaMul<R> {}
pub trait Algebra<R : Ring> : Ring + Module<R> {}

#[derive(Debug, Clone, PartialEq, Eq, BasicObject, Hash)]
pub struct VarTable(Vec<String>);
impl Deref for VarTable {
	type Target = Vec<String>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for VarTable {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl VarTable {
	pub fn new() -> VarTable {
		VarTable(Vec::new())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, BasicObject, Copy, BasicObjectCopy, Hash)]
pub struct Var<'a> {
	index : usize,
	source : &'a VarTable
}
impl<'a> Var<'a> {
	/// Creates a new `Var` instance. To archive it, you need a `VarTable` first.
	///
	/// # Arguments
	///
	/// * `name` - The name of the variable.
	/// * `vt` - A reference to the variable table.
	///
	/// # Example
	/// ```
	/// use general::num::VarTable;
	/// use general::num::Var;
	/// let mut vt = VarTable::new();
	/// let v = Var::new(String::from("x"), &mut vt);
	///
	/// assert_eq!(v.name(), String::from("x"));
	/// ```
	pub fn new(name : String, vt : &'a mut VarTable) -> Var<'a> {
		if vt.contains(&name) {
			panic!("Variable already defined!")
		} else {
			let index = vt.len();
			vt.push(name);
			Var {
				index,
				source : vt
			}
		}
	}
	/// Get a `Var` instance from a `VarTable` instance by **index**.
	///
	/// # Arguments
	///
	/// * `index` - The index of the variable.
	/// * `vt` - A reference to the variable table.
	///
	/// # Example
	/// ```
	/// use general::num::VarTable;
	/// use general::num::Var;
	/// let mut vt = VarTable::new();
	/// vt.push(String::from("x"));
	/// let v = Var::from_index(0, &vt);
	///
	/// assert_eq!(v.name(), String::from("x"));
	/// ```
	pub fn from_index(index : usize, vt : &VarTable) -> Var {
		if index >= vt.len() {
			panic!("We did not defined so many variables")
		} else {
			Var { index, source : vt }
		}
	}
	/// Get a `Var` instance from a `VarTable` instance by **name**.
	///
	/// # Arguments
	///
	/// * `name` - The name of the variable.
	/// * `vt` - A reference to the variable table.
	///
	/// # Example
	/// ```
	/// use general::num::VarTable;
	/// use general::num::Var;
	/// let mut vt = VarTable::new();
	/// vt.push(String::from("x"));
	/// let v = Var::from_name(String::from("x"), &vt);
	///
	/// assert_eq!(v.name(), String::from("x"));
	/// ```
	pub fn from_name(name : String, vt : &VarTable) -> Var {
		if let Some((index, _)) = vt.iter().enumerate().find(|(_, x)| **x == name) {
			Var { index, source : vt }
		} else {
			panic!("Variable {name} is not defined yet")
		}
	}

	pub fn name(&self) -> String {
		self.source[self.index].clone()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, BasicObject)]
pub struct MonomialFormat<'a>(HashMap<Var<'a>, usize>);
impl<'a> MonomialFormat<'a> {
	pub fn from(m : HashMap<Var<'a>, usize>) -> MonomialFormat {
		MonomialFormat(m)
	}
}
impl<'a> Deref for MonomialFormat<'a> {
	type Target = HashMap<Var<'a>, usize>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub trait Monomial<'a, R : Ring, A : Algebra<R>> : BasicObjectCopy {
	/// return variables
	fn vars(&self) -> HashSet<Var<'a>>;
	/// return coefficients
	fn coef(&self) -> R;
	/// evaluate monomial `f`` at `x = sth`
	fn eval(&self, at : HashMap<Var, A>) -> A;
	fn into_format(self) -> MonomialFormat<'a>;
	fn into_polynomial(self) -> impl Polynomial<'a, R, A>;
}
pub trait Polynomial<'a, R : Ring, A : Algebra<R>> : BasicObject {
	/// return variables
	fn vars(&self) -> HashSet<Var<'a>>;
	/// return coefficients
	fn coef(&self, fmt : MonomialFormat) -> R;
	/// evaluate monomial `f`` at `x = sth`
	fn eval(&self, at : HashMap<Var, A>) -> A;
	fn is_monomial(&self) -> bool;
	fn try_into_monomial(self) -> Result<impl Monomial<'a, R, A>, &'static str>;
}

pub mod num {
	use crate::algebra::*;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, BasicObject, BasicObjectCopy)]
	pub struct ZZ {value : u128}
	impl Deref for ZZ {
		type Target = u128;
		fn deref(&self) -> &Self::Target {
			&self.value
		}
	}

	impl Add for ZZ { type Output = ZZ; fn add(self, other: ZZ) -> ZZ { ZZ { value: *self + *other } } }
	impl AddGroup for ZZ {
		fn zero() -> ZZ { ZZ {value : 0} }
	}

	impl Mul for ZZ { type Output = ZZ; fn mul(self, other: ZZ) -> ZZ { ZZ { value: *self * *other } } }
	impl Ring for ZZ {
		fn one() -> ZZ { ZZ {value : 1} }
	}

	impl Div for ZZ { type Output = QQ; fn div(self, other: ZZ) -> QQ { QQ { inner: (*self, *other) } } }

	// impl Numeric for ZZ {fn numeric(&self) -> f64 { self.value.into() }}

	#[derive(Debug, Clone, Eq, BasicObject, Copy, BasicObjectCopy)]
	pub struct QQ {
		inner : (u128, u128)
	}
	impl QQ {
		pub fn from_pair(a : u128, b : u128) -> QQ {
			if b == 0 {
				panic!("0 cannot be divisor")
			} else {
				QQ { inner: (a, b) }
			}
		}
	}
	impl Add for QQ {
		type Output = QQ;
		fn add(self, rhs: Self) -> Self::Output {
			QQ {inner : (self.inner.0 * rhs.inner.1 + rhs.inner.0 * self.inner.1, self.inner.1 * rhs.inner.1)}
		}
	}
	impl Mul for QQ {
		type Output = QQ;
		fn mul(self, rhs: Self) -> Self::Output {
			QQ {inner : (self.inner.0 * rhs.inner.0, self.inner.1 * rhs.inner.1)}
		}
	}
	impl Div for QQ {
		type Output = QQ;
		fn div(self, rhs: Self) -> Self::Output {
			QQ {inner : (self.inner.0 * rhs.inner.1, self.inner.1 * rhs.inner.0)}
		}
	}
	impl PartialEq for QQ {
		fn eq(&self, other: &Self) -> bool {
			let tmp = *self / *other;
			tmp.inner.0 == tmp.inner.1
		}
	}

	impl AddGroup for QQ {
		fn zero() -> Self {
			QQ {inner : (0, 1)}
		}
	}
	impl Ring for QQ {
		fn one() -> QQ {
			QQ {inner: (1, 1)}
		}
	}
	impl Field for QQ {}

	// impl Numeric for QQ {fn numeric(&self) -> f64 { (self.inner.1 / self.inner.0).into() }}

	#[test] fn field() {
		let a = ZZ {value : 2};
		let b = ZZ {value : 4};
		let q = a / b;
	
		assert_eq!(q, QQ { inner: (1, 2) })
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, BasicObject)]
	struct V {inner : (QQ, QQ)}
	impl BasicObjectCopy for V {}

	impl ScaMul<QQ> for V {
		fn sca_mul(self, sca: QQ) -> Self {
			V {inner : (sca * self.inner.0, sca *self.inner.1)}
		}
	}

	impl Add for V {
		type Output = V;
		fn add(self, rhs: Self) -> Self::Output {
			V {inner : (self.inner.0 + rhs.inner.0, self.inner.1 + rhs.inner.1)}
		}
	}
	impl AddGroup for V {
		fn zero() -> V { V { inner: (QQ::zero(), QQ::zero()) } }
	}
	impl Module<QQ> for V {}

	#[test] fn vec_qq2() {
		let v = V { inner : (QQ {inner : (1, 2)}, QQ { inner : (7, 1)})};

		assert_eq!(v.sca_mul(QQ { inner: (5, 1) }), V { inner : (QQ {inner : (5, 2)}, QQ { inner : (35, 1)})})
	}

	impl ScaMul<QQ> for QQ {
		fn sca_mul(self, sca: QQ) -> Self {
			sca * self
		}
	}
	impl Module<QQ> for QQ {}
	impl Algebra<QQ> for QQ {}

	#[derive(Debug, Clone, PartialEq, Eq, BasicObject, Copy, BasicObjectCopy)]
	pub struct QQMonomial<'a> {
		degree : usize,
		coef : QQ,
		var : Var<'a>
	}
	impl<'a> QQMonomial<'a> {
		pub fn new(var : Var<'a>) -> QQMonomial<'a> {
			QQMonomial {
				degree : 0,
				coef : QQ::zero(),
				var 
			}
			// TODO check coef not zero unless degree is zero
		}
	}
	impl<'a> Monomial<'a, QQ, QQ> for QQMonomial<'a> {
		fn vars(&self) -> HashSet<Var<'a>> {
			let mut m = HashSet::new();
			m.insert(self.var);
			m
		}
		fn coef(&self) -> QQ {
			self.coef
		}
		fn eval(&self, at : HashMap<Var, QQ>) -> QQ {
			if let Some(value) = at.get(&self.var) {
				self.coef * value.pow(self.degree)
			} else {
				panic!("no such a variable!")
			}
		}
		fn into_format(self) -> MonomialFormat<'a> {
			let mut m = HashMap::new();
			m.insert(self.var, self.degree);
			MonomialFormat(m)
		}
		fn into_polynomial(self) -> impl Polynomial<'a, QQ, QQ> {
			let mut v = vec![QQ::zero()].repeat(self.degree);
			v.push(self.coef);
			QQPolynomial::from_vec(self.var, v)
			// TODO test it
		}
	}

	#[derive(Debug, Clone, Eq, BasicObject)]
	pub struct QQPolynomial<'a> {
		inner : Vec<QQ>,
		var : Var<'a>
	}
	impl<'a> QQPolynomial<'a> {
		pub fn from_vec(var : Var<'a>, mut v : Vec<QQ>) -> QQPolynomial<'a> {
			while Some(&(QQ::zero())) == v.last() {
				v.pop();
			}
			QQPolynomial {
				inner : v,
				var
			}
		}
		pub fn from_vec_int(var : Var<'a>, mut v : Vec<u128>) -> QQPolynomial<'a> {
			while let Some(&0) = v.last() {
				v.pop();
			}
			QQPolynomial {
				inner : v.into_iter().map(|i| QQ::from_pair(i, 1)).collect(),
				var
			}
		}
		// TODO: parse 字符串来得到 QQPolynomial

		/// to check whether the polynomial is already simplied
		fn check_last_zero(&self) {
			if let Some(e) = self.inner.last() {
				if e.is_zero() {
					panic!("polynomials are not simplied, check the constructors");
				}
			}
		}
	}
	
	impl<'a> PartialEq for QQPolynomial<'a> {
		fn eq(&self, other: &Self) -> bool {
			self.check_last_zero();
			other.check_last_zero();

			if self.inner.len() != other.inner.len() {
				false
			} else {
				self.inner.iter().zip(other.inner.iter())
					.all(|(s, o)| s == o)
			}
		}
	}
	#[test] fn qq_polynomial_eq() {
		let mut vt = VarTable::new();
		let x = Var::new(String::from("x"), &mut vt);
		let f1 = QQPolynomial::from_vec_int(x, vec![1, 1, 0]);
		let f2 = QQPolynomial::from_vec_int(x, vec![1, 1]);
		let f3 = QQPolynomial::from_vec_int(x, vec![1, 1, 2]);
		let f4 = QQPolynomial::from_vec_int(x, vec![1, 1, 1]);
		assert_eq!(f1, f2);
		assert_ne!(f2, f3);
		assert_ne!(f3, f4);
	}
	impl<'a> Polynomial<'a, QQ, QQ> for QQPolynomial<'a> {
		fn vars(&self) -> HashSet<Var<'a>> {
			HashSet::from([self.var.clone()])
		}
		fn coef(&self, fmt : MonomialFormat) -> QQ {
			if let Some(index) = fmt.get(&self.var) {
				self.inner[*index]
			} else {
				panic!("no such a variable!")
			}
			// TODO: fmt应该是一个monomial
		}
		fn eval(&self, at : HashMap<Var, QQ>) -> QQ {
			if let Some(value) = at.get(&self.var) {
				self.inner.iter().enumerate()
					.map(|(n, s)| (*s) * value.pow(n))
					// .map(|e| {!("eval: {:?}", e); e})
					.fold(QQ::zero(), |acc, x| acc + x)
			} else {
				panic!("no such a variable!")
			}
			// TODO: at 应该是单元素的
		}

		fn is_monomial(&self) -> bool {
			self.inner.iter().filter(|e| !e.is_zero()).collect::<Vec<_>>().len() == 1
		}
		fn try_into_monomial(self) -> Result<impl Monomial<'a, QQ, QQ>, &'static str> {
			if self.is_monomial() {
				let (n, e) = self.inner.iter().enumerate().filter(|(_, e)| !e.is_zero()).next().unwrap();
				Ok(QQMonomial {
					var : self.var,
					degree : n,
					coef : *e
				})
			} else {
				Err("it is not a monomial")
			}
		}
	}

	#[test] fn qq_polynomial() {
		let mut vt = VarTable::new();
		let var = Var::new("x".into(), &mut vt);

		// eval
		let f = QQPolynomial::from_vec_int(var, vec![0, 1, 2, 3, 4]); // 
		let fp = |x : u128| {0 + 1 * x + 2 * x.pow(2) + 3 * x.pow(3) + 4 * x.pow(4)};
		assert_eq!(f.eval(HashMap::from([(var , QQ::from_pair(5, 1))])), QQ::from_pair(fp(5), 1));

		// try_into_monomial
		let f = QQPolynomial::from_vec_int(var, vec![0, 1, 1, 0, 0]);
		assert!(f.try_into_monomial().is_err());
		let f = QQPolynomial::from_vec_int(var, vec![0, 0, 1, 0, 0]);
		let fm = f.try_into_monomial().unwrap();
		assert_eq!(fm.coef(), QQ::one());
		assert_eq!(fm.eval(HashMap::from([(var , QQ::from_pair(5, 1))])), QQ::from_pair(25, 1));
	}
}
