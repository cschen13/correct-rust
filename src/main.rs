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
    correct(stdin(), &dictionary);
}

type Dictionary = std::collections::BTreeMap<String, usize>;

/// Counts the frequency of each word in a file and adds each word/frequency
/// pair into a HashMap.
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

/// Given a word (key), increments the frequency (value) by one, or
/// inserts it into the 'Dictionary' if it doesn't exist.
fn increment_word(mut map: &mut Dictionary, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

/// Prints each word in the input along with a suggested correction
/// OR a "-" if no correction exists within two small edits.
///
/// Note: the correction logic places precedence on single-edit
/// corrections over double-edit corrections.
fn correct<R: Read>(input: R, dictionary: &Dictionary) {
	let mut words = BufReader::new(input).lines();

	while let Some(Ok(word)) = words.next() {
		let mut max_frequency: usize = 0;
		if dictionary.contains_key(&*word.to_lowercase()) {
			println!("{}", word);
			continue;
		}

		let mut splits: Vec<_> = Vec::new();
		let mut first_edits: Vec<String> = Vec::new();
		let original_word = word.clone();

		for i in 0..(word.len() + 1) {
			splits.push(word.split_at(i));
		}

		make_edits(&mut splits, &mut first_edits);

		match possible_edit(&mut max_frequency, &mut first_edits, &dictionary) {
			Some(edit) => {
				println!("{}, {}", original_word, edit);
				continue;
			} None => {

			}
		}

		let mut second_splits: Vec<_> = Vec::new();
		let mut second_edits: Vec<String> = Vec::new();

		for word_edit in &first_edits {
			for i in 0..(word_edit.len() + 1) {
				second_splits.push(word_edit.split_at(i));
			}
		}

		make_edits(&mut second_splits, &mut second_edits);
		match possible_edit(&mut max_frequency, &mut second_edits, &dictionary) {
			Some(edit) => {
				println!("{}, {}", original_word, edit);
			} None => {
				println!("{}, {}", original_word, "-");
			}
		}
	}	
}

/// Computes and stores all possible single small edits of a word into the 
/// "edits" 'Vec<String>'.
fn make_edits(splits: &mut Vec<(&str, &str)>, edits: &mut Vec<String>) {
	let alphabet = "abcdefghijklmnopqrstuvwxyz";

	for split in splits {
		//Deletes
		if !(split.1).is_empty() {
			edits.push(String::from(split.0) + (split.1).split_at(1).1);
		}

		//Transposes
		if split.1.len() > 1 {
			let (chars_to_switch, rest) = split.1.split_at(2);
			let switch_iter: Vec<char> = chars_to_switch.chars().collect();
			let mut new_string = String::from(split.0);
			new_string.push(switch_iter[1]);
			new_string.push(switch_iter[0]);
			edits.push(new_string + rest);
		}

		//Replaces and Inserts
		for letter in alphabet.chars() {
			if !(split.1).is_empty() {
				let mut replace_string = String::from(split.0);
				replace_string.push(letter);
				edits.push(replace_string + (split.1).split_at(1).1);
			}
			
			let mut insert_string = String::from(split.0);
			insert_string.push(letter);
			edits.push(insert_string + split.1);
		}
	}
}

/// Calculates the most likely correction of a word from
/// a 'Vec<String>' of its edited versions, given a 'Dictionary'.
fn possible_edit<'a, 'b>(max_frequency: &'a mut usize, edits: &'a mut Vec<String>, dictionary: &'b Dictionary) -> Option<&'a str> {
	let mut edit = "-";
	for word_edit in edits {
		if dictionary.contains_key(&*word_edit.to_lowercase()) {
			let frequency = *dictionary.get(&*word_edit.to_lowercase()).unwrap();
			if frequency > *max_frequency {
				edit = &*word_edit;
				*max_frequency = frequency;
		    }
		}
	}

	if edit != "-" {
		Some(edit)
	} else {
		None
	}
}

///Prints the entries in the 'Dictionary'. This function was used 
/// mainly for testing purposes.
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
		//I don't know how to test the correct function using unit tests,
		//since I can only tell if it worked from the output (my function
		//doesn't return anything). 

		//A workaround that I've used is to only run
		//the tests in this module one at a time with -- --nocapture
		//on to see the output and confirm that it is correct.
		use super::StringReader;
    	use super::super::{Dictionary, train, correct};

    	#[test]
    	fn deletes_test() {
    		let training_file = StringReader::new("hello world hello word hello world".to_owned());
	        let mut dictionary = Dictionary::new();
	        train(training_file, &mut dictionary);

	        let input_file = StringReader::new("Hellob\nkhello\nwjorld\nwioprld".to_owned());
	        correct(input_file, &dictionary);
	        //use --nocapture and verify that corrections return:
	        //Hello
	        //hello
	        //world
	        //world
    	}

    	
    	fn replaces_test() {
    		let training_file = StringReader::new("hello world hello word hello world".to_owned());
	        let mut dictionary = Dictionary::new();
	        train(training_file, &mut dictionary);

	        let input_file = StringReader::new("Hellb\nyello\ndorw\nrorld".to_owned());
	        correct(input_file, &dictionary);
	        //use --nocapture and verify that corrections return:
	        //Hello
	        //hello
	        //word
	        //world
	    }

	    
	    fn transposes_test() {
    		let training_file = StringReader::new("hello world hello word hello world".to_owned());
	        let mut dictionary = Dictionary::new();
	        train(training_file, &mut dictionary);

	        let input_file = StringReader::new("eHllo\nehlol\nowdr\nowrdl".to_owned());
	        correct(input_file, &dictionary);
	        //use --nocapture and verify that corrections return:
	        //Hello
	        //hello
	        //word
	        //world
	    }

	    
	    fn inserts_test() {
    		let training_file = StringReader::new("hello world hello word hello world".to_owned());
	        let mut dictionary = Dictionary::new();
	        train(training_file, &mut dictionary);

	        let input_file = StringReader::new("Helo\nheo\nwd\nwld".to_owned());
	        correct(input_file, &dictionary);
	        //use --nocapture and verify that corrections return:
	        //Hello
	        //hello
	        //word
	        //world
	    }

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

