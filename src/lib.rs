#[derive(Debug)]
pub struct Chunk {}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk {}
	}
}

impl std::fmt::Display for Chunk {
	fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		return Ok(());
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn exploration() {
		assert_eq!(2 + 2, 4);
	}
}
