#[macro_use]
extern crate cellauto;
use cellauto::rules;
use cellauto::pattern::Pattern;
use cellauto::golly;
use std::convert::TryFrom;

#[test]
fn test_life() {
	let tree = "num_states=2
num_neighbors=8
num_nodes=32
1 0 0
2 0 0
1 0 1
2 0 2
3 1 3
1 1 1
2 2 5
3 3 6
4 4 7
2 5 0
3 6 9
4 7 10
5 8 11
3 9 1
4 10 13
5 11 14
6 12 15
3 1 1
4 13 17
5 14 18
6 15 19
7 16 20
4 17 17
5 18 22
6 19 23
7 20 24
8 21 25
5 22 22
6 23 27
7 24 28
8 25 29
9 26 30
";
	let (rf, _, _) = rule!{B 3 / S 2 3};
	let states = ".#";
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_tree(&rf);
	assert_eq!(format!("{:?}", rt), tree);
}
