fn main() {
    let word = "test";

    println!("Remove vowels: {:?}", remove_vowels(word))
    //  println!("middle is {}", get_middle(word));
}

fn get_middle(word: &str) -> &str {
    let count = word.len();
    let from = (count - 1) / 2;
    let to = (count / 2) + 1;

    &word[from..to]
}

fn remove_vowels(word: &str) {
    let vowels = vec!["a", "e", "i", "o", "u"];
    let mut new_word = vec!["a", "e"];

    for &letter in word {
        println!("this is letter: {}", letter)
        if !vowels.contains(letter) {
            new_word.push(letter)
        }
    }

    let ret = new_word.join("").as_str();
}
