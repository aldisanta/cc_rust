fn accum(s:&str)->String {
    let mut i: u8 = 0;
    let mut mumble: String = "".to_string();
    for ss in s.chars() {
        if i > 0 {
            mumble.push_str("-");
            mumble.push_str(ss.to_uppercase().to_string().as_str());
            mumble.push_str(ss.to_string().as_str().repeat(i as usize).to_lowercase().as_str());
        } else {
            mumble.push_str(ss.to_uppercase().to_string().as_str());
        }
        i += 1;
    }
    mumble
}

fn accum_best(s:&str)->String {
    s.chars().enumerate()
    .map(|(i,c)| c.to_string().to_uppercase() + 
          &(0..i).map(|_| c.to_string().to_lowercase()).collect::<String>())
    .collect::<Vec<_>>().join("-")
}

fn main() {
    accum("test");
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_tests() {
  assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
  assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
  assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
  assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
  assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}