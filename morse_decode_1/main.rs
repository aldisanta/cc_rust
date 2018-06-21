// Preloaded:
//
// struct MorseDecoder {
//     morse_code: HashMap<String, String>,
// }
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

impl MorseDecoder {

    fn decode_morse(&self, encoded: &str) -> String {
        let words: Vec<String> = encoded.split("   ").to_owned();
        for word in  {
            
        }
        decoded.to_string()
    }

}