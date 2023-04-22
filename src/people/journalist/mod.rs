use crate::team::teams::TeamName;


#[derive(Debug, Clone)]
enum Reigon {
    National,
    TeamName(TeamName),
}

#[derive(Debug, Clone)]
pub struct Journalist {
    reigon: Reigon,
    sub_stack: String,
    articles: Vec<String>,
}
