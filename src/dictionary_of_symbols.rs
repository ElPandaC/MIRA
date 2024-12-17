use std::collections::HashMap;

struct AllChars {
    all_chars: Vec<char>,
    length: i32,
    weights: Vec<f64>,
    char_map: HashMap<char, f64>,
}

impl AllChars {
    pub fn new() -> Self {
        let all_chars = Self::generate_chars();
        let weights = Self::generate_weights(&all_chars);
        let char_map = Self::generate_char_map(&all_chars, &weights);

        AllChars {
            all_chars: all_chars.clone(),
            length: all_chars.len() as i32,
            weights,
            char_map,
        }
    }

    fn generate_chars() -> Vec<char> {
        let english_lowercase = 'a'..='z';
        let english_uppercase = 'A'..='Z';
        let russian_lowercase = 'а'..='я';
        let russian_uppercase = 'А'..='Я';
        let digits = '0'..='9';
        let special_symbols = '!'..='/';
        let special_symbols_2 = ':'..='@';
        let special_symbols_3 = '['..='`';
        let special_symbols_4 = '{'..='~';

        let mut all_chars: Vec<char> = english_lowercase.chain(english_uppercase)
            .chain(russian_lowercase)
            .chain(russian_uppercase)
            .chain(digits)
            .chain(special_symbols)
            .chain(special_symbols_2)
            .chain(special_symbols_3)
            .chain(special_symbols_4)
            .chain(std::iter::once(' '))
            .collect();

        all_chars.sort();
        all_chars
    }

    fn generate_weights(all_chars: &Vec<char>) -> Vec<f64> {
        all_chars.iter()
            .map(|c| {
                let unicode_value = *c as u32;
                (unicode_value as f64 / 65536.0) * 100.0
            })
            .collect()
    }

    fn generate_char_map(all_chars: &Vec<char>, weights: &Vec<f64>) -> HashMap<char, f64> {
        let mut char_map: HashMap<char, f64> = HashMap::new();
        for (i, &c) in all_chars.iter().enumerate() {
            char_map.insert(c, weights[i]);
        }
        char_map
    }

    pub fn get_char_weight(&mut self, c: char) -> &f64 {
        if !self.char_map.contains_key(&c) {
            self.add_char(c);
        }
        self.char_map.get(&c).unwrap()
    }

    fn add_char(&mut self, c: char) {
        if !self.all_chars.contains(&c) {
            self.length += 1;
            self.all_chars.push(c);
            let weight = (c as u32 as f64 / 65536.0) * 100.0;
            self.weights.push(weight);
            self.char_map.insert(c, weight);
        }
    }
}


