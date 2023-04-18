use rand_derive2::RandGen;
use strum_macros::{EnumString, Display};

#[derive(Debug, Display, RandGen, EnumString)]
enum FirstNameAmerican {
    Ryan, Mike, Jalen, Jaden,
    Seth, Stephen, Cam, James,
    Tobias, Henry, Paul, Anthony,
    Spencer, Royce, Joe, Nick,
    Joel, Michael, Mikal, Tyrese,
    Dyson, Samson, Webster, Talon,
    Morley, Chad, Elliot, Doug,
    Orrell, Cosmo,
}

#[derive(Debug, Display, RandGen, EnumString)]
enum LastNameAmerican {
    Johnson, Williams, Brown, Jones,
    Garcia, Miller, Davis, Rodriguez,
    Martinez, Hernandez, Lopez, Gonzales,
    Wilson, Anderson, Thomas, Taylor,
    Moore, Jackson, Martin, Lee,
    Indie, Howard, Shirley, Westbrook,
    Rowe, Barclay, Bannister, Hartell,
    Reed, Beckham, Warrick, Lyndon,
    Tucker, Denzil,
}


pub fn gen_name() -> String {
    let mut name = String::new();

    let first_name:FirstNameAmerican = rand::random();
    let last_name:LastNameAmerican = rand::random();

    name.push_str(&first_name.to_string());
    name.push(' ');
    name.push_str(&last_name.to_string());

    name 
}
