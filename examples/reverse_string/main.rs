use std::time::Instant;

fn main() {
    let instant = Instant::now();
    // let mut v = vec![];
    let mut v = vec!['s', '1', 'A', 'B', 'C', 'G', 'e', '3'];
    reverse_string(&mut v);
    println!("{:?}", v);
    println!("{:?}", instant.elapsed());
}

fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    if len > 0 {
        let (mut l, mut r) = (0, len - 1);
        while l < r {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}
