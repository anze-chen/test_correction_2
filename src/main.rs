fn count_words(s:String){
    let list = s.split_whitespace();
    let counts = list.iter().(filter|x|x.len()>3).count();
    println!("{}", counts);
}