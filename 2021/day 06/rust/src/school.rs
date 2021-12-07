#[derive(Clone, Debug)]
pub struct Generation {
    pub age: i32,
    pub size: u128,
}

pub struct School {
    pub generations: Vec<Generation>,
}

impl School {
    pub fn pass_day(&mut self) {
        let mut new_generation_size: u128 = 0;

        for generation in &mut self.generations {
            if generation.age == 0 {
                new_generation_size += generation.size;
            }

            generation.age -= 1;
        }

        if new_generation_size > 0 {
            let existing_generation_zero_opt = self.generations.iter_mut().position(|generation| generation.age == -1);
            if let Some(existing_generation_zero) = existing_generation_zero_opt {
                self.generations.swap_remove(existing_generation_zero);
            }

            let existing_generation_six_opt = self.generations.iter_mut().find(|generation| generation.age == 6);
            if let Some(existing_generation_six) = existing_generation_six_opt {
                existing_generation_six.size += new_generation_size;
            } else {
                self.generations.push(Generation { age: 6, size: new_generation_size });
            }
    
            self.generations.push(Generation { age: 8, size: new_generation_size });
        }
    }

    pub fn pass_days(&mut self, days: i32) {
        for _ in 0..days {
            self.pass_day();
        }
    }

    pub fn count_fish(&self) -> u128 {
        return self.generations.iter().fold(0, |sum, current| sum + current.size as u128);
    }
}

pub fn from(fishes: Vec<i32>) -> School {
    let mut generations: Vec<Generation> = Vec::new(); 

    for fish in fishes {
        let existing_generation = generations.iter().position(|generation| generation.age == fish); 

        if let Some(index) = existing_generation {
            generations[index].size += 1;
        } else {
            let new_generation = Generation { age: fish, size: 1 };
            generations.push(new_generation);
        }
    }

    return School { generations };
}