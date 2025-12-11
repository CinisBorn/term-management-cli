use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Term {
    pub term: String,
    pub more_information: String,
}

#[derive(Debug)]
pub struct TermRelation {
    pub from_id: i64,
    pub to_id: i64,
}

#[derive(Debug, Tabled)]
pub struct TermRelationView {
    pub from_id: String,
    pub to_id: String,
}