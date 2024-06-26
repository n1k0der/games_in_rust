extern crate rand;

use rand::Rng;

mod citizen {
    struct Citizen {
        id: u64,
        birth_year: u16,
        can_vote: bool,
        sex_at_birth: Sex,
        political_preference: PoliticalOptions,
        children: u8,
    }
    pub enum Sex {
        Male,
        Female,
    }
    enum PoliticalOptions {
        ThreeOptions(u8),
        TwoOptions(u8),
    }

    trait RandomEnumSelection {
        
    }

    impl Citizen {
        fn new(id:u64, year:u16) -> Citizen {
            use crate::citizen::Sex;
            Citizen {
                id,
                birth_year: year,
                can_vote: false,
                sex_at_birth: Sex,
                political_preference: PoliticalOptions,
                children: u8,
            }
        }
        
    }

}

mod society {

}