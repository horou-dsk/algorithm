use std::time::{Instant, Duration};
use std::thread;

fn main() {
    let v = vec!["bella".to_string(),"label".to_string(),"roller".to_string()];
    // thread::sleep(Duration::from_millis(3000));
    // let instant = Instant::now();
    // println!("{:?}", common_a_chars(v.clone()));
    // println!("{:?}", instant.elapsed());
    let instant = Instant::now();
    println!("{:?}", common_chars(v.clone()));
    println!("{:?}", instant.elapsed());
}

const ASCII_A: u8 = 'a' as u8;

fn common_a_chars(a: Vec<String>) -> Vec<String> {
    let mut freqs = vec![];
    for word in a {
        let mut freq = [0;26];
        for letter in word.chars() {
            let index = (letter as u8 - ASCII_A) as usize;
            freq[index] += 1;
        }
        freqs.push(freq.clone());
    }
    let mut result = vec![];
    for i in 0..26 {
        let min = freqs.iter().map(|&f| f[i]).min().unwrap();
        for _ in 0..min {
            let i = i as u8;
            let letter = ((i + ASCII_A) as char).to_string();
            result.push(letter.clone());
        }
    }
    result
}

fn common_chars(a: Vec<String>) -> Vec<String> {
    let mut ans=[std::i32::MAX;(b'z'+1) as usize];
    {a.into_iter().for_each(|x|{
        let mut tmp=[0;(b'z'+1) as usize];
        {
            x.bytes().for_each(|x|tmp[x as usize]+=1);
            drop(x);
        }
        for i in b'a' as usize..=b'z' as usize{
            ans[i]=ans[i].min(tmp[i])
        }}
    )}
    let mut ret=Vec::with_capacity(10);
    for i in b'a' as usize..=b'z' as usize{
        for _ in 0..ans[i]{ret.push((i as u8 as char).to_string())}
    }
    ret
}
