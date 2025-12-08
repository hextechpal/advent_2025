fn main(){
    let v = vec![1,2,3,4,5,6,7];

    for (i, j) in v[3..].iter().enumerate() {
        println!("i={i}, j={j}");
    }

}