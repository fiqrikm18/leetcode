struct LenOfLastWordSolution {}

impl LenOfLastWordSolution {
    pub fn length_of_last_word(s: String) -> i32 {
        let splited_str = s
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let last_word_len: i32 = splited_str[splited_str.len() - 1].len().try_into().unwrap();

        last_word_len
    }
}

pub fn solve() {
    let last_word_ln = LenOfLastWordSolution::length_of_last_word(String::from("Hello      World"));
    println!("{last_word_ln}");
}
