use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct TermMetaData {
    pub term: String,
    pub origin: String,
    pub r#type: String,
}

#[derive(Debug)]
pub struct Term {
    pub term: String,
    pub definition: String,
    pub r#type: String,
    pub origin: String,
}

#[derive(Debug, Clone)]
pub struct TermRelation {
    pub from_id: i64,
    pub to_id: i64,
    pub relation: String,
}
