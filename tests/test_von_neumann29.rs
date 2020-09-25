#[macro_use]
extern crate ca;
use ca::rules;
use ca::pattern::Pattern;
use ca::golly;
use std::convert::TryFrom;

#[test]
fn test_von_neumann29() {
	let tree = "num_states=29
num_neighbors=4
num_nodes=67
1 0 2 4 6 8 12 18 20 9 9 10 11 12 9 10 11 12 17 18 19 20 17 18 19 20 25 27 25 27
1 1 3 5 7 11 17 19 25 10 13 14 15 12 13 14 15 12 0 0 0 0 0 0 0 0 26 28 26 28
1 1 3 5 7 11 17 19 25 10 0 0 0 0 0 0 0 0 21 22 23 20 21 22 23 20 0 0 0 0
1 0 2 4 6 8 12 18 20 9 13 14 15 12 13 14 15 12 21 22 23 20 21 22 23 20 25 27 25 27
2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 2 0 0 0 0 3 3
1 1 3 5 7 11 17 19 25 10 13 14 15 12 13 14 15 12 0 0 0 0 0 0 0 0 25 27 25 27
2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 5 0 0 0 0 0 0 0 2 0 0 0 0 3 3
1 1 3 5 7 11 17 19 25 10 9 14 15 16 9 14 15 16 0 0 0 0 0 0 0 0 26 28 26 28
1 1 3 5 7 11 17 19 25 10 9 14 15 16 9 14 15 16 0 0 0 0 0 0 0 0 25 27 25 27
1 1 3 5 7 11 17 19 25 10 13 14 15 16 13 14 15 16 0 0 0 0 0 0 0 0 26 28 26 28
1 1 3 5 7 11 17 19 25 10 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
2 7 7 7 7 7 7 7 7 7 7 8 7 7 7 9 7 7 7 7 7 7 7 10 7 7 7 7 9 9
1 1 3 5 7 11 17 19 25 10 0 0 0 0 0 0 0 0 17 22 23 24 17 22 23 24 0 0 0 0
1 1 3 5 7 11 17 19 25 10 0 0 0 0 0 0 0 0 21 22 23 24 21 22 23 24 0 0 0 0
2 12 12 12 12 12 12 12 12 12 12 12 12 12 12 10 12 12 12 12 12 12 12 13 12 12 12 12 13 13
1 0 2 4 6 8 12 18 20 9 9 14 15 16 9 14 15 16 17 22 23 24 17 22 23 24 25 27 25 27
1 0 2 4 6 8 12 18 20 9 13 14 15 16 13 14 15 16 21 22 23 24 21 22 23 24 25 27 25 27
2 15 15 15 15 15 15 15 15 15 15 15 15 15 15 9 15 15 15 15 15 15 15 13 15 15 15 15 16 16
3 4 4 4 4 4 4 4 4 4 4 4 6 4 4 4 11 4 4 4 4 4 4 4 14 4 4 4 17 17
1 1 3 5 7 11 17 19 25 10 13 14 15 16 13 14 15 16 0 0 0 0 0 0 0 0 25 27 25 27
2 8 8 8 8 8 8 8 8 8 8 8 8 8 8 19 8 8 8 8 8 8 8 10 8 8 8 8 19 19
2 15 15 15 15 15 15 15 15 15 15 15 15 15 15 19 15 15 15 15 15 15 15 13 15 15 15 15 16 16
3 6 6 6 6 6 6 6 6 6 6 6 6 6 6 6 20 6 6 6 6 6 6 6 14 6 6 6 21 21
1 1 3 5 7 11 17 19 25 10 13 14 11 16 13 14 11 16 0 0 0 0 0 0 0 0 26 28 26 28
1 1 3 5 7 11 17 19 25 10 13 14 11 16 13 14 11 16 0 0 0 0 0 0 0 0 25 27 25 27
2 23 23 23 23 23 23 23 23 23 23 24 23 23 23 9 23 23 23 23 23 23 23 10 23 23 23 23 9 9
2 24 24 24 24 24 24 24 24 24 24 24 24 24 24 19 24 24 24 24 24 24 24 10 24 24 24 24 19 19
2 9 9 9 9 9 9 9 9 9 9 19 9 9 9 9 9 9 9 9 9 9 9 10 9 9 9 9 9 9
2 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10
3 25 25 25 25 25 25 25 25 25 25 25 26 25 25 25 27 25 25 25 25 25 25 25 28 25 25 25 27 27
1 1 3 5 7 11 17 19 25 10 0 0 0 0 0 0 0 0 21 22 19 24 21 22 19 24 0 0 0 0
2 30 30 30 30 30 30 30 30 30 30 30 30 30 30 10 30 30 30 30 30 30 30 13 30 30 30 30 13 13
2 13 13 13 13 13 13 13 13 13 13 13 13 13 13 10 13 13 13 13 13 13 13 13 13 13 13 13 13 13
3 31 31 31 31 31 31 31 31 31 31 31 31 31 31 31 28 31 31 31 31 31 31 31 32 31 31 31 32 32
1 0 2 4 6 8 12 18 20 9 13 14 11 16 13 14 11 16 21 22 19 24 21 22 19 24 25 27 25 27
2 34 34 34 34 34 34 34 34 34 34 34 34 34 34 9 34 34 34 34 34 34 34 13 34 34 34 34 16 16
2 34 34 34 34 34 34 34 34 34 34 34 34 34 34 19 34 34 34 34 34 34 34 13 34 34 34 34 16 16
2 16 16 16 16 16 16 16 16 16 16 16 16 16 16 9 16 16 16 16 16 16 16 13 16 16 16 16 16 16
3 35 35 35 35 35 35 35 35 35 35 35 36 35 35 35 27 35 35 35 35 35 35 35 32 35 35 35 37 37
4 18 18 18 18 18 18 18 18 18 22 18 18 18 29 18 18 18 18 18 18 18 33 18 18 18 18 18 38 38
2 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 19 10 19 19 19 19 19 19
3 26 26 26 26 26 26 26 26 26 26 26 26 26 26 26 40 26 26 26 26 26 26 26 28 26 26 26 40 40
2 16 16 16 16 16 16 16 16 16 16 16 16 16 16 19 16 16 16 16 16 16 16 13 16 16 16 16 16 16
3 36 36 36 36 36 36 36 36 36 36 36 36 36 36 36 40 36 36 36 36 36 36 36 32 36 36 36 42 42
4 22 22 22 22 22 22 22 22 22 22 22 22 22 41 22 22 22 22 22 22 22 33 22 22 22 22 22 43 43
1 1 3 5 7 11 17 19 25 10 13 10 15 16 13 10 15 16 0 0 0 0 0 0 0 0 26 28 26 28
1 1 3 5 7 11 17 19 25 10 13 10 15 16 13 10 15 16 0 0 0 0 0 0 0 0 25 27 25 27
2 45 45 45 45 45 45 45 45 45 45 46 45 45 45 9 45 45 45 45 45 45 45 10 45 45 45 45 9 9
2 46 46 46 46 46 46 46 46 46 46 46 46 46 46 19 46 46 46 46 46 46 46 10 46 46 46 46 19 19
3 47 47 47 47 47 47 47 47 47 47 47 48 47 47 47 27 47 47 47 47 47 47 47 28 47 47 47 27 27
3 48 48 48 48 48 48 48 48 48 48 48 48 48 48 48 40 48 48 48 48 48 48 48 28 48 48 48 40 40
3 27 27 27 27 27 27 27 27 27 27 27 40 27 27 27 27 27 27 27 27 27 27 27 28 27 27 27 27 27
3 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28 28
4 49 49 49 49 49 49 49 49 49 50 49 49 49 51 49 49 49 49 49 49 49 52 49 49 49 49 49 51 51
1 1 3 5 7 11 17 19 25 10 0 0 0 0 0 0 0 0 21 18 23 24 21 18 23 24 0 0 0 0
2 54 54 54 54 54 54 54 54 54 54 54 54 54 54 10 54 54 54 54 54 54 54 13 54 54 54 54 13 13
3 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 28 55 55 55 55 55 55 55 32 55 55 55 32 32
3 32 32 32 32 32 32 32 32 32 32 32 32 32 32 32 28 32 32 32 32 32 32 32 32 32 32 32 32 32
4 56 56 56 56 56 56 56 56 56 56 56 56 56 52 56 56 56 56 56 56 56 57 56 56 56 56 56 57 57
1 0 2 4 6 8 12 18 20 9 13 10 15 16 13 10 15 16 21 18 23 24 21 18 23 24 25 27 25 27
2 59 59 59 59 59 59 59 59 59 59 59 59 59 59 9 59 59 59 59 59 59 59 13 59 59 59 59 16 16
2 59 59 59 59 59 59 59 59 59 59 59 59 59 59 19 59 59 59 59 59 59 59 13 59 59 59 59 16 16
3 60 60 60 60 60 60 60 60 60 60 60 61 60 60 60 27 60 60 60 60 60 60 60 32 60 60 60 37 37
3 61 61 61 61 61 61 61 61 61 61 61 61 61 61 61 40 61 61 61 61 61 61 61 32 61 61 61 42 42
3 37 37 37 37 37 37 37 37 37 37 37 42 37 37 37 27 37 37 37 37 37 37 37 32 37 37 37 37 37
4 62 62 62 62 62 62 62 62 62 63 62 62 62 51 62 62 62 62 62 62 62 57 62 62 62 62 62 64 64
5 39 39 39 39 39 39 39 39 39 39 39 39 44 39 39 39 53 39 39 39 39 39 39 39 58 39 39 65 65
";
	let (rf, _, _) = rule!{von Neumann 29};
	let states = concat!{
		"USS0S1S00S01S10S11S000",
		"To>_To^_To<_Tov_",
		"To>~To^~To<~Tov~",
		"Ts>_Ts^_Ts<_Tsv_",
		"Ts>~Ts^~Ts<~Tsv~",
		"C__C_~C~_C~~"
	};
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_von_neumann_tree(&rf);
	assert_eq!(format!("{:?}", rt), tree);
}