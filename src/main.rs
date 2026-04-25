fn safe_divide(a:f64,b:f64) -> Option<f64>{
    match b{
        0.0 => None,
        _ => Some(a/b),
    }
}

fn main(){
    match safe_divide(10.0,24.0){
        Some(result) => println!("Result: {}", result),
        None => println!("Error: cannot divide by zero"),
    }
}
