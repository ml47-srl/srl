use cell::Cell;

pub struct ScopeInterface<'a>(&'a mut Vec<Cell>);

impl<'a> ScopeInterface<'a> {
	pub fn new(x : &'a mut Vec<Cell>) -> ScopeInterface {
		ScopeInterface(x)
	}
}
