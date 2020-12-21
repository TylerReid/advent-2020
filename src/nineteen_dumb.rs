use std::fs;
use regex::Regex;

pub fn day_nineteen() {
    let parts = DUMB.split("\n\n").collect::<Vec<&str>>();
    
    let parser = CYK::new(G{}, WB{});

    let mut num_follow = 0;
    for l in parts[1].lines() {
        println!("{} {:?}", l, parser.parse(l));
        if let Some(_) = parser.parse(l).get_final() {
            num_follow += 1;
        }
    }

    println!("{}", num_follow);
}

struct G {}

impl<'g> Grammar<'g> for G {
    fn convert(&self) -> Vec<GrammarRule<'g>> {
        let parts = DUMB.split("\n\n").collect::<Vec<&str>>();

        let mut v = Vec::new();
        for r in parts[0].lines() {
            if let Some(x) = TERMINAL_RULE.captures(r) {
                v.push(GrammarRule{
                    left_symbol: x.get(1).unwrap().as_str(),
                    right_symbol: x.get(2).unwrap().as_str(),
                })
            } else if let Some(x) = RULE.captures(r) {
                v.push(GrammarRule{
                    left_symbol: x.get(1).unwrap().as_str(),
                    right_symbol: x.get(2).unwrap().as_str(),
                })
            } else {
                panic!("unexpected rule {}", r);
            }
        }
        println!("{:?}", v);
        v
    }
}

struct WB {}
impl WordBank for WB {
    fn lookup(&self, word: &str) -> &str {
        match word {
            "a" => "a",
            "b" => "b",
            _ => "dne",
        }
    }
}

lazy_static! {
    static ref TERMINAL_RULE: Regex = Regex::new(r#"^(\d*): "(\w)"$"#).unwrap();
    static ref RULE: Regex = Regex::new(r#"^(\d*): (.*)$"#).unwrap();
}

lazy_static! {
    static ref DUMB: &'static str = 
r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

a b a b b b
b a b a b a
a b b b a b
a a a b b b
a a a a b b b"#;
}

// tried to import the crate below, but the GrammarRule has private fields, so its useless
// so F it, just copy
// rusty_grammar = "0.1.2"

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;
use itertools::{iproduct, join};
use linked_hash_map::LinkedHashMap;

#[derive(Clone, Debug)]
/// The Struct to define a Grammar Rule.
pub struct GrammarRule<'symbol> {
    /// The left side of a grammar rule.
    pub left_symbol: &'symbol str,
    /// The right side of a grammar rule.
    pub right_symbol: &'symbol str,
}

/// The trait for a grammar. For a struct to be a grammar it must implement these methods.
pub trait Grammar<'grammar> {
    /// The convert function takes whatever struct you define and generates a Vector of Grammar Rules.
    fn convert(&self) -> Vec<GrammarRule<'grammar>>;
}

/// The trait for a wordbank. For a struct to be a wordbank it must implment these methods.
pub trait WordBank {
    /// The lookup function takes whatever struct you have given a word and looks up the type of word it is. i.e. noun.
    fn lookup(&self, word: &str) -> &str;
}

/// The struct for the CYK algorithm.
pub struct CYK<'rules, W> {
    /// Grammar Rules: the list of grammar rules in CNF form.
    grammar_rules: Vec<GrammarRule<'rules>>,
    /// The struct that implments the WordBank Trait.
    word_bank: W,
}

#[derive(Clone, Debug, Hash, PartialEq)]
/// A struct to represent the poisiton of the CYK rule lookup in a hashmap. i.e. the key of a hashmap.
struct MatrixIndicator {
    x: usize,
    y: usize,
}

impl fmt::Display for MatrixIndicator {
    /// A print method for the MatrixIndicator method.
    ///
    /// # Arguments
    ///
    /// * `f` - A fmt Formatter to be passed to the write macro.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "({}, {})", self.x, self.y)
    }
}

// To implment Equality for the MatrixIndicator struct.
impl Eq for MatrixIndicator {}

#[derive(Clone, Debug)]
/// A struct to store the result of the CYK algorithm.
pub struct MatrixResult {
    /// A hashmap to store all the rule conversions done by the algorithm.
    map: HashMap<MatrixIndicator, String>,
    /// The final result of what the given sentence was.
    final_res: Option<String>,
    /// The number of words in the given sentence.
    num_words: usize,
}

impl MatrixResult {
    /// A function to create an instance of the MatrixResult struct.
    fn new() -> Self {
	Self {
	    map: HashMap::new(),
	    final_res: None,
	    num_words: 0
	}
    }

    /// A function to get the final result of the CYK algorithm.
    pub fn get_final(&self) -> Option<String> {
	self.final_res.clone()
    }

    /// A function to set the final result of the CYK algorithm.
    ///
    /// # Arguments
    ///
    /// * `final_res` - The string to set for the final result of CYK algo.
    fn set_final(&mut self, final_res: String) {
	self.final_res = Some(final_res);
    }

    /// A function to insert a result into a position in the map of the result.
    fn insert(&mut self, mi: MatrixIndicator, res: String) {
	self.map.insert(mi, res);
    }

    /// A function to set the number of words in the Matrix Result. Refers to the number of words in the sentence given.
    fn set_num_words(&mut self, size: usize) {
	self.num_words = size;
    }

    /// A function to get the number of words in the Matrix Result. Refers to the number of words in the sentence given.
    pub fn get_num_words(&self) -> usize {
	self.num_words
    }
}

impl fmt::Display for MatrixResult {
    /// Function to display the MatrixResult. Shows the table printed in line by line format.
    ///
    /// # Arguments
    ///
    /// * `f` - A fmt Formatter to be passed to the write macro.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	if self.num_words == 0 {
	    return write!(f, "No result caluclated.");
	}

	let mut output = Vec::new();
	output.push("LN# 1:".to_owned());

	for position in 0..self.num_words {
	    output.push(format!("\t({}, {}):", position+1, position+1));
	    let entry = self.map.get(&MatrixIndicator{ x: position, y: position }).expect("Can not be empty");
	    output.push(format!("{}", entry));
	}
	output.push("\n".to_owned());

	let mut line_num = 2;
	for ln in 1..self.num_words {
	    output.push(format!("LN# {}:", line_num));

	    for x in 0..self.num_words-ln {
		let entry = self.map.get(&MatrixIndicator{ x, y: x+ln }).expect("Can not be empty");
		if entry == "" {
		    continue;
		}
		output.push(format!("\t({}, {}):", x, x+ln));
		output.push(format!("{}", entry));
	    }
	    output.push("\n".to_owned());
	    line_num += 1;
	}

	write!(f, "{}", join(&output, ""))
    }
}


lazy_static::lazy_static! {
    /// Used for the memoized version of parse. Memoizes 100 sentences.
    static ref MEMO: Mutex<LinkedHashMap<&'static str, MatrixResult>> = Mutex::new(LinkedHashMap::with_capacity(101));
}

/// A function to join two strings into one.
///
/// # Arguments
///
/// * `str1` - The frist string to be split and combined.
/// * `str2` - The second string to be split and combined.
fn vec_production(str1: &str, str2: &str) -> Vec<String> {
    iproduct!(
	str1.split(" ").collect::<Vec<&str>>(),
	str2.split(" ").collect::<Vec<&str>>())
	.map(|vals| {
	    join(&[vals.0, vals.1], " ")
	})
	.collect::<Vec<String>>()
}

impl<'grammar, W> CYK<'grammar, W> where
    W: WordBank {
    /// Creates a new instance of a CYK algo with a set of rules and a word brank.
    ///
    /// # Arguments
    ///
    /// * `rules` - The object that implements the Grammar Trait.
    /// * `word_bank` - The object that implements the WordBank Trait.
    pub fn new<G>(rules: G, word_bank: W) -> Self where
	G: Grammar<'grammar> {
	Self {
	    grammar_rules: rules.convert(),
	    word_bank
	}
    }

    /// Finding the terminal left side rule for a grammar given a terminal right side rule.
    fn find_terminal_assign(&self, terminal: &str) -> String {
	let mut res = Vec::new();

	for grammar in &self.grammar_rules {
	    for rule in grammar.right_symbol.split(" | ").collect::<Vec<&str>>() {
		if rule == terminal {
		    res.push(grammar.left_symbol.clone());
		}
	    }
	}
	
	join(res, " ")
    }

    /// A function that parses the given sentence for the rules and wordbank in the CYK struct.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to be parsed and validated.
    pub fn parse<'word>(&self, input: &'word str) -> MatrixResult {
	let mut result: MatrixResult = MatrixResult::new();
	
	let words = input.split_whitespace().collect::<Vec<&str>>();
	let num_words = words.len();
	result.set_num_words(num_words);
	
	for (pos, word) in words.iter().enumerate() {
	    let terminal = self.word_bank.lookup(word);
	    result.insert(MatrixIndicator{ x: pos, y: pos }, self.find_terminal_assign(terminal));
	}

	for l in 1..=num_words {
	    for i in 0..(num_words - l) {
		let j = i + l;

		let mut targets: Vec<String> = Vec::new();
		for k in 1..=j {
		    let empty = String::from("");
		    let fv = result.map.get(&MatrixIndicator{ x: i, y: i+k-1 }).unwrap_or(&empty);
		    let sv = result.map.get(&MatrixIndicator{ x: i+k, y: j }).unwrap_or(&empty);
		    let mut products = vec_production(fv, sv);
		    targets.append(&mut products);
		}

		let mut res = String::from("");
		for target in targets {
		    let target_symbol = self.find_terminal_assign(target.as_str());
		    
		    if !res.contains(&target_symbol) {
			res = match res.as_str() {
			    "" => target_symbol,
			    _ => join(&[res, target_symbol], " ")
			};
		    }
		}

		result.insert(MatrixIndicator{ x: i, y: j }, res);
	    }
	}
	
	let final_result = result.map.get(&MatrixIndicator{ x: 0, y: num_words - 1 }).expect("Can not be empty").to_owned();
	result.set_final(final_result);
	
	return result;
    }

    /// A memoized version of the parse function. i.e. if sentence exists in map just instant return results.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to be parsed and validated.
    pub fn memoized_parse<'word>(&self, input: &'static str) -> MatrixResult {
	if MEMO.lock().expect("Memo should not be NONE.").contains_key(input) {
	    return MEMO.lock().expect("Memo should not be NONE.").get(input).expect("Should never be none.").clone();
	}

	let res = self.parse(input);
	MEMO.lock().expect("Memo should not be NONE.").insert(input, res.clone());
	if MEMO.lock().expect("Memo should not be NONE.").len() > 100 {
	    MEMO.lock().expect("Memo should not be NONE.").pop_back();
	}

	res
    }


}