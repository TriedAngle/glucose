use crate::numeric::num::Num;
use crate::{Matrix, Vector};
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::marker::PhantomData;

impl<T: Num + Serialize, const N: usize> Serialize for Vector<T, { N }> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N))?;
        for e in &self.data {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct VectorVisitor<T, const N: usize> {
    marker: PhantomData<T>,
}

impl<T, const N: usize> VectorVisitor<T, { N }> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}
impl<'de, T: Num + Default + Deserialize<'de> + Copy, const N: usize> Deserialize<'de>
    for Vector<T, { N }>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VectorVisitor::new())
    }
}

impl<'de, T: Num + Default + Deserialize<'de> + Copy, const N: usize> Visitor<'de>
    for VectorVisitor<T, { N }>
{
    type Value = Vector<T, { N }>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("vector")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Vector<T, { N }>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vector::default();
        for i in 0..N {
            vec[i] = seq.next_element()?.unwrap();
        }
        Ok(vec)
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::Vector;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn vec_serialize_deserialize() {
        let vec = Vector::new([0.1, 0.4, -2.0]);
        assert_tokens(
            &vec,
            &[
                Token::Seq { len: Some(3) },
                Token::F64(0.1),
                Token::F64(0.4),
                Token::F64(-2.0),
            ],
        );
    }
}
