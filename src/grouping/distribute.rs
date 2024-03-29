use std::rc::Rc;

use crate::error;
use crate::error::Error;
use crate::sequence::Sequence;

#[allow(dead_code)]
struct DistributeInner<T> 
where
T: Clone + PartialEq + 'static
{
    pub(crate) buf: Box<dyn Sequence<T>>,
    pub(crate) bucket_count: usize,
}

pub struct Distribute<T> 
where
T: Clone + PartialEq + 'static
{
    inner: Rc<DistributeInner<T>>
}

#[allow(dead_code)]
pub struct Cursor<T>
where
T: Clone + PartialEq + 'static
{
    dist_inner: Rc<DistributeInner<T>>,
    cur: usize,
    step: usize
}

impl<T> Iterator for Cursor<T>
where
T: Clone + PartialEq
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.dist_inner.buf.len() {
            return None;
        }

        let ret = usize::overflowing_add(self.cur, self.step);
        if ret.1 {
            // cursor overflow
            return Some(Err(error::overflow_error("cursor overflow".to_string())));
        }

        let real_ret = Some(Ok(self.dist_inner.buf.get(self.cur).unwrap().clone()));
        
        self.cur = ret.0;

        return real_ret;
    }
}

impl<T> Distribute<T> 
where
T: Clone + PartialEq + 'static
{
    pub fn new(buf: Box<dyn Sequence<T>>, bucket_count: usize) -> Self {
        let inner = DistributeInner {
            buf: buf,
            bucket_count: bucket_count,
        };

        let ret = Distribute {
            inner: Rc::new(inner)
        };

        return ret;
    }

    pub fn iter(&self, bucket_no: usize) -> Box<dyn Iterator<Item = Result<T, Error>>> {
        
        let iter: Box<dyn Iterator<Item = Result<T, Error>>> = Box::new(Cursor {
                dist_inner: Rc::clone(&self.inner),
                cur: bucket_no,
                step: self.inner.bucket_count
            });

        return iter;
    }
}

pub fn distribute<T>(buf: Box<dyn Sequence<T>>, bucket_cnt: usize) -> Distribute<T>
where
T: Clone + PartialEq + 'static
{
    let dist = Distribute::new(buf, bucket_cnt);
    return dist;
}


#[cfg(test)]
mod tests {
    use crate::sequence::create_seq_from_vec;

    use super::*;

    #[test]
    fn test1() {
        let v = create_seq_from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
        let dist = distribute(v, 3);

        let mut cur_0 = dist.iter(0);
        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(4, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(7, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(10, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_0.next());
    }

    #[test]
    fn test2() {
        let v = create_seq_from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
        let dist = distribute(v, 3);

        let mut cur_0: Box<dyn Iterator<Item = Result<i32, Error>>> = dist.iter(0);
        let mut cur_1 = dist.iter(1);
        let mut cur_2 = dist.iter(2);
        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(2, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(3, cur_2.next().unwrap().ok().unwrap());


        assert_eq!(4, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(5, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(6, cur_2.next().unwrap().ok().unwrap());
        
        
        assert_eq!(7, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(8, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(9, cur_2.next().unwrap().ok().unwrap());

        assert_eq!(10, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_1.next());
        assert_eq!(None, cur_2.next());
        
        assert_eq!(None, cur_0.next());
    }

    #[test]
    fn test3() {
        let v = create_seq_from_vec(vec![1,2,3]);
        let dist = distribute(v, 5);

        let mut cur_0 = dist.iter(0);
        let mut cur_1 = dist.iter(1);
        let mut cur_2 = dist.iter(2);
        let mut cur_3 = dist.iter(3);
        let mut cur_4 = dist.iter(4);

        assert_eq!(1, cur_0.next().unwrap().ok().unwrap());
        assert_eq!(2, cur_1.next().unwrap().ok().unwrap());
        assert_eq!(3, cur_2.next().unwrap().ok().unwrap());
        assert_eq!(None, cur_3.next());
        assert_eq!(None, cur_4.next());
    }
}