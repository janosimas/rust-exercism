pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count < 1 {
            return PascalsTriangle { rows: vec![] };
        }

        let rows: Vec<Vec<u32>> = std::iter::repeat(()).take((row_count - 1) as usize).fold(
            vec![vec![1]],
            |mut vec, _| {
                let mut row: Vec<u32> = vec
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|a| a.iter().sum())
                    .collect();
                row.insert(0, 1);
                row.push(1);
                vec.push(row);

                vec
            },
        );

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
