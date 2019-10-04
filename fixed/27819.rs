pub fn anagrams_for(input: String, inputs: &[str]) -> Vec<&str> {
    //vec![]; // either this line or the following crashes the compiler
    Vec::new()
}

fn main(){
    let inputs = ["hello", "world", "zombies", "pants"];
    let outputs: Vec<&str> = vec![];
    assert_eq!(anagrams_for("diaper", &inputs), outputs);
}
