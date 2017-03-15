use app::App;
use keys;
extern crate libsrl;
use libsrl::navi::CellID;

impl App {
	// returns whether to go on
	pub fn handle_key(&mut self, key : i32) -> bool {
		match key {
			keys::QUIT => return false,
			keys::LEFT => {
				match self.prim_marker.get_left_sibling() {
					Ok(x) => self.prim_marker = x,
					Err(_) => {}
				}
			},
			keys::RIGHT => {
				match self.prim_marker.get_right_sibling() {
					Ok(x) => self.prim_marker = x,
					Err(_) => {}
				}
			},
			keys::IN => {
				match self.prim_marker.get_child(0) {
					Ok(x) => self.prim_marker = x,
					Err(_) => {}
				}
			},
			keys::OUT => {
				match self.prim_marker.get_parent() {
					Ok(x) => self.prim_marker = x,
					Err(_) => {}
				}
			},
			keys::UP => {
			},
			keys::DOWN => {
			},
			keys::DELETE => {
				// self.put_message("Can't yet delete rules", RED);
			},
			keys::EQUALS_LAW => {
			},
			keys::EQUALS_LAW_IMPL => {
			},
			keys::INEQUAL_CONSTANTS => {
			},
			keys::ADD_EQT => {
			},
			keys::RM_EQT => {
			},
			keys::SCOPE_CREATION => {
			},
			keys::IMPL_DERIVATION => {
			},
			keys::SCOPE_EXCHANGE => {
			},
			keys::CASE_CREATION => {
			},
			keys::DECLARATION => {
			},
			keys::SEC_MARKER => {
				self.sec_markers = vec![self.prim_marker.clone()];
			},
			keys::TOGGLE_SEC_MARKER => {
				if self.sec_markers.contains(&self.prim_marker) {
					let prim = self.prim_marker.clone();
					self.sec_markers.retain(|x : &CellID| *x != prim);
				} else {
					self.sec_markers.push(self.prim_marker.clone());
				}
			},
			_ => {
				// self.put_message("unknown key", WHITE);
			}
		}
		true
	}
}

