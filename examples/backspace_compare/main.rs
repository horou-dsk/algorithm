fn main() {
    let (s, t) = ("a#c".to_string(), "b".to_string());
    println!("{}", backspace_compare(s, t));
}

// fn backspace_compare(s: String, t: String) -> bool {
//     let f = |p: String| {
//         let mut v = Vec::new();
//         p.chars().for_each(|char| {
//             if char != '#' {
//                 v.push(char);
//             } else {
//                 v.pop();
//             }
//         });
//         v
//     };
//     f(s).eq(&f(t))
// }

fn backspace_compare(s: String, t: String) -> bool {
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let (mut i, mut j) = (s.len(), t.len());
    let sign = Some(&b'#');
    loop {
        i -= 1;
        j -= 1;
        let mut k = 0;
        while s.get(i) == sign || k > 0 {
            k += if s.get(i) == sign {1} else {-1};
            i -= 1;
        }
        k = 0;
        while t.get(j) == sign || k > 0 {
            k += if t.get(j) == sign {1} else {-1};
            j -= 1;
        }
        match (s.get(i), t.get(j)) {
            (Some(p), Some(q)) if p == q => (),
            (None, None) => break true,
            _ => break false
        }
    }
}
