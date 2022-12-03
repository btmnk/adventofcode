#[derive(Clone)]
pub struct Rucksack {
    pub first_compartment: String,
    pub second_compartment: String,
}

impl Rucksack {
    pub fn from_content(rucksack_content: String) -> Rucksack {
        let first_compartment = &rucksack_content[..rucksack_content.len() / 2];
        let second_compartment = &rucksack_content[rucksack_content.len() / 2..];

        Rucksack {
            first_compartment: first_compartment.to_string(),
            second_compartment: second_compartment.to_string(),
        }
    }

    pub fn get_common_type(&self) -> Option<char> {
        let common = self
            .first_compartment
            .find(|c| self.second_compartment.contains(c));

        if let Some(common_index) = common {
            return self.first_compartment.chars().nth(common_index);
        } else {
            panic!("Could not find common item type!");
        }
    }

    pub fn get_all_items(&self) -> Vec<char> {
        self.first_compartment
            .chars()
            .chain(self.second_compartment.chars())
            .collect()
    }

    pub fn has_item_type(&self, item_type: &char) -> bool {
        return self.first_compartment.contains(*item_type)
            || self.second_compartment.contains(*item_type);
    }
}
