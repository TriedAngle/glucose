mod mat;
mod vec;

#[cfg(test)]
mod tests {
    use super::vec::*;
    use std::ops::{Neg};

    #[test]
    fn create_vectors() {
        let vec_1 = Vec3::new([0.0, 1.0, 2.0]);
        let vec_2 = Vec3::unit_y();

        assert_eq!(vec_1.data, [0.0, 1.0, 2.0]);
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
        let dot = vec_1.dot(vec_2);

        assert_eq!(dot, 12.0);
    }

    #[test]
    fn scale_vectors() {
        let mut vec_1 = Vec3::new([1.0, 3.0, 1.0]);
        let vec_2 = Vec3::new([2.0, 6.0, 2.0]);
        vec_1 = 2.0 * vec_1;

        let vec_3 = vec_2.neg();

        assert_eq!(vec_1, vec_2);
        assert_eq!(vec_3.data, [-2.0, -6.0, -2.0])
    }

    #[test]
    fn normalize_vectors() {
        let mut vec_1 = Vec3::new([2.0, 3.0, 1.0]);
        let vec_1_normalized = vec_1.normalized();
        vec_1.normalize();

        assert_eq!(vec_1.data, [0.5345225, 0.8017837, 0.26726124]);
        assert_eq!(vec_1, vec_1_normalized);
    }

    #[test]
    fn split_vectors() {
        let vec_1 = Vec3::new([2.0, 3.0, 1.0]);
        let vec_2 = Vec4::new([2.0, 3.0, 1.0, 1.4]);
        let vec_3 = vec_1.xy();
        let vec_4 = vec_2.yzw();
        assert_eq!(vec_3.data, [2.0, 3.0]);
        assert_eq!(vec_4.data, [3.0, 1.0, 1.4]);
    }
}
