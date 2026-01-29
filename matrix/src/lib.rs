pub struct Matrix {
    data: String,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Matrix {
            data: input.to_string(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        let is_one_liner = !self.data.contains('\n');

        if is_one_liner {
            self.data.parse::<u32>().ok().map(|n| vec![n])
        } else {
            let rows: Vec<&str> = self.data.split('\n').collect();
            if row_no == 0 || row_no > rows.len() {
                return None;
            }

            let row_data = rows[row_no - 1];
            let numbers: Vec<u32> = row_data
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            if numbers.is_empty() {
                None
            } else {
                Some(numbers)
            }
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let is_one_liner = !self.data.contains('\n');

        if is_one_liner {
            self.data.parse::<u32>().ok().map(|n| vec![n])
        } else {
            let rows: Vec<&str> = self.data.split('\n').collect();
            let mut column_data = Vec::new();

            for row in rows {
                let numbers: Vec<u32> = row
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect();

                if col_no == 0 || col_no > numbers.len() {
                    return None;
                }

                column_data.push(numbers[col_no - 1]);
            }

            if column_data.is_empty() {
                None
            } else {
                Some(column_data)
            }
        }
    }
}
