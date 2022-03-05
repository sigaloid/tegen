use rand::Rng;

/// The `TextGenerator` struct, used for generating text from a template.
pub struct TextGenerator {
    start_c: char,
    end_c: char,
    sep: char,
}

impl TextGenerator {
    /// Creates a new `TextGenerator` struct.
    pub fn new() -> Self {
        Self {
            start_c: '{',
            end_c: '}',
            sep: '|',
        }
    }
    
    /// Generates text from the given template.
    pub fn generate(&self, text: String) -> String {
        self.scan_and_replace(text.chars().collect::<Vec<char>>()).iter().map(|x| (*x).to_string()).collect()
    }

    fn get_random_part(&self, text: Vec<char>) -> Vec<char> {
        let mut open_level = 0;
        let mut last_pos = 0;
        let mut is_ignore = false;

        let mut parts = Vec::<String>::new();

        for i in 0..text.len() {
            if text[i] ==self.start_c {
                open_level += 1;
                is_ignore = true;
                continue;
            }

            if text[i] ==self.end_c {
                open_level -= 1;
                if open_level == 0 {
                    is_ignore = false;
                }
                continue;
            }

            if is_ignore {
                continue;
            }

            if text[i] ==self.sep {
                parts.push(text[last_pos..i].iter().map(|x| (*x).to_string()).collect());
                last_pos = i + 1;
            }
        }

        parts.push(text[last_pos..].iter().map(|x| (*x).to_string()).collect());

        let mut rng = rand::thread_rng();
        parts[rng.gen_range(0..parts.len())].chars().collect()
    }

    fn scan_and_replace(&self, text: Vec<char>) -> Vec<char> {
        let mut start_safe_pos = 0;
        let mut start_pos = 0;
        let mut end_pos = 0;
        let mut open_level = 0;
        let mut is_find = false;
        let mut result = Vec::<char>::new();

        for i in 0..text.len() {
            if text[i] ==self.start_c {
                if open_level == 0 {
                    start_pos = i;
                    result.append(&mut text[start_safe_pos..start_pos].iter().map(|x| (*x).to_string()).collect::<String>().chars().collect::<Vec<char>>());
                }
                
                open_level += 1;
                continue;
            }

            if text[i] ==self.end_c {
                open_level -= 1;
                if open_level == 0 {
                    is_find = true;
                    end_pos = i;

                    start_safe_pos = end_pos + 1;
                    result.append(&mut self.scan_and_replace(self.get_random_part(text[(start_pos+1)..end_pos].iter().map(|x| (*x).to_string()).collect::<String>().chars().collect::<Vec<char>>())));
                }
                continue;
            }
        }

        if !is_find {
            return text;
        }

        result.append(&mut text[(end_pos+1)..].iter().map(|x| (*x).to_string()).collect::<String>().chars().collect::<Vec<char>>());

        result
    }
}