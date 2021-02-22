use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DVector<T> {
    pub data: Vec<T>,
}

#[derive(Debug)]
pub struct DMatrix<T> {
    pub data: Vec<Vec<T>>,
    pub size: (usize, usize),
}

impl<T> DMatrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let mut size = (0, 0);
        size.1 = data.len();
        if data.len() != 0 {
            size.0 = data.get(0).unwrap().len()
        }

        Self { data, size }
    }
}

impl<T: Display + Copy> Display for DMatrix<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let biggest = format!("{}", self.data.get(1).unwrap().get(1).unwrap()).len();
        for m in 0..self.size.0 {
            string.push_str("|");
            for n in 0..self.size.1 {
                if n == self.size.1 - 1 {
                    &string.push_str(&format!("{}", self.data.get(n).unwrap().get(m).unwrap()));
                    break;
                }
                let current = format!("{}", self.data.get(n).unwrap().get(m).unwrap()).len();

                string.push_str(&format!("{} ", self.data.get(n).unwrap().get(m).unwrap()));

                for _ in current..biggest + 1 {
                    string.push(' ');
                }
            }
            &string.push_str("|\n");
        }
        write!(f, "{}", string)
    }
}


//   ----> N
//  | 3  2 |
//  | 1  4 |
//  | 2  9 |
//