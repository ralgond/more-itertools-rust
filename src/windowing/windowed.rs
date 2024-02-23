use std::collections::VecDeque;
use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
pub struct Windowed<I>
where 
    I: Iterator,
    I::Item: Clone
 {
    buf: VecDeque<I::Item>,
    iter: I,
    n: usize,
    step: usize,
    cache_first_window: bool
}

impl<I> Iterator for Windowed<I> 
where 
    I: Iterator,
    I::Item: Clone
{
    type Item = Result<Vec<<I as Iterator>::Item>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return Some(Err(error::value_error(String::from("n should not be 0."))));
        }

        if !self.cache_first_window {
            for _ in 0..self.n {
                match self.iter.next() {
                    Some(v) => {
                        self.buf.push_back(v);
                    },
                    None => {
                        break;
                    }
                }
            }
            self.cache_first_window = true;

            if self.buf.len() < self.n {
                return None;
            }
            let mut ret = Vec::new();
            for v in self.buf.iter() {
                ret.push(v.clone());
            }
            return Some(Ok(ret));
        }

        

        for _ in 0..self.step {
            self.buf.pop_front();
            match self.iter.next() {
                Some(v) => {
                    self.buf.push_back(v);
                },
                None => {
                    break;
                }
            }
        }



        if self.buf.len() < self.n {
            return None;
        }
        let mut ret = Vec::new();
        for v in self.buf.iter() {
            ret.push(v.clone());
        }
        return Some(Ok(ret));
    }
}

/// https://more-itertools.readthedocs.io/en/v10.2.0/api.html#more_itertools.windowed
pub fn windowed<I>(iterable: I, n: usize, step: usize) -> Windowed<I::IntoIter>
where
    I: IntoIterator,
    I::Item: Clone
{
    Windowed {
        buf: VecDeque::new(),
        iter: iterable.into_iter(),
        n: n,
        step: step,
        cache_first_window: false
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5];
        let mut w = windowed(v, 3, 1);

        assert_eq!(vec![1,2,3], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![2,3,4], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![3,4,5], w.next().unwrap().ok().unwrap());
        assert_eq!(None, w.next());
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3,4,5,6,7,8];
        let mut w = windowed(v, 3, 2);
        assert_eq!(vec![1,2,3], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![3,4,5], w.next().unwrap().ok().unwrap());
        assert_eq!(vec![5,6,7], w.next().unwrap().ok().unwrap());
        assert_eq!(None, w.next());
    }

    #[test]
    fn test3() {
        let v = vec![1,2];
        let mut w = windowed(v, 3, 1);
        assert_eq!(None, w.next());

        let v = vec![1,2];
        let mut w = windowed(v, 0, 1);
        assert_eq!(error::Kind::ValueError, w.next().unwrap().err().unwrap().kind());
    }
}