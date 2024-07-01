extern crate rand;

use rand::Rng;

mod citizen {
    use std::fmt::Debug;

    pub struct Citizen {
        pub id: u64,
        pub birth_year: i16,
        pub can_vote: bool,
        pub political_preference: u32,
    }

    impl Citizen {
        pub(crate) fn new(id:u64, year:i16, political_preference:u32) -> Citizen {
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
    use std::{collections::HashMap, fmt::Debug};

    use rand::Rng;

    use crate::citizen::{self, Citizen};

    pub struct Society {
        pub citizens : Vec<Citizen>,
        political_options: PoliticalOptions,
        government_performance: GovernmentPerformance,
        initial_distribution: InitialDistribution,
        year: u32,
        vote_age: VoteAge,
        government: Government,
      
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

    enum Government {
        None, 
        Liberal,
        Conservative, 
        Progressist,
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
            for i in 0..citizens_number {
                citizens.push(
                    Citizen::new(i as u64, 
                               rand::thread_rng().gen_range(-100..=-21), 
                               rand::thread_rng().gen_range(1..=90),                               
                                )
                            )
            }

            Society {
                citizens,
                political_options: PoliticalOptions::ThreeOptions(90),
                government_performance: GovernmentPerformance::Fixed(50),
                initial_distribution: InitialDistribution::Random,
                year: 0,
                vote_age:VoteAge::TwentyOne,
                government: Government::None,

            }


        }        
        
        pub fn elections(&mut self) {
            // in the next vector, the votes are [Progressism, Liberal, Conservative]
            let mut election_results = vec![0,0,0];

            for citizen in self.citizens.iter() {
                if citizen.political_preference <= 30 {
                    election_results[0] += 1;
                } else if citizen.political_preference > 30 &&
                          citizen.political_preference <= 60 {
                    election_results[1] += 1;
                } else {
                    election_results[2] += 1;
                }
            }

            let total_votes = election_results[0] + election_results[1] + election_results[2];

            if (election_results[0] / total_votes) as f32 > 0.5 {
                self.government = Government::Progressist;
                return
            } else if  (election_results[1] / total_votes) as f32 > 0.5 {
                self.government = Government::Liberal;
                return
            } else if  (election_results[2] / total_votes) as f32 > 0.5 {
                self.government = Government::Conservative;
                return
            }

            if election_results[0] == election_results[1] &&
                election_results[1] == election_results[2] &&
                election_results[0] == election_results[2] {
                    let mut random_elector = rand::thread_rng();
                    let index_elected = random_elector.gen_range(0..3);
                        if index_elected == 0 {
                            self.government = Government::Progressist;
                            return
                        } else if index_elected == 1 {
                            self.government = Government::Liberal;
                            return
                        } else if index_elected == 2 {
                            self.government = Government::Conservative;
                            return
                        }
            } else if election_results[0] == election_results[1] &&
                      election_results[1] > election_results[2] {

            } else if election_results[1] == election_results[2] &&
                      election_results[1] > election_results[0] {
                    let mut random_elector = rand::thread_rng();
                    let index_elected = random_elector.gen_range(1..3);
                    if index_elected == 1 {
                        self.government = Government::Liberal;
                    } else if index_elected == 2 {
                        self.government = Government::Conservative;
                    }
            }

            
            }

           


            


        }

        impl Debug for Society {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Result::Ok(())
            }
        }
    
    }
 
    


