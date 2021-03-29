#[cfg(feature = "algebra")]
pub mod algebra {
    use crate::algebra::linear::{Point, SquareMatrix, Vector};
    use mint::{
        ColumnMatrix2, ColumnMatrix3, ColumnMatrix4, Point2, Point3, Vector2, Vector3, Vector4,
    };

    impl<T> From<Point2<T>> for Point<T, 2> {
        fn from(rhs: Point2<T>) -> Self {
            Self::from([rhs.x, rhs.y])
        }
    }

    impl<T> From<Point3<T>> for Point<T, 3> {
        fn from(rhs: Point3<T>) -> Self {
            Self::from([rhs.x, rhs.y, rhs.z])
        }
    }

    impl<T> From<Vector2<T>> for Vector<T, 2> {
        fn from(rhs: Vector2<T>) -> Self {
            Self::from([rhs.x, rhs.y])
        }
    }

    impl<T> From<Vector3<T>> for Vector<T, 3> {
        fn from(rhs: Vector3<T>) -> Self {
            Self::from([rhs.x, rhs.y, rhs.z])
        }
    }

    impl<T> From<Vector4<T>> for Vector<T, 4> {
        fn from(rhs: Vector4<T>) -> Self {
            Self::from([rhs.x, rhs.y, rhs.z, rhs.w])
        }
    }

    impl<T> From<ColumnMatrix2<T>> for SquareMatrix<T, 2> {
        fn from(rhs: ColumnMatrix2<T>) -> Self {
            let col_1 = [rhs.x.x, rhs.x.y];
            let col_2 = [rhs.y.x, rhs.y.y];
            Self::new([col_1, col_2])
        }
    }

    impl<T> From<ColumnMatrix3<T>> for SquareMatrix<T, 3> {
        fn from(rhs: ColumnMatrix3<T>) -> Self {
            let col_1 = [rhs.x.x, rhs.x.y, rhs.x.z];
            let col_2 = [rhs.y.x, rhs.y.y, rhs.y.z];
            let col_3 = [rhs.z.x, rhs.z.y, rhs.z.z];
            Self::new([col_1, col_2, col_3])
        }
    }

    impl<T> From<ColumnMatrix4<T>> for SquareMatrix<T, 4> {
        fn from(rhs: ColumnMatrix4<T>) -> Self {
            let col_1 = [rhs.x.x, rhs.x.y, rhs.x.z, rhs.x.w];
            let col_2 = [rhs.y.x, rhs.y.y, rhs.y.z, rhs.y.w];
            let col_3 = [rhs.z.x, rhs.z.y, rhs.z.z, rhs.z.w];
            let col_4 = [rhs.w.x, rhs.w.y, rhs.w.z, rhs.w.w];
            Self::new([col_1, col_2, col_3, col_4])
        }
    }

    impl<T: Copy> From<Point<T, 2>> for Point2<T> {
        fn from(rhs: Point<T, 2>) -> Self {
            Self::from(rhs.data[0])
        }
    }

    impl<T: Copy> From<Point<T, 3>> for Point3<T> {
        fn from(rhs: Point<T, 3>) -> Self {
            Self::from(rhs.data[0])
        }
    }

    impl<T: Copy> From<Vector<T, 2>> for Vector2<T> {
        fn from(rhs: Vector<T, 2>) -> Self {
            Self::from(rhs.data[0])
        }
    }

    impl<T: Copy> From<Vector<T, 3>> for Vector3<T> {
        fn from(rhs: Vector<T, 3>) -> Self {
            Self::from(rhs.data[0])
        }
    }

    impl<T: Copy> From<Vector<T, 4>> for Vector4<T> {
        fn from(rhs: Vector<T, 4>) -> Self {
            Self::from(rhs.data[0])
        }
    }

    impl<T: Copy> From<SquareMatrix<T, 2>> for ColumnMatrix2<T> {
        fn from(rhs: SquareMatrix<T, 2>) -> Self {
            let array = [rhs[[0, 0]], rhs[[0, 1]], rhs[[1, 0]], rhs[[1, 1]]];
            Self::from(array)
        }
    }

    impl<T: Copy> From<SquareMatrix<T, 3>> for ColumnMatrix3<T> {
        fn from(rhs: SquareMatrix<T, 3>) -> Self {
            let array = [
                rhs[[0, 0]],
                rhs[[0, 1]],
                rhs[[0, 2]],
                rhs[[1, 0]],
                rhs[[1, 1]],
                rhs[[1, 2]],
                rhs[[2, 0]],
                rhs[[2, 1]],
                rhs[[2, 2]],
            ];
            Self::from(array)
        }
    }

    impl<T: Copy> From<SquareMatrix<T, 4>> for ColumnMatrix4<T> {
        fn from(rhs: SquareMatrix<T, 4>) -> Self {
            let array = [
                rhs[[0, 0]],
                rhs[[0, 1]],
                rhs[[0, 2]],
                rhs[[0, 3]],
                rhs[[1, 0]],
                rhs[[1, 1]],
                rhs[[1, 2]],
                rhs[[1, 3]],
                rhs[[2, 0]],
                rhs[[2, 1]],
                rhs[[2, 2]],
                rhs[[2, 3]],
                rhs[[3, 0]],
                rhs[[3, 1]],
                rhs[[3, 2]],
                rhs[[3, 3]],
            ];
            Self::from(array)
        }
    }
}
