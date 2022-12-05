pub struct Storage {
    crates: Vec<Vec<char>>,
}

impl Storage {
    pub fn move_crates_sequentially(&mut self, source: usize, target: usize, amount: usize) {
        let drain_range_start = self.crates[source].len() - amount;

        let mut moved_crates: Vec<char> = self.crates[source].drain(drain_range_start..).collect();
        moved_crates.reverse();
        self.crates[target].append(&mut moved_crates);
    }

    pub fn move_crates_at_once(&mut self, source: usize, target: usize, amount: usize) {
        let drain_range_start = self.crates[source].len() - amount;
        let mut moved_crates: Vec<char> = self.crates[source].drain(drain_range_start..).collect();
        self.crates[target].append(&mut moved_crates);
    }

    pub fn get_top_crates(&self) -> String {
        return self
            .crates
            .iter()
            .fold(String::from(""), |mut sequence, current| {
                if let Some(top) = current.last() {
                    sequence.push_str(&top.to_string());
                }

                return sequence;
            });
    }

    pub fn fill_slot_reversed(&mut self, column: usize, char: char) {
        if self.crates.len() <= column {
            self.crates.resize(column + 1, vec![]);
        }

        self.crates[column].insert(0, char);
    }

    pub fn print_storage(&self) {
        let highest = self.crates.iter().fold(0, |highest, current| {
            if current.len() > highest {
                current.len()
            } else {
                highest
            }
        });

        let mut lines: Vec<String> = vec![];

        for row in 0..highest {
            self.crates.iter().enumerate().for_each(|(_, items)| {
                let item_opt: Option<char> = if items.len() > row {
                    Some(items[row])
                } else {
                    None
                };

                if lines.len() <= row {
                    lines.resize(row + 1, "".to_string());
                }

                let text = if let Some(item) = item_opt {
                    format!("[{}]", item.to_string())
                } else {
                    "   ".to_string()
                };

                lines[row].push_str(&text);
            })
        }

        lines.reverse();

        lines.iter().for_each(|line| {
            println!("{}", line);
        });

        let column_legend =
            (0..self.crates.len())
                .into_iter()
                .fold("".to_string(), |mut line, current| {
                    line.push_str(&format!(" {} ", current));
                    return line;
                });

        println!("{}", column_legend)
    }

    /**
     * [M] [H]         [N]                
     * [S] [W]         [F]     [W] [V]    
     * [J] [J]         [B]     [S] [B] [F]
     * [L] [F] [G]     [C]     [L] [N] [N]
     * [V] [Z] [D]     [P] [W] [G] [F] [Z]
     * [F] [D] [C] [S] [W] [M] [N] [H] [H]
     * [N] [N] [R] [B] [Z] [R] [T] [T] [M]
     * [R] [P] [W] [N] [M] [P] [R] [Q] [L]
     *  1   2   3   4   5   6   7   8   9
     */
    pub fn from_input(input: &str) -> Storage {
        let mut storage = Storage { crates: vec![] };

        let mut total_column_count_opt: Option<usize> = None;

        let mut next_is_value = false;

        for (index, char) in input.chars().enumerate() {
            if char == '\n' {
                // set the total column count for the first time
                if total_column_count_opt.is_none() {
                    total_column_count_opt = Some((index / 4) + 1);
                }

                continue;
            }

            let intermed_column = ((index as f32) / 4.0).floor() as usize;

            let column_count = if let Some(total_column_count) = total_column_count_opt {
                intermed_column % total_column_count
            } else {
                intermed_column
            };

            if char == '[' {
                next_is_value = true;
                continue;
            }

            if next_is_value {
                storage.fill_slot_reversed(column_count, char);
                next_is_value = false;
                continue;
            }
        }

        return storage;
    }
}
