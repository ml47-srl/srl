use spec::Spec;
use libsrl::db::Database;
use libsrl::cell::Cell;

pub enum Action {
	EqualsLaw(Spec, Spec),
	EqualsLawImpl(Spec, Spec),
	InequalConstants(Spec),
	AddEqt(Spec),
	RmEqt(Spec),
	ScopeInsertion(Spec, Spec),
	ScopeCreation(Spec, Spec),
	ImplicationsDerivation(Spec, Spec),
	ScopeExchange(Spec),
	CaseCreation(Spec, Spec),
	Declaration(Spec)
}

impl Action {
	pub fn gen() -> Action {
		panic!("return random Action")
	}

	pub fn call(&self, target : &Cell, db : &mut Database) -> u32 { // returns the amount of created rules
		match &self {
			&&Action::EqualsLaw(ref spec1, ref spec2) => {
				let mut counter = 0;
				let ids1 = spec1.get_cell_ids(db, target);
				let ids2 = spec2.get_cell_ids(db, target);
				for id1 in &ids1 {
					for id2 in &ids2 {
						if db.equals_law(id1.clone(), id2.clone()).is_ok() {
							counter += 1;
						}
					}
				}
				return counter;
			},
			&&Action::EqualsLawImpl(ref spec1, ref spec2) => {
			},
			&&Action::InequalConstants(ref spec) => {
			},
			&&Action::AddEqt(ref spec) => {
			},
			&&Action::RmEqt(ref spec) => {
			},
			&&Action::ScopeInsertion(ref spec1, ref spec2) => {
			},
			&&Action::ScopeCreation(ref spec1, ref spec2) => {
			},
			&&Action::ImplicationsDerivation(ref spec1, ref spec2) => {
			},
			&&Action::ScopeExchange(ref spec) => {
			},
			&&Action::CaseCreation(ref spec1, ref spec2) => {
			},
			&&Action::Declaration(ref spec) => {
			}
		}
		panic!("")
	}
}
