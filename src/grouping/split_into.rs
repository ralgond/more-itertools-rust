use std::{collections::LinkedList, fmt::Debug};

#[derive(Debug, Clone)]
struct SplitIntoOutputItem<I: Iterator> {
    items: Vec<I::Item>,
    size: usize,
    finished: bool
}

#[derive(Debug, Clone)]
pub struct SplitInto<I: Iterator>
where I::Item: Debug + Clone
{
    ret_buf: LinkedList<SplitIntoOutputItem<I>>,
    iter: I,
    sizes: Vec<usize>,
    iter_finished: bool
}

impl<I: Iterator> Iterator for SplitInto<I> 
where I::Item: Debug + Clone
{
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter_finished {
                if self.ret_buf.len() > 0 {
                    let front = self.ret_buf.pop_front();
                    return Some(front.unwrap().items);
                } else {
                    return None;
                }
            }

            if self.ret_buf.len() > 0 {
                let front = self.ret_buf.front_mut().unwrap();
                if front.items.len() == front.size {
                    front.finished = true;
                }

                if front.finished {
                    let front = self.ret_buf.pop_front();
                    return Some(front.unwrap().items);
                }
            }

            let _next = self.iter.next();
            match _next {
                None => {
                    self.iter_finished = true;
                    for item in self.ret_buf.iter_mut() {
                        item.finished = true;
                    }
                    continue;
                },
                Some(v) => {
                    if self.ret_buf.len() == 0 {
                        continue;
                    }

                    let front = self.ret_buf.front_mut();
                    front.unwrap().items.push(v);

                    continue;
                }
            }
        }
    }
}

pub fn split_into<I>(iter: I, sizes: Vec<usize>) -> SplitInto<I>
where
    I: Iterator,
    I::Item: Debug + Clone
{
    let mut ret = SplitInto {
        ret_buf: LinkedList::new(),
        iter: iter,
        sizes: sizes.clone(),
        iter_finished: false
    };

    for size in ret.sizes.iter() {
        ret.ret_buf.push_back(SplitIntoOutputItem {
            finished: false,
            size: *size,
            items: Vec::new()
        });
    }

    return ret;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,3,4,5,6];
        let sizes = vec![1,2,3];
        let si = split_into(v.into_iter(), sizes);
        let ret = si.collect::<Vec<_>>();
        assert_eq!(vec![vec![1], vec![2, 3], vec![4, 5, 6]], ret);

        let v = vec![1,2,3,4,5,6];
        let sizes = vec![2,3];
        let si = split_into(v.into_iter(), sizes);
        let ret = si.collect::<Vec<_>>();
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1, 2], vec![3, 4, 5]], ret);

        let v = vec![1,2,3,4];
        let sizes = vec![1,2,3,4];
        let si = split_into(v.into_iter(), sizes);
        let ret = si.collect::<Vec<_>>();
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1], vec![2, 3], vec![4], vec![]], ret);

        let v = vec![1,2,3,4];
        let sizes = vec![1,2,0,3,4];
        let si = split_into(v.into_iter(), sizes);
        let ret = si.collect::<Vec<_>>();
        // println!("{:?}", ret);
        assert_eq!(vec![vec![1], vec![2, 3], vec![], vec![4], vec![]], ret);
    }

}