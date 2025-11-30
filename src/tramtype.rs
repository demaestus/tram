use serde::{Serialize, Deserialize};

pub enum TramCommand {
	Undetermined,
	List,
	Edit,
	Open,
	Close,
	Init,
	Config,
	Tram,
}
pub struct OptionsManipulate {
	milestone: String,
	group: String,
	category: String,
	identifier: String,
	remove: bool,
}
pub struct OptionsInit {
	name: String,
}
pub enum IssueStatus {
	Open,
	Closed,
	Assigned,
}
pub enum IssueCat {

}
pub struct Issue {
	name: String,
	status: IssueStatus,
	cat: IssueCat,
	mil: String,
	group: String,

}

#[derive(Debug)]
pub enum TramFlag {
	OptLowerC,
	OptLowerM,
	OptLowerG,
	OptLowerR,
	OptLowerA,
	Trust,
	All,
	Arg(u8),
}
pub enum TramErr{
	Generic(&'static str),
	CommandMissing,
	ParsingFailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TramUser {
	name: String,
	email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TramConfig {
	proj_name: String,
	user: TramUser,
	milestones: Vec<String>,
	groups: Vec<String>,
	tags: Vec<String>,
}
impl TramConfig {
	pub fn new(proj_name: String) -> TramConfig {
		TramConfig {
			proj_name: proj_name,
			user: TramUser { name: "Ferko".into(), email: "anything@all.org".into() },
			milestones: Vec::new(),
			groups: Vec::new(),
			tags: vec!["urgent".into(), "".into()],
		}
	}
}