#[macro_use]
extern crate cellauto;
use cellauto::rules;
use cellauto::pattern::Pattern;
use cellauto::golly;
use std::convert::TryFrom;
use cellauto::rules::logic_land::State::*;
use cellauto::rules::logic_land::HorizontalDir::*;
use cellauto::rules::logic_land::VerticalDir::*;
use cellauto::rules::logic_land::Action::*;
use std::fs;

#[test]
fn test_logic_land() {
	// 此样本规则来自<https://conwaylife.com/forums/viewtopic.php?f=11&t=2163&p=30044#p33295>，使用golly自带脚本转换
	let tree = fs::read_to_string("tests/LogicLand.rule").expect("Could not read file");
	let (rf, _, _) = rule!{LogicLand};
	let states = [
		Empty,
		InactiveWire,
		ActiveWire,
		InhibitedWire,
		Cross(None,None),
		NorGate(Inactive),
		OrGate(Inactive),
		XorGate(Inactive),
		AndGate(Inactive),
		TFlipFlop(Inactive),
		GateOutput(Inactive),
		NorGate(Active),
		OrGate(Active),
		XorGate(Active),
		AndGate(Active),
		TFlipFlop(Active),
		GateOutput(Active),
		Cross(None,Some(North)),
		Cross(Some(West),Some(South)),
		Cross(None,Some(South)),
		Cross(Some(West),Some(North)),
		AuxiliaryAcive,
		Cross(Some(East),Some(South)),
		Cross(Some(East),Some(North)),
		AuxiliaryInhibited,
		Cross(Some(East),None),
		Cross(Some(West),None),
	];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_von_neumann_tree(&rf);
	assert_eq!(format!("@RULE LogicLand\n\n@TREE\n\n{:?}", rt), tree);
}