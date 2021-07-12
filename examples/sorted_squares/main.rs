use std::time::Instant;

fn main() {
    let v = vec![-4,-1,0,3,10];
    let instant = Instant::now();
    println!("{:?}", sorted_squares(v));
    println!("{:?}", instant.elapsed())
}

fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut n = a.len();
    let (mut l, mut r) = (0, n - 1);
    let mut v = vec![0; n];
    while n > 0 {
        let (ln, rn) = (a[l].pow(2), a[r].pow(2));
        n -= 1;
        v[n] = if ln < rn {
            r -= 1;
            rn
        } else {
            l += 1;
            ln
        };
    }
    v
}

/*fn sorted_squares(mut a: Vec<i32>) -> Vec<i32> {
    a.iter_mut().for_each(|v| *v = v.pow(2));
    a.sort();
    a
}*/
