use crate::generators::constants;
use crate::generators::gen_ratings;
use crate::ratings::player_ratings::Ratings;



pub struct Skills {
    on_ball_adv_creation: u16,
    off_ball_adv_creation: u16,

    post_shot:u16,
    inside_shot: u16,
    mid_shot: u16,
    three_shot: u16,

    movement_shooting: u16,
    pull_up_shooting: u16,

    finishing: u16,

    passing_vision: u16,
    passing_iq: u16,
    passing_creativity: u16,


    on_ball_defense: u16,
    help_defense: u16,
    block: u16,
    steal:u16,

    rebounding_iq:u16,

    hustle: u16,
    endurance:u16,
}

impl Skills{
    fn calc(ratings:Ratings) -> Skills {
        

        let on_ball_adv_creation = (ratings.speed * 5) + (ratings.burst * 3) + (ratings.fluidity * 4 ) ;



        return Skills{
            on_ball_adv_creation,
            off_ball_adv_creation,

            post_shot,
            inside_shot,
            mid_shot,
            three_shot,

            movement_shooting,
            pull_up_shooting,

            finishing,
            passing_vision,
            passing_iq,
            passing_creativity,


            on_ball_defense,
            help_defense,
            block,
            steal,

            hustle,
            endurance,
        };

    }
}

