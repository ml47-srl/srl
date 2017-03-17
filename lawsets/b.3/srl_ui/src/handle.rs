use app::App;
use keys;
extern crate libsrl;
use libsrl::navi::CellID;
use libsrl::cell::Cell;

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
				match self.db.delete_rule(self.prim_marker.get_rule_id()) {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
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
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::EQUALS_LAW_IMPL => {
				let len = self.sec_markers.len();
				if len != 1 {
					self.put_error(format!("There are {} secondary markers, but only 1 is allowed", len));
					return true;
				}
				let result = self.db.equals_law_impl(self.prim_marker.clone(), self.sec_markers[0].clone());
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::INEQUAL_CONSTANTS => {
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.inequal_constants(self.prim_marker.clone());
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::ADD_EQT => {
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.add_eqt(self.prim_marker.clone());
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::RM_EQT => {
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.rm_eqt(self.prim_marker.clone());
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::SCOPE_INSERTION => {
				let input = self.read_input();
				let cell = match Cell::by_string(&input) {
					Ok(x) => x,
					Err(srl_error) => {
						self.put_error(srl_error.to_string());
						return true;
					}
				};
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.scope_insertion(self.prim_marker.clone(), cell);
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::SCOPE_CREATION => {
				let mut vec = Vec::new();
				let prim_indices = self.prim_marker.get_indices();
				for sec in self.sec_markers.clone() {
					let mut sec_indices = sec.get_indices();
					for index in prim_indices.clone() {
						if index != sec_indices.remove(0) {
							self.put_error("sec_marker is out of prim_marker".to_string());
							return true;
						}
					}
					vec.push(sec_indices);
				}
				let result = self.db.scope_creation(self.prim_marker.clone(), vec);
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::IMPL_DERIVATION => {
				let len = self.sec_markers.len();
				if len != 1 {
					self.put_error(format!("There are {} secondary markers, but only 1 is allowed", len));
					return true;
				}
				let result = self.db.implications_derivation(self.prim_marker.clone(), self.sec_markers[0].clone());
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::SCOPE_EXCHANGE => {
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.scope_exchange(self.prim_marker.clone());
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::CASE_CREATION => {
				let input = self.read_input();
				let cell = match Cell::by_string(&input) {
					Ok(x) => x,
					Err(srl_error) => {
						self.put_error(srl_error.to_string());
						return true;
					}
				};
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.case_creation(self.prim_marker.clone(), cell);
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
			},
			keys::DECLARATION => {
				let input = self.read_input();
				let len = self.sec_markers.len();
				if len != 0 {
					self.put_error(format!("There are {} secondary markers, but none allowed", len));
					return true;
				}
				let result = self.db.declaration(self.prim_marker.clone(), &input);
				match result {
					Ok(_) => {},
					Err(srl_error) => self.put_error(srl_error.to_string())
				}
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

