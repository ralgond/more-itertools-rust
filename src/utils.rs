use crate::error::Error;
use std::collections::HashMap;
use std::hash::Hash;

pub fn extract_value_from_result_vec<T>(vec: Vec<Result<T, Error>>) -> (Vec<T>, bool) {
    let mut ret_vec = Vec::new();
    for v in vec.into_iter() {
        match v {
            Err(_) => { return (ret_vec, true); },
            Ok(v2) => { ret_vec.push(v2); }
        }
    }
    return (ret_vec, false);
}

pub fn join_string_vec(v: &Vec<char>) -> String{
    return v.iter().collect();
}

pub fn join_char_vec_second_level(l: &Vec<Vec<char>>) -> Vec<String> {
    let mut ret = vec![];
    for item in l {
        ret.push(join_string_vec(item));
    }
    return ret;
}

pub fn any(v: &Vec<bool>) -> bool {
    for i in v.iter() {
        if *i {
            return true;
        }
    }
    return false;
}


pub fn any_result(v: Result<Vec<bool>, Error>) -> bool {
    if v.is_err() {
        return false;
    }
    return any(&(v.as_ref().ok().unwrap()));
}



pub fn are_same<T>(op1: Option<&T> , op2: Option<&T>) -> bool 
where T: PartialEq
{
    match (op1, op2) {
        (None, None) => { return true; },
        (Some(v1), Some(v2)) => { return *v1 == *v2; },
        _ => { return false; }
    }
}

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    return indices;
}

pub fn counter<T>(hm: &mut HashMap<T, usize>, mut iter: Box<dyn Iterator<Item = T>>)
where T: Hash + Eq + PartialEq
{
    loop {
        let _next = iter.next();
        match _next {
            None => {
                return;
            },
            Some(key) => {
                hm.entry(key).and_modify(|x| {*x += 1}).or_insert(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error, itertools::iter::iter_from_vec};

    #[test]
    fn test1() {
        let v = vec![Ok(4),Ok(3),Ok(3)];
        let a = extract_value_from_result_vec(v);
        assert!(!a.1);
        assert_eq!(vec![4,3,3], a.0);

        let v = vec![Ok(4),Err(error::any_error(error::Kind::OverflowError, "Overflow".to_string())),Ok(3)];
        let a = extract_value_from_result_vec(v);
        assert!(a.1);
        assert_eq!(vec![4], a.0);
    }

    #[test]
    fn test2() {
        let v = vec!['a', 'b', 'c'];
        assert_eq!(join_string_vec(&v), "abc".to_string());
    }

    #[test]
    fn test_any() {
        let v = vec![true,true];
        assert_eq!(true, any(&v));

        let v = vec![true,false];
        assert_eq!(true, any(&v));

        let v: Vec<bool> = vec![false,false];
        assert_eq!(false, any(&v));

    }

    #[test]
    fn test_are_same() {
        assert!(are_same(None::<&i32>, None));
        assert!(!are_same(None, Some(&1)));
        assert!(!are_same(Some(&1), None));
        assert!(are_same(Some(&1), Some(&1)));
        assert!(!are_same(Some(&2), Some(&1)));
    }

    #[test]
    fn test_argsort() {
        let v = [3,1,2,4];
        let ret = argsort(v.as_slice());
        assert_eq!(vec![1usize,2usize,0usize,3usize], ret);
    }

    #[test]
    fn test_counter() {
        let v = vec![5,1,2,2,3,3,3,4,4,4,4,5,5,5,5];
        let mut hm = HashMap::new();
        counter(&mut hm, iter_from_vec(v));
        assert_eq!(&1, hm.get(&1).unwrap());
        assert_eq!(&2, hm.get(&2).unwrap());
        assert_eq!(&3, hm.get(&3).unwrap());
        assert_eq!(&4, hm.get(&4).unwrap());
        assert_eq!(&5, hm.get(&5).unwrap());
    }
}