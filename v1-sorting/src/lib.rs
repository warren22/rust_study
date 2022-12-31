use std::fmt::Debug;
pub fn bubble_sort<T:PartialOrd + Debug>(v: &mut [T]){
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len()-1)-p {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                sorted = false;
            }    
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }        
}
pub fn merge_sort<T:PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T>{
    println!("MS:{:?}",v);
    if v.len() <=1 {
        return  v;
     }
     
     let mut res = Vec::with_capacity(v.len());
     let b = v.split_off(v.len() / 2);
     let a = merge_sort(v);
     let b = merge_sort(b);

     let mut a_it = a.into_iter();
     let mut b_it = b.into_iter();
     let mut a_peek = a_it.next();
     let mut b_peek = b_it.next();

     loop {
        match a_peek {
            Some(ref a_val) => match b_peek{
                Some(ref b_val) => {
                    if b_val <a_val{
                       res.push(b_peek.take().unwrap());
                       b_peek = b_it.next();
                    }else{
                       res.push(a_peek.take().unwrap()); 
                       a_peek = a_it.next();
                    }                  
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return  res ;
                } 
            },
            None =>{
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
     }
}

#[cfg(test)]
mod tests {
    use super::*;
   #[test]
    fn test_bubble_sort() {
        let mut v = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut v);
        assert_eq!(v, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_merge_sort() {
        let  v = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        let  v = merge_sort(v);
        assert_eq!(v, vec![7,9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }
}
