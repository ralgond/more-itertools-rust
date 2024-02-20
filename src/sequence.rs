
pub trait Sequence<T> 
where T: PartialEq
{
    fn get<'a>(&'a self, index: usize) -> Option<&'a T>;
    fn len(&self) -> usize;
    fn equals(&self, other: &dyn Sequence<T>) -> bool;
    fn slice(&self, begin: usize, end: usize) -> &[T];
    fn as_slice(&self) -> &[T];
}

pub struct SequenceVector<T> {
    v: Vec<T>
}

impl<T> SequenceVector<T> {
    pub(crate) fn new(v: Vec<T>) -> SequenceVector<T> {
        SequenceVector {
            v
        }
    }
}

impl<T> Sequence<T> for SequenceVector<T> 
where T: PartialEq
{
    fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        return self.v.get(index);
    }

    fn len(&self) -> usize {
        return self.v.len();
    }

    fn slice(&self, begin: usize, end: usize) -> &[T] {
        return &self.v[begin..end];
    }

    fn equals(&self, other: &dyn Sequence<T>) -> bool {
        return self.v.len() == other.len() && self.v.starts_with(other.as_slice());
    }

    fn as_slice(&self) -> &[T] {
        return self.v.as_slice();
    }
}

pub fn create_seq_from_vec<T>(v: Vec<T>) -> impl Sequence<T> 
where T: PartialEq
{
    return SequenceVector::new(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3];
        let v = create_seq_from_vec(v);
        assert_eq!(3, v.len());
        assert_eq!(1, *v.get(0).unwrap());
        assert_eq!(2, *v.get(1).unwrap());
        assert_eq!(3, *v.get(2).unwrap());
    }
}