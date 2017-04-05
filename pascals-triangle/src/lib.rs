pub struct PascalsTriangle {
    row_count: usize
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count as usize }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vec: Vec<Vec<u32>> = Vec::with_capacity(self.row_count);
        if self.row_count == 0 {
            return vec;
        }
        vec.push(vec![1]);
        if self.row_count == 1 {
            return vec;
        }
        for i in 1..self.row_count {
            let max_length = i + 1;
            let mut sub_vec: Vec<u32> = Vec::with_capacity(max_length);
            // fill first with 1
            sub_vec.push(1);
            if max_length > 2 {
                let previous_vec = &vec[i - 1];
                for j in 1..previous_vec.len() {
                    let total = previous_vec[j - 1] + previous_vec[j];
                    sub_vec.push(total);
                }
            }
            // fill last element with 1
            sub_vec.push(1);
            vec.push(sub_vec);
        }
        return vec;
    }
}
