fn first_word_len(s:&str) -> (u32){
    if s.contains(" "){
        for i in 1.. = s.len(){
            if s[i] == " "{
                return (i);}
        }
    }
    else {return s.len();}
}