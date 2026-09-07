// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    //okay let's experiment 
    let vecto = vec![10, 20, 30];
    println!("vecto before: {:?}", vecto)
    let vecto2 = fill_vec(vecto);
    println!("vectoris anoris mantoris extrolis: {:?}", vecto2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
