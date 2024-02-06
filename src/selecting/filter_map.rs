use crate::error::Error;
use crate::error;

#[derive(Debug, Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct FilterMap<I: Iterator, T> {
    // cur: usize,
    iter: I,
    func: fn(item: &I::Item) -> Result<T, Error>,
    failed: bool
}

impl<I: Iterator, T> Iterator for FilterMap<I, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }

        loop {
            let ret = self.iter.next();
            match ret {
                None => { return None; }
                Some(v) => {
                    let res_func_ret = (self.func)(&v);
                    match res_func_ret {
                        Ok(func_ret) => {
                            return Some(Ok(func_ret));
                        },
                        Err(e) => {
                            self.failed = true;
                            match e.message() {
                                None => { return Some(Err(error::any_error(e.kind(), "func failed".to_string()))); }
                                Some(m) => { return Some(Err(error::any_error(e.kind(), "func failed: ".to_string()+m))); }
                            }
                        }
                    }
                }
            }
        }
    }

}

pub fn filter_map<I, T>(iterable: I, 
    func: fn(item: &I::Item) -> Result<T, Error>) -> FilterMap<I::IntoIter, T> 
where
I: IntoIterator,
{  
    FilterMap {
        // cur: 0,
        iter: iterable.into_iter(),
        func: func,
        failed: false
    }
}


mod tests {
    use super::*;

    #[test]
    fn test1() {
        let iterable = vec!["1", "2", "three", "4", "5"];
        let mut fm = filter_map(iterable,
            |x| {
                let ret = x.parse::<i32>();
                match ret {
                    Ok(v) => { return Ok(v); },
                    Err(e) => { return Err(error::value_error(e.to_string())); }
                }
            }
        );

        match fm.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!(1, v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }

        match fm.next() {
            Some(v) => {
                match v {
                    Ok(v2) => { assert_eq!(2, v2) }
                    Err(_) => { assert!(false); }
                }
            },
            None => {}
        }

        match fm.next() {
            Some(v) => {
                match v {
                    Ok(_) => { assert!(false); }
                    Err(e) => { 
                        assert!(true);
                        println!("{:?}", e); 
                    }
                }
            },
            None => {}
        }

        match fm.next() {
            Some(_) => { assert!(false);},
            None => { assert!(true); }
        }
        match fm.next() {
            Some(_) => { assert!(false);},
            None => { assert!(true); }
        }


    }
}