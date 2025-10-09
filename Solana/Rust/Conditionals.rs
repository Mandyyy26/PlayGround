fn main() {
    let is_even = true;
    if is_even {
        println!("the number is even");
    } else if !is_even {
        println!("the number is odd");
    }
}

fn main() {
    for i in 0..100 {
        print!("{}", i);
    }
}

fn main() {
    // iteration on arrays, maos ,striings
    let sentence: String = String::from("my name is Mandeep");
    let first_word: () = get_first_word(sentence);
    print!("First word: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(String::from(char.to_string().as_str()));
        if char == ' ' {
            break;
        }
    }
    return ans;
}
