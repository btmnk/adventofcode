use std::fmt;

#[derive(Clone)]
pub struct Field {
    pub value: String,
    pub marked: bool,
}

impl Field {
    pub fn mark(&mut self) {
        self.marked = true;
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.value, self.marked)
    }
}

#[derive(Clone)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
    pub last_called_number: Option<i32>,
    pub rounds_until_bingo: Option<i32>,
}

impl Board {
    pub fn get_fields_by_value(&mut self, field_value: &str) -> Vec<&mut Field> {
        let mut target_fields: Vec<&mut Field> = Vec::new();

        for field_row in &mut self.fields {
            for field in field_row {
                if field.value == field_value {
                    target_fields.push(field);
                }
            }
        }

        return target_fields;
    }

    pub fn get_unmarked_fields(&self) -> Vec<&Field> {
        let mut target_fields: Vec<&Field> = Vec::new();

        for field_row in &self.fields {
            for field in field_row {
                if field.marked == false {
                    target_fields.push(field);
                }
            }
        }

        return target_fields;
    }

    pub fn get_vertical_zip(&self) -> Vec<Vec<&Field>> {
        let mut vertical_zip: Vec<Vec<&Field>> = Vec::new();

        for column in 0..4 {
            let mut column_fields: Vec<&Field> = Vec::new();

            for field in &self.fields {
                column_fields.push(field.into_iter().nth(column).unwrap());
            }

            vertical_zip.push(column_fields);
        }

        return vertical_zip;
    }

    pub fn mark_fields(&mut self, field_value: &str) {
        let fields = self.get_fields_by_value(field_value);
        for field in fields {
            field.mark();
        }
        self.last_called_number = Some(field_value.parse::<i32>().unwrap());
        self.rounds_until_bingo = if let Some(rounds) = self.rounds_until_bingo {
            Some(rounds + 1)
        } else {
            Some(1)
        }
    }

    pub fn is_bingo(&self) -> bool {
        let has_horizontal_bingo = &self.fields.iter().any(|row| {
            return row.iter().all(|field| field.marked);
        });

        let has_vertical_bingo = &self.get_vertical_zip().iter().any(|row| {
            return row.iter().all(|field| field.marked);
        });

        return has_horizontal_bingo == &true || has_vertical_bingo == &true;
    }

    pub fn calculate_score(&self) -> i32 {
        let unmarked_fields_as_intergers: Vec<i32> = self
            .get_unmarked_fields()
            .into_iter()
            .map(|it| it.value.parse::<i32>().unwrap())
            .collect();
        let unmarked_fields_sum: i32 = unmarked_fields_as_intergers.into_iter().sum();
        let result = unmarked_fields_sum * self.last_called_number.unwrap();

        return result;
    }
}

pub fn create_board_from_input(board_string: String) -> Board {
    let fields: Vec<Vec<Field>> = board_string
        .split("\n")
        .map(|row| {
            return row
                .to_string()
                .split(" ")
                .map(str::to_string)
                .filter(|field| field != "")
                .map(|field| Field {
                    value: field,
                    marked: false,
                })
                .collect();
        })
        .collect();

    return Board {
        fields,
        last_called_number: None,
        rounds_until_bingo: None,
    };
}
