// Convert strings to pig latin. The first consonant of each  word is moved to the end of the word,
// and "ay" is added, so "first" becomes "irst-fay." Words that start with a vowel  have "hay" added
// to the end instead ("apple" becomes  "apple-hay"). Keep in mind the details about UTF-8 encoding!

fn piggify(word: &str) -> String {
    let mut pig = String::new();

    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    
    if vowels.iter().any(|&v| word.starts_with(v)) {
        pig.push_str(word);
        pig.push_str("-hay");
    } else {
        let mut letters = word.chars();
        let c = letters.next().unwrap();
        pig.push_str(letters.as_str());
        pig.push('-');
        pig.push(c);
        pig.push_str("ay");
    }

    pig
}

fn main() {
    let mut word = String::new();
    println!("Enter a word, and I'll translate it into pig-latin: ");
    std::io::stdin().read_line(&mut word).unwrap();

    let pig = piggify(&(word.trim()).to_lowercase());
    println!("{}", pig);
}
