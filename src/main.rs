struct StringBuilder{
    c=content:String,
}

impl StringBuilder{
    fn new() -> StringBuilder{
        let c = StringBuilder("")
        c}
        ///Sorry, I don't remember why would I wrote a c in here. It doesn't make any sense to me right now.
    fn add(&mut self, text:&str){
            let new_string = self.push_str(text);
            return new_string;
        }
    fn build(&self) -> String{
            let copy = self.clone();
            return copy;}
        }
fn main(){let word=""::StringBuilder::new();
        let new_word = word::StringBuilder::add("Hello, ");
        let new_word = new_word::StringBuilder::add("World!");
        println!("{:?}",new_word);
}