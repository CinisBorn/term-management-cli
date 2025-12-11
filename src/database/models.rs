use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Term {
    pub term: String,
    pub more_information: String,
}

#[derive(Debug, Clone)]
pub struct TermRelation {
    pub from_id: i64,
    pub to_id: i64,
}
