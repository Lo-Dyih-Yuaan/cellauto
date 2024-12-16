#[macro_use]
extern crate cellauto;
use cellauto::rules;
use cellauto::pattern::Pattern;
use cellauto::golly;
use std::fs;
use std::convert::TryFrom;

#[test]
fn test_hutton32() {
	// <http://gollygang.github.io/ruletablerepository/downloads/Hutton32.table>
	let tree = fs::read_to_string("tests/Hutton32.rule").expect("Could not read file");
	let (rf, _, _) = rule!{Hutton 32};
	let states = concat!{
		"USS0S1S00S01S10S11S000",
		"To>_To^_To<_Tov_",
		"To>~To^~To<~Tov~",
		"Ts>_Ts^_Ts<_Tsv_",
		"Ts>~Ts^~Ts<~Tsv~",
		"C__C_~C~_C~~C-C|C+"
	};
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_von_neumann_tree(&rf);
	assert_eq!(format!("@RULE Hutton32\n\n@TREE\n\n{:?}", rt), tree);
}