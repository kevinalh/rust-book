fn main() {
    // -- Ownership --
    let mut s1 = String::from("hello");
    s1.push_str(", world!");

    // Without using references
    let (length, s1) = get_length_verbose(s1);
    println!("{} of size {}", s1, length);

    // Using references
    let length = get_length_referenced(&s1);
    println!("{} of size {}", s1, length);

    // -- Slices --
    // Getting location of first word via an index variable
    let mut s2 = String::from("hello world");
    let first_word_index = first_word_with_index(&s2);
    println!(
        "The index of the first word ending in '{}' is {}",
        s2, first_word_index
    );
    s2.clear();
    println!(
        "Now the string is '{}' but the index is still {}!",
        s2, first_word_index
    );
    // Using string slices
    let s3 = String::from("hello world");
    let hello = &s3[0..5];
    println!("Hardcoding the position, the first word is {}", hello);
    // Should be &s3[..] to get &str instead of &String, but Rust is smart about this
    println!(
        "Using the function, the first word is {}",
        first_word_with_slices(&s3)
    );
}

fn get_length_verbose(string: String) -> (usize, String) {
    (string.len(), string)
}

fn get_length_referenced(string: &String) -> usize {
    string.len()
}

fn first_word_with_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // We use the byte literal syntax here
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_with_slices(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
