mod mat;
mod vec;

#[cfg(test)]
mod tests {
    use super::vec::*;
    #[test]
    fn it_works() {
        let test_vec_1 = Vec2::new([0.5, 1.0]);
        let unit_vec_1 = Vec4::unit_n(1);
        let mut unit_vec_2: Vec3 = Vec3::unit_y();
        unit_vec_2.set_y(2.0);

        println!("{:?}", unit_vec_2);
    }
}
