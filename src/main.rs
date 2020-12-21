use std::env;
use std::collections::HashMap;

fn rot13(input:&str)->String{
    let ALPHABET: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ ".chars().collect();
    let ALPHABET_INDEX:HashMap<usize, char> = ALPHABET.clone().into_iter().enumerate().collect();
    let ALPHABET_REVERSE_INDEX:HashMap<char, usize> = ALPHABET.clone().into_iter().enumerate().map(|x| (x.1, x.0)).collect();

    let mut dest_string = String::with_capacity(input.len());
    for (i, c) in input.chars().enumerate() {
        let index = ALPHABET_REVERSE_INDEX.get(&c);
        if index.is_none(){
            dest_string.push(c);
        }
        else{
        let target_index = (index.unwrap() + 13)%ALPHABET.len();
        dest_string.push(ALPHABET_INDEX[&target_index]);

        }

    }
    dest_string
}

fn main() {
    let input:Vec<String> = env::args().collect();
    println!("{}", rot13(&input[1]))

}
