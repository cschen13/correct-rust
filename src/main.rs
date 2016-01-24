use std::{env, fs};
use std::io::{BufRead, BufReader, Read, stdin};

fn main() {
    //Get arguments
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
    	panic!("Need a training file and an input file");
    }
    let train_file = fs::File::open(&args[1]).expect("Error opening training file");

    let mut dictionary = Dictionary::new();
    train(train_file, &mut dictionary);
    //print_dictionary(dictionary);	//testing purposes mostly

    correct(stdin());
}

type Dictionary = std::collections::BTreeMap<String, usize>;

fn train<R: Read>(train_file: R, mut dictionary: &mut Dictionary) {
    let mut lines = BufReader::new(train_file).lines();

    while let Some(Ok(line)) = lines.next() {
        if let Ok(unclean_line) = line.parse::<String>() {
        	//This time, I'm ignoring apostrophes altogether...
            let training_words: Vec<&str> = unclean_line.splitn(unclean_line.len() + 1, |c: char| !c.is_alphabetic()).collect();

            for word in training_words {
                match word {
                    "" => {
                        continue;
                    }
                    _ => {
                        increment_word(dictionary, String::from(word)
                            .to_lowercase());
                    }
                }
            }
        }
    }
}

fn increment_word(mut map: &mut Dictionary, word: String) {
    *map.entry(word).or_insert(0) += 1;
}


fn correct<R: Read>(input: R) {
	let mut words = BufReader::new(input).lines();
	let alphabet = "abcdefghijklmnopqrstuvwxyz";

	while let Some(Ok(word)) = words.next() {
		let mut splits: Vec<_> = Vec::new();
		let mut deletes: Vec<String> = Vec::new();
		let mut transposes: Vec<String> = Vec::new();
		let mut replaces: Vec<String> = Vec::new();
		let mut inserts: Vec<String> = Vec::new();

		for i in 0..(word.len() + 1) {
			splits.push(word.split_at(i));
		}

		// for x in splits {
		// 	println!("{} {}", x.0, x.1);
		// }

		for split in splits {
			if !(split.1).is_empty() {
				deletes.push(String::from(split.0) + (split.1).split_at(1).1);
			}

			if split.1.len() > 1 {
				let (chars_to_switch, rest) = split.1.split_at(2);
				let switch_iter: Vec<char> = chars_to_switch.chars().collect();
				let mut new_string = String::from(split.0);
				new_string.push(switch_iter[1]);
				new_string.push(switch_iter[0]);
				transposes.push(new_string + rest);
			}

			for letter in alphabet.chars() {
				if !(split.1).is_empty() {
					let mut replace_string = String::from(split.0);
					replace_string.push(letter);
					replaces.push(replace_string + (split.1).split_at(1).1);
				}
				
				let mut insert_string = String::from(split.0);
				insert_string.push(letter);
				inserts.push(insert_string + split.1);
			}
		}

		for edit in deletes {
			println!("{}", edit);
		}

		println!("");

		for edit in transposes {
			println!("{}", edit);
		}

		println!("");

		for edit in replaces {
			println!("{}", edit);
		}

		println!("");

		for edit in inserts {
			println!("{}", edit);
		}

		println!("");
	}
}

// fn print_dictionary(dictionary: Dictionary) {
// 	let mut sorted_vec: Vec<_> = dictionary.iter().collect();
//     sorted_vec.sort_by(|a, b| b.1.cmp(a.1));

//     for word in sorted_vec.iter() {
//         println!("{}: {}", word.0, word.1);
//     }
// }

#[cfg(test)]
mod tests {
	use std::io::{Read, Result};

	mod correct_tests {
		use super::StringReader;
    	use super::super::{Dictionary, train, correct};
	}

    mod training_tests {
    	use super::StringReader;
    	use super::super::{Dictionary, train};
    	
		#[test]
		fn basic_training() {
			let input = StringReader::new("hello world hello word hello world".to_owned());
	        let mut under_test = Dictionary::new();
	        train(input, &mut under_test);

	        let mut expected = Dictionary::new();
	        expected.insert("hello".to_owned(), 3);
	        expected.insert("world".to_owned(), 2);
	        expected.insert("word".to_owned(), 1);

	        assert_eq!(expected, under_test);
		}

		#[test]
		fn ignore_non_alphabetic_training() {
			let input = StringReader::new("hello\n####'''world hello\nword 395 hello world".to_owned());
	        let mut under_test = Dictionary::new();
	        train(input, &mut under_test);

	        let mut expected = Dictionary::new();
	        expected.insert("hello".to_owned(), 3);
	        expected.insert("world".to_owned(), 2);
	        expected.insert("word".to_owned(), 1);

	        assert_eq!(expected, under_test);
		}
	}

	struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }
    
    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }
            return Ok(count);
        }
    }
}

