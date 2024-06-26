extern crate rand;

use rand::Rng;

mod citizen {
    use std::fmt::Debug;

    pub struct Citizen {
        pub id: u64,
        pub birth_year: u16,
        pub can_vote: bool,
        pub political_preference: u16,
    }

    impl Citizen {
        pub(crate) fn new(id:u64, year:u16, political_preference:u16) -> Citizen {
            Citizen {
                id,
                birth_year: year,
                can_vote: false,
                political_preference,
            }
        
    }}

    impl Debug for Citizen {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Result::Ok(())
        }
    }


}

pub mod society {
    use std::fmt::Debug;

    use crate::citizen::Citizen;

    pub struct Society {
        pub citizens : Vec<Citizen>,
        political_options: PoliticalOptions,
        government_performance: GovernmentPerformance,
        initial_distribution: InitialDistribution,
        year: u32,
        vote_age: VoteAge,
      
    }
    enum PoliticalOptions {
        ThreeOptions(u8),
        TwoOptions(u8),
    }

    enum GovernmentPerformance {
        Fixed(u8),
        Ranged(u8, u8),
    }

    enum InitialDistribution {
        Random,
        Seed(Vec<u8>, u8),
        FixedPattern(Vec<u8>),
    }

    enum VoteAge {
        TwentyOne,
        Eighteen,
    }

    impl Society {
        pub fn new (
            citizens_number: u32,
            // political_options_str: &str,
            // government_performance_str: &str,
            // initial_distribution: &str,
            // vote_age: &str,
        ) -> Society {        

            // Creating citizens
            let mut citizens: Vec<Citizen> = Vec::new();
            for i in (0..citizens_number) {
                citizens.push(
                    Citizen::new(1, 2023, 50)
                )
            }

            Society {
                citizens,
                political_options: PoliticalOptions::TwoOptions(100),
                government_performance: GovernmentPerformance::Fixed(50),
                initial_distribution: InitialDistribution::Random,
                year: 0,
                vote_age:VoteAge::TwentyOne,

            }


        }        
    }
 
    impl Debug for Society {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Result::Ok(())
        }
    }


}