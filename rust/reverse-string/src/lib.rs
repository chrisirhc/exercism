use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut g = input.graphemes(true).collect::<Vec<&str>>();
    g.reverse();
    let mut s = String::new();
    for i in g.iter() {
        s = s + i;
    }
    s
}
