fn main() {
    let v1 = vec![1.0,2.0,3.0];
    let v2 = vec![4.0,5.0,6.0];
    let dp = v1.iter().v1.zip(v2).map(|v1,v2|v1*v2).sum();
    println! ("{}",dp);
}
