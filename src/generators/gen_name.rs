use rand_derive2::RandGen;
use strum::EnumString;
use std::fmt;

#[derive(Debug, RandGen, EnumString)]
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

#[derive(Debug, RandGen, EnumString)]
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
macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl fmt::Display for $t {
            fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
                write!(f, "{:?}", self)
            }
        })*
    }
}

impl_T!(for FirstNameAmerican, LastNameAmerican);


pub fn gen_name(){

    let first_name: FirstNameAmerican = rand::random();
    let last_name: LastNameAmerican = rand::random();
    
    println!("{:?} {:?}", first_name, last_name);
}
