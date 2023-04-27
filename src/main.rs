use team::teams::Team;

pub mod ratings;
pub mod generators;
pub mod people;
pub mod team;
pub mod save;


fn main() {
    let mut teams: Vec<Team> = Team::gen_teams();

    println!("Player #1 {:#?}", teams[0].players[0]); 

    for team in teams.iter_mut(){
        team.develop_team();
    }

    println!("Player #1 {:#?}", teams[0].players[0]);
    
}
