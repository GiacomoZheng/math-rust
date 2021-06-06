use super::*;
use std::fmt;

macro_rules! single_struct {
	($name: ident, $component: block, $display: block) => {
		pub struct $name;
		impl $name {
			pub fn new() -> $name {
				$name
			}
		}
		impl Complex for $name {
			fn component(&self) -> Component {
				Component::from($component())
			}
		}
		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				writeln!(f, "{}	dim: {}", stringify!($name), self.dim().unwrap())?;
				writeln!(f, "{:?}", self.component())
			}
		}
		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				writeln!(f, "{}", $display())
			}
		}
	};
	($name: ident, $component: expr, $display: expr) => {
		single_struct!($name, {|| {$component}}, {|| {$display}});
	};
}

macro_rules! countable_struct {
	($name: ident, $component: block, $display: block) => {
		pub struct $name(usize);
		impl $name {
			pub fn new(n : usize) -> $name {
				if n > 0 {
					$name(n)
				} else {
					panic!("invalid input \"{}\": genus should be positive for torus-like surface", n)
				}
			}
		}
		impl Complex for $name {
			fn component(&self) -> Component {
				Component::from($component(self.0))
			}
		}
		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				writeln!(f, "{}	dim: {}", stringify!($name), self.dim().unwrap())?;
				writeln!(f, "{:?}", self.component())
			}
		}
		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				writeln!(f, "{}", $display(self.0))
			}
		}
	};
	($name: ident, $component: expr, $display: expr) => {
		countable_struct!($name, {|g| {$component}}, {|g| {$display}});
	};
}

// 0 dim
single_struct!(Point, vec![1], "•");
#[test] fn point() {
	let p = Point;
	// dim
	assert_eq!(p.dim(), Ok(0));
	println!("{:?}", p);
	assert_eq!(p.euler(), 1);
}

// 1 dim
single_struct!(Segment, vec![2, 1], "•-•");
#[test] fn segment() {
	let p = Segment;
	// dim
	assert_eq!(p.dim(), Ok(1));
	println!("{:?}", p);
	assert_eq!(p.euler(), 1);	
}

countable_struct!(WedgeLoop, {|g| {vec![1, g]}}, {|g| {format!("•{}", "ↄ".repeat(g))}});
#[test] fn wedge_loop() {
	let one_c = WedgeLoop(1);
	let two_c = WedgeLoop(2);

	assert_eq!(one_c.component(), Component::from(vec![1, 1]));
	assert_eq!(one_c.euler(), 0);	
	assert_eq!(two_c.component(), Component::from(vec![1, 2]));
	assert_eq!(two_c.euler(), -1);	

	println!("{:?}", one_c);
	println!("{:?}", two_c);

	assert!(std::panic::catch_unwind(||{
		WedgeLoop::new(0)
	}).is_err());
}

// 2 dim
single_struct!(Circle, vec![1, 1, 1], "◉"); // + test

single_struct!(Strip, vec![2, 3, 1], "■■"); // + test

single_struct!(MobiusStrip, vec![2, 3, 1], "■▶◀■"); // + test

single_struct!(Sphere, vec![2, 3, 1], "●"); // + test

#[macro_export]
macro_rules! H2 {
    ($num : expr) => {
		if ($num >= 1) {
			use std::fmt;
			struct H2Num;
			impl Complex for H2Num {
				fn component(&self) -> Component {
					Component::from(vec![1, 2 * $num, 1])
				}
			}
			impl fmt::Debug for H2Num {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					writeln!(f, "{}-copy torus", $num)
				}
			}
			H2Num
		} else {
			panic!("invalid input \"{}\": genus should be positive for torus-like surface", $num)
		}
    }
}

#[macro_export]
macro_rules! M {
    ($num : expr) => {
		if ($num >= 1) {
			use std::fmt;
			struct MNum;
			impl Complex for MNum {
				fn component(&self) -> Component {
					Component::from(vec![1, $num, 1])
				}
			}
			impl fmt::Debug for MNum {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					writeln!(f, "{}-copy torus", $num)
				}
			}
			MNum
		} else {
			panic!("invalid input \"{}\": genus should be positive for torus-like surface", $num)
		}
    }
}