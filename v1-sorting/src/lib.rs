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

#[cfg(test)]
mod tests {
    use super::*;
   #[test]
    fn test_bubble_sort() {
        let mut v = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut v);
        assert_eq!(v, vec![7,9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }
}
