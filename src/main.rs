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

	while let Some(Ok(word)) = words.next() {
		println!("{}", word);
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
mod training_tests {
	use super::{Dictionary, train};
    use std::io::{Read, Result};

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

