use std::collections::BTreeMap;

fn order(sentence: &str) -> String {
    let mut ordered_string: String = "".to_string();
    if !sentence.to_string().is_empty() {
        let num = String::from("123456789");
        let byte_num = &num.as_bytes();
        let mut orders = BTreeMap::new();
        let input_words: Vec<_> = sentence.split_whitespace().collect();
        for word in input_words  {
            for byte in word.as_bytes() {
                if byte_num.contains(byte) {
                    orders.insert(byte, word);
                }
            }
        }

        let mut i: u8 = 0;
        for value in orders.values() {
            if i > 0 {
                ordered_string.push_str(" ");
            }
            i += 1;
            ordered_string.push_str(value);
        }
    }
    ordered_string
}