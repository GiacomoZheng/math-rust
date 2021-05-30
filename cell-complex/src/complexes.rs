use super::*;
use std::fmt;

// 0 dim
pub struct Point;
impl Complex for Point {
	fn component(&self) -> Component {
		Component::from(vec![1])
	}
}
impl fmt::Debug for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "•")
    }
}
#[test] fn point() {
	let p = Point;
	// dim
	assert_eq!(p.dim(), Ok(0));
	println!("{:?}", p);
}

// 1 dim
pub struct Segment;
impl Complex for Segment {
	fn component(&self) -> Component {
		Component::from(vec![2, 1])
	}
}
impl fmt::Debug for Segment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "•-•")
    }
}
#[test] fn segment() {
	let p = Segment;
	// dim
	assert_eq!(p.dim(), Ok(1));
	println!("{:?}", p);
}

#[allow(unused_macros)]
macro_rules! WedgeS1 {
    ($num : expr) => {
		{
			struct WedgeS1Num;
			impl Complex for WedgeS1Num {
				fn component(&self) -> Component {
					Component::from(vec![1, $num])
				}
			}
			impl fmt::Debug for WedgeS1Num {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					writeln!(f, "•{}", "ↄ".repeat($num))
				}
			}

			WedgeS1Num
		}
    }
}
#[allow(non_snake_case)]
#[test] fn wedge_S1() {
	let one_c = WedgeS1![1];
	let two_c = WedgeS1![2];

	assert_eq!(one_c.component(), Component::from(vec![1, 1]));
	assert_eq!(two_c.component(), Component::from(vec![1, 2]));
	
	println!("happ");
	println!("{:?}", one_c);
	println!("{:?}", two_c);
}

// type Circle = WedgeS1![1];

// 2 dim


