pub trait MathClass {
	/// criteria for a math concept
	fn check(&self) -> Result<(), String>;
}

