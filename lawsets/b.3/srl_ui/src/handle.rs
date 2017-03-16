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
				let new = match self.prim_marker.get_right_sibling() {
					Ok(x) => x,
					Err(_) => return true
				};
				if new.get_path(&self.db.get_rules()).is_ok() {
					self.prim_marker = new;
				}
			},
			keys::IN => {
				let new = match self.prim_marker.get_child(0) {
					Ok(x) => x,
					Err(_) => return true
				};
				if new.get_path(&self.db.get_rules()).is_ok() {
					self.prim_marker = new;
				}
			},
			keys::OUT => {
				match self.prim_marker.get_parent() {
					Ok(x) => self.prim_marker = x,
					Err(_) => {}
				}
			},
			keys::UP => {
				if self.prim_marker.get_rule_id() == 0 {
					return true;
				}
				self.prim_marker = CellID::create(self.prim_marker.get_rule_id() - 1, self.prim_marker.get_indices());
				while self.prim_marker.get_path(&self.db.get_rules()).is_err() {
					self.prim_marker = self.prim_marker.get_parent().unwrap();
				}
			},
			keys::DOWN => {
				if self.prim_marker.get_rule_id() == self.db.count_rules() - 1 {
					return true;
				}
				self.prim_marker = CellID::create(self.prim_marker.get_rule_id() + 1, self.prim_marker.get_indices());
				while self.prim_marker.get_path(&self.db.get_rules()).is_err() {
					self.prim_marker = self.prim_marker.get_parent().unwrap();
				}
			},
			keys::DELETE => {
				self.put_error("Can't yet delete rules".to_string());
			},
			keys::EQUALS_LAW => {
				let len = self.sec_markers.len();
				if len != 1 {
					self.put_error(format!("There are {} secondary markers, but only 1 is allowed", len));
					return true;
				}
				let result = self.db.equals_law(self.prim_marker.clone(), self.sec_markers[0].clone());
				match result {
					Ok(_) => {},
					Err(x) => self.put_error(x.to_string())
				}
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
				self.put_note(format!("unknown key {}", key));
			}
		}
		true
	}
}

