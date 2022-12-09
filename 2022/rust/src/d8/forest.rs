use std::{
    collections::{HashMap, HashSet},
    thread::current,
};

#[derive(Debug)]
pub struct Forest {
    pub horizontal: Vec<Vec<usize>>,
    pub vertical: Vec<Vec<usize>>,
}

impl Forest {
    pub fn from_lines(lines: Vec<String>) -> Forest {
        let mut new_forest = Forest {
            horizontal: vec![],
            vertical: vec![],
        };

        let lines_as_usize_vec = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|line_char| line_char.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        new_forest.horizontal.extend(lines_as_usize_vec);

        for column_index in 0..new_forest.horizontal.first().unwrap().len() {
            let mut vertical_line_as_usize_vec: Vec<usize> = vec![];

            for row_index in 0..lines.len() {
                vertical_line_as_usize_vec.push(new_forest.horizontal[row_index][column_index]);
            }

            new_forest.vertical.push(vertical_line_as_usize_vec);
        }

        return new_forest;
    }

    pub fn get_tree_scores(&self) -> HashMap<(usize, usize), i32> {
        let mut tree_scores: HashMap<(usize, usize), i32> = HashMap::new();

        self.horizontal
            .iter()
            .enumerate()
            .for_each(|(row_index, tree_row)| {
                tree_row.iter().enumerate().for_each(|(column_index, _)| {
                    let key = (row_index, column_index);
                    let next_score = Self::get_tree_score(tree_row, column_index);

                    tree_scores
                        .entry(key)
                        .and_modify(|score| *score = *score * next_score)
                        .or_insert(next_score);
                })
            });

        self.vertical
            .iter()
            .enumerate()
            .for_each(|(column_index, tree_row)| {
                tree_row.iter().enumerate().for_each(|(row_index, _)| {
                    let key = (row_index, column_index);
                    let next_score = Self::get_tree_score(tree_row, row_index);

                    tree_scores
                        .entry(key)
                        .and_modify(|score| *score = *score * next_score)
                        .or_insert(next_score);
                })
            });

        return tree_scores;
    }

    fn get_tree_score(trees: &Vec<usize>, tree_index: usize) -> i32 {
        let target_tree = trees[tree_index];
        let mut right_score: i32 = 0;
        let mut left_score: i32 = 0;

        for current_index in (tree_index + 1)..trees.len() {
            if target_tree > trees[current_index] {
                right_score += 1;
            }

            if target_tree <= trees[current_index] {
                right_score += 1;
                break;
            }
        }

        if right_score == 0 {
            return 0;
        }

        for current_index in 0..tree_index {
            let reversed_index = tree_index - current_index - 1;

            if target_tree > trees[reversed_index] {
                left_score += 1;
            }

            if target_tree <= trees[reversed_index] {
                left_score += 1;
                break;
            }
        }

        return left_score * right_score;
    }

    pub fn get_all_visible_trees(&self) -> usize {
        let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

        self.horizontal
            .iter()
            .enumerate()
            .for_each(|(row_index, row)| {
                Self::get_visible_tree_indexes(row)
                    .iter()
                    .for_each(|column_index| {
                        visible_trees.insert((row_index, *column_index));
                    });

                let mut reversed_row = row.clone();
                reversed_row.reverse();

                Self::get_visible_tree_indexes(&reversed_row)
                    .iter()
                    .for_each(|column_index| {
                        let actual_column_index = row.len() - 1 - column_index;
                        visible_trees.insert((row_index, actual_column_index));
                    });
            });

        self.vertical
            .iter()
            .enumerate()
            .for_each(|(column_index, column)| {
                Self::get_visible_tree_indexes(column)
                    .iter()
                    .for_each(|row_index| {
                        visible_trees.insert((*row_index, column_index));
                    });

                let mut reversed_column = column.clone();
                reversed_column.reverse();

                Self::get_visible_tree_indexes(&reversed_column)
                    .iter()
                    .for_each(|row_index| {
                        let actual_row_index = column.len() - 1 - row_index;
                        visible_trees.insert((actual_row_index, column_index));
                    });
            });

        return visible_trees.len();
    }

    fn get_visible_tree_indexes(trees: &Vec<usize>) -> Vec<usize> {
        let mut highest_opt: Option<usize> = None;
        let mut visible_trees: Vec<usize> = vec![];

        trees.iter().enumerate().for_each(|(index, tree)| {
            if let Some(highest) = highest_opt {
                if tree > &highest {
                    visible_trees.push(index);
                    highest_opt = Some(*tree);
                }
            } else {
                visible_trees.push(index);
                highest_opt = Some(*tree);
                return;
            }
        });

        return visible_trees;
    }
}
