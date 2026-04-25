fn safe_divide(a:f64,b:f64) -> Option<f64>{
    match b:f64{
        b = 0.0 => Err("Can't divide by zero"),
        _ => Some(a/b),
    }
}
