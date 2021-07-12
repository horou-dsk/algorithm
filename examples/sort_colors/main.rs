fn main() {
    let mut v = vec![2, 1, 2];
    sort_colors(&mut v);
    println!("{:?}", v);
}

fn sort_colors(nums: &mut Vec<i32>) {
    // let n = nums.len();
    // if n < 2 {
    //     return;
    // }
    // let (mut p, mut p2) = (0, n - 1);
    // let mut index = 0i32;
    // while index <= p2 as i32 {
    //     let i = index as usize;
    //     if nums[i] == 0 {
    //         nums.swap(i, p);
    //         p += 1;
    //     }
    //     if nums[i] == 2 {
    //         nums.swap(i, p2);
    //         p2 -= 1;
    //         if nums[i] != 1 {
    //             index -= 1;
    //         }
    //     }
    //     index += 1;
    // }
    let mut tmp = [0; 3];
    nums.iter().for_each(|&x| tmp[x as usize] += 1);
    tmp[1] += tmp[0];
    tmp[2] += tmp[1];
    nums[0..tmp[0] as usize].iter_mut().for_each(|x| *x = 0);
    nums[tmp[0] as usize..tmp[1] as usize].iter_mut().for_each(|x| *x = 1);
    nums[tmp[1] as usize..tmp[2] as usize].iter_mut().for_each(|x| *x = 2)
}
