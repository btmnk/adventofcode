#[derive(Clone)]
pub struct Generation {
    pub age: i32,
    pub size: i64,
}

pub struct School {
    pub generations: Vec<Generation>,
}

impl School {
    pub fn pass_day(&mut self) {
        let mut new_generation_size: i64 = 0;

        for generation in &mut self.generations {
            if generation.age == 0 {
                new_generation_size += generation.size;
                generation.age = 6;
            } else {
                generation.age -= 1;
            }
        }

        self.generations.push(Generation { age: 8, size: new_generation_size });
    }

    pub fn pass_days(&mut self, days: i32) {
        for _ in 0..days {
            self.pass_day();
        }
    }

    pub fn count_fish(&self) -> usize {
        let mut fish_count: usize = 0;

        for generation in &self.generations {
            fish_count += generation.size as usize;
        }

        return fish_count;
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