use std::ops::{Add, Mul};

fn  findstem(mut arr:Vec<String>) -> Vec<String>
{
    // Determine size of the array
    arr.sort_by(|l, r| { // l.size() < r.size()
       let l_len = l.len();
       let r_len = r.len();
       l_len.cmp(&r_len)
    });
    
    let n = arr.len();
    // Take first word from array as reference

    let s = &arr[0];

    let len = s.len();
 
    let mut res : Vec<String> =  vec![];

    for i in 0..len {
        
        for j in 0..len-i+1 {
println!("{} {}", i, j);
            // generating all possible substrings
            // of our reference string arr[0] i.e s
           let stem = &s[j..i];

            let mut k : usize;
            k = 1;
            
            for k in 1..n {
                // Check if the generated stem is
                // common to all words
                match arr[k].find(&stem) {
                     // not found
                    None => break,
                    Some(_) => continue
                }
            }
           // cout << "looking for " << stem << " and found in " << k << " end " << n << endl;
            // If current substring is present in all strings
            if k == n {
                res.push(stem.to_string());
            }
                 
        }
        
        if res.len() > 0 {
            break
        }
    }
 
    return res;
}

fn dot<N>(v1: &[N], v2: &[N]) -> N
where N: Add<Output=N> + Mul<Output=N> + Default + Copy
{
let mut total = N::default();
for i in 0 .. v1.len() {
total = total + v1[i] * v2[i];
}
total
}

fn test_dot() {
assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
fn main() {
    test_dot();
    let arr = vec![ "grace".to_string(), "graceful".to_string(), "disgraceful".to_string(),
                       "gracefully".to_string() ];
    let res = findstem(arr);
    println!("{:?}", res);
    println!("Done.")
}
