struct StringBuilder{
    content: String,
}

impl StringBuilder{
    fn new() -> StringBuilder{
        StringBuilder{
            content: String::new(),
            }
        }
        
    fn add(&mut self, text:&str){
            self.content.push_str(text);
        }
    fn build(&self) -> String{
            self.content.clone()
        }
    }
fn main(){let mut word=StringBuilder::new();
        word.add("Hello, ");
        word.add("World!");

        let new_word = word.build();
        println!("{}",new_word);}