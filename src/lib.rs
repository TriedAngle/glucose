mod vec;
mod mat;

#[cfg(test)]
mod tests {
    use super::vec::*;
    #[test]
    fn it_works() {
        let test_vec_1 = Vec2::new([0.5, 1.0]);
        println!("{:?}", test_vec_1);
    }
}
