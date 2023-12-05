use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Input file not found...");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line..."))
        .collect()
}

pub struct PartNumber {
    value: i32,
    row: i32,
    column: i32, // the first column from the right where the number starts
}

impl PartNumber {
    pub fn is_valid(&self, engine_schematic: &Vec<&str>) -> bool {
        // When our part number is not on any boundary (i.e., top, bottom, left, right)
        if self.row > 0 && self.row < 140 && self.column > 0 && self.column < 137 {
            let row_range = (self.row - 1)..(self.row + 1);

            for i in row_range {
                if self.row == i {
                    if is_symbol(engine_schematic, i, self.column - 1)
                        || is_symbol(engine_schematic, i, self.column + 3)
                    {
                        return true;
                    }
                } else {
                    let column_range = (self.column - 1)..(self.column + 3);

                    for j in column_range {}
                }
            }
        }
        true
    }
}
