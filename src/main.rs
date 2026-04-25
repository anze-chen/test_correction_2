fn count_words(s:String){
    let list = s.split_whitespace();
    let counts = list.filter(|x|x.len()>3).count();
    println!("{}", counts);
}

fn main(){
    let sentence:String = "the quick brown fox jumps over the lazy dog".to_string();
    count_words(sentence);
}