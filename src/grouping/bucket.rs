use std::{collections::HashMap, hash::Hash, rc::Rc};

#[allow(dead_code)]
pub struct BucketInner<T>
where 
T: Clone + PartialEq + Eq + Hash
{
    buf: Vec<Vec<T>>, 
    key_func: fn(&Vec<T>) -> Vec<T>,
    table: HashMap<Vec<T>, Vec<Vec<T>>>
}

pub struct Bucket<T>
where 
T: Clone + PartialEq + Eq + Hash {
    inner: Rc<BucketInner<T>>
}

pub struct Cursor<T> 
where
T: Clone + PartialEq + Eq + Hash
{
    inner: Rc<BucketInner<T>>
}

impl<T> Cursor<T> 
where
T: Clone + PartialEq + Eq + Hash {
    pub fn get(&self, key: &Vec<T>) -> Option<&Vec<Vec<T>>> {
        return self.inner.table.get(key);
    }
}


impl<T> Bucket<T> 
where 
T: Clone + PartialEq + Eq + Hash
{
    pub fn new(buf: Vec<Vec<T>>, key_func: fn(&Vec<T>)->Vec<T>) -> Self {
        let mut inner = BucketInner {
            buf: buf,
            key_func: key_func,
            table: HashMap::new()
        };

        for i in inner.buf.iter() {
            let _value = i;
            let _key = key_func(_value);
            let v = inner.table.entry(_key).or_insert(vec![]);
            v.push(_value.clone());
        }

        let ret = Bucket {
            inner: Rc::new(inner)
        };

        return ret;
    }

    pub fn keys(&self) -> Vec<Vec<T>> {
        let mut ret = Vec::new();
        for k in self.inner.table.keys() {
            ret.push(k.clone());
        }
        return ret;
    } 

    pub fn get_cursor(&self) -> Cursor<T> {
        Cursor {
            inner: Rc::clone(&self.inner)
        }
    }
}

pub fn bucket<T>(buf: Vec<Vec<T>>, key: fn(&Vec<T>)->Vec<T>) -> Bucket<T> 
where 
T: Clone + PartialEq + Eq + Hash
{
    return Bucket::new(buf, key);
}


#[cfg(test)]
mod tests {
    use crate::utils::join_char_vec_second_level;

    use super::*;

    #[test]
    fn test1() {
        let v = vec!["a1", "b1", "c1", "a2", "b2", "c2", "b3"];
        let mut v2: Vec<Vec<char>> = Vec::new();
        for a in v.iter() {
            let char_vec: Vec<char> = a.chars().collect();
            v2.push(char_vec);
        }

        println!("{:?}", v2);

        let b = bucket(v2, 
            |x| { vec![(*(x.get(0).unwrap()))] });

        println!("{:?}", join_char_vec_second_level(&b.keys()));

        let cursor = b.get_cursor();
        let m = cursor.get(&vec!['a']).unwrap();
        assert_eq!(vec!["a1", "a2"], join_char_vec_second_level(m));

        let cursor = b.get_cursor();
        let m = cursor.get(&vec!['b']).unwrap();
        assert_eq!(vec!["b1", "b2", "b3"], join_char_vec_second_level(m));

        let cursor = b.get_cursor();
        let m = cursor.get(&vec!['c']).unwrap();
        assert_eq!(vec!["c1", "c2"], join_char_vec_second_level(m));
    }
}