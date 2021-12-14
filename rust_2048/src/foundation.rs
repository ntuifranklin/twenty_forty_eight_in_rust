extern crate num;

fn partn_s<T, I, F>(val: I, prd: F) -> Vec<T>
where
    T: Copy,
    I: IntoIterator<Item = T>,
    for<'r> F: Fn(&'r T) -> bool,
{
    let (mut l, r): (Vec<T>, Vec<T>) = val.into_iter().partition(prd);
    l.extend(r.iter());
    l
}

fn backwrd_merging(val: &mut [i32]) -> i32 {
    if val[0] == val[1] && val[1] != 0 {
        val[0] = 0;
        val[1] += 1;
        num::pow::pow(2, val[1] as usize)
    } else {
        0
    }
}

pub fn right_slider(data: &[i32]) -> (Vec<i32>, i32) {
    let mut ret = partn_s(data.iter().cloned(), |x| *x == 0);
    let mut idx = data.len();
    let mut score = 0;
    while idx > 1 {
        score += backwrd_merging(&mut ret[idx - 2..idx]);
        idx -= 1;
    }
    (partn_s(ret.iter().cloned(), |x| *x == 0), score)
}

pub fn left_slider(num: &[i32]) -> (Vec<i32>, i32) {
    let ret = num.clone().iter().rev().cloned().collect::<Vec<_>>();
    let (num, score) = right_slider(&ret);
    (num.iter().cloned().rev().collect::<Vec<_>>(), score)
}

pub fn transpose(num: &mut [i32; 16]) {
    for i in 0..4 {
        for j in i..4 {
            num.swap(i + 4 * j, j + 4 * i);
        }
    }
}
