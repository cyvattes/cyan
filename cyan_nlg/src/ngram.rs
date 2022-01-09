pub(crate) fn from(text: &str, n: usize) -> Vec<String> {
    let t: Vec<_> = text
        .split_ascii_whitespace()
        .collect();

    if t.len() < n {
        return vec![];
    }

    let mut ngram = Vec::new();
    for i in 0..=t.len()-n {
        let mut v = Vec::new();
        for j in 0..n {
            v.push(t[i+j]);
        }

        ngram.push(v.join(" "));
    }

    ngram
}
