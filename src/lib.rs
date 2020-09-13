mod mat;
mod vec;

#[cfg(test)]
mod tests {
    use super::vec::*;
    #[test]
    fn create_vectors() {
        let vec_1 = Vec3::new([0.0, 1.0, 2.0]);
        let vec_2 = Vec3::unit_y();

        assert_eq!(vec_2.data, [0.0, 1.0, 0.0]);
    }

    #[test]
    fn add_vectors() {
        let vec_1 = Vec3::new([1.0, 3.0, 2.5]);
        let vec_2 = Vec3::new([2.0, 3.0, -2.5]);
        let vec_3 = vec_2 + vec_1;

        assert_eq!(vec_3.data, [3.0, 6.0, 0.0]);
    }

    #[test]
    fn dot_vectors() {
        let vec_1 = Vec3::new([1.0, 3.0, 1.0]);
        let vec_2 = Vec3::new([2.0, 3.0, 1.0]);
        let dot = vec_1.dot(&vec_2);

        assert_eq!(dot, 12.0);
    }
}
