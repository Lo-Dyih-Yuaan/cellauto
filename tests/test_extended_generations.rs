#[macro_use]
extern crate cellauto;
use cellauto::rules;
use cellauto::pattern::Pattern;
use cellauto::golly;
use std::convert::TryFrom;

#[test]
fn test_b345_s5_a3i3() {
	let tree = "num_states=7
num_neighbors=8
num_nodes=35
1 0 2 3 4 5 6 0
2 0 0 0 0 0 0 0
3 1 1 1 1 1 1 1
1 1 2 3 4 5 6 0
2 0 3 3 3 0 0 0
3 1 4 4 4 1 1 1
4 2 5 5 5 2 2 2
2 3 3 3 3 3 3 3
3 4 7 7 7 4 4 4
4 5 8 8 8 5 5 5
5 6 9 9 9 6 6 6
1 1 1 2 3 5 6 0
2 3 11 11 11 3 3 3
3 7 12 12 12 7 7 7
4 8 13 13 13 8 8 8
5 9 14 14 14 9 9 9
6 10 15 15 15 10 10 10
2 11 0 0 0 11 11 11
3 12 17 17 17 12 12 12
4 13 18 18 18 13 13 13
5 14 19 19 19 14 14 14
6 15 20 20 20 15 15 15
7 16 21 21 21 16 16 16
3 17 1 1 1 17 17 17
4 18 23 23 23 18 18 18
5 19 24 24 24 19 19 19
6 20 25 25 25 20 20 20
7 21 26 26 26 21 21 21
8 22 27 27 27 22 22 22
4 23 2 2 2 23 23 23
5 24 29 29 29 24 24 24
6 25 30 30 30 25 25 25
7 26 31 31 31 26 26 26
8 27 32 32 32 27 27 27
9 28 33 33 33 28 28 28
";
	let (rf, _, _) = rule!{B 3 4 5 / S 5 / A 3 I 3};
	let states = ".ABCDEF";
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_tree(&rf);
	assert_eq!(format!("{:?}", rt), tree);
}

#[test]
fn test_b2_ek_s345_a1i3a1i3() {
	let tree = "num_states=9
num_neighbors=8
num_nodes=51
1 0 2 3 4 5 6 7 8 0
2 0 0 0 0 0 0 0 0 0
3 1 1 1 1 1 1 1 1 1
1 1 2 3 4 5 6 7 8 0
1 0 1 3 4 5 5 7 8 0
2 3 4 3 3 3 4 3 3 3
3 1 5 1 1 1 5 1 1 1
4 2 6 2 2 2 6 2 2 2
2 0 3 0 0 0 3 0 0 0
2 0 4 0 0 0 4 0 0 0
3 8 9 8 8 8 9 8 8 8
2 4 4 4 4 4 4 4 4 4
3 9 11 9 9 9 11 9 9 9
4 10 12 10 10 10 12 10 10 10
5 7 13 7 7 7 13 7 7 7
3 8 5 8 8 8 5 8 8 8
4 15 12 15 15 15 12 15 15 15
3 11 11 11 11 11 11 11 11 11
4 12 17 12 12 12 17 12 12 12
5 16 18 16 16 16 18 16 16 16
6 14 19 14 14 14 19 14 14 14
3 5 11 5 5 5 11 5 5 5
4 10 21 10 10 10 21 10 10 10
5 22 18 22 22 22 18 22 22 22
4 21 17 21 21 21 17 21 21 21
2 4 0 4 4 4 0 4 4 4
3 11 25 11 11 11 25 11 11 11
4 17 26 17 17 17 26 17 17 17
5 24 27 24 24 24 27 24 24 24
6 23 28 23 23 23 28 23 23 23
7 20 29 20 20 20 29 20 20 20
4 6 12 6 6 6 12 6 6 6
5 31 24 31 31 31 24 31 31 31
6 32 28 32 32 32 28 32 32 32
3 25 1 25 25 25 1 25 25 25
4 26 34 26 26 26 34 26 26 26
5 27 35 27 27 27 35 27 27 27
6 28 36 28 28 28 36 28 28 28
7 33 37 33 33 33 37 33 33 33
8 30 38 30 30 30 38 30 30 30
3 1 9 1 1 1 9 1 1 1
4 40 21 40 40 40 21 40 40 40
5 41 24 41 41 41 24 41 41 41
6 42 28 42 42 42 28 42 42 42
7 43 37 43 43 43 37 43 43 43
4 34 2 34 34 34 2 34 34 34
5 35 45 35 35 35 45 35 35 35
6 36 46 36 36 36 46 36 36 36
7 37 47 37 37 37 47 37 37 37
8 44 48 44 44 44 48 44 44 44
9 39 49 39 39 39 49 39 39 39
";
	let (rf, _, _) = rule!(non-totalistic B "2-ek" / S "345" / A 1 I 3 A 1 I 3);
	let states = ".ABCDEFGH";
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_tree(&rf);
	assert_eq!(format!("{:?}", rt), tree);
}
