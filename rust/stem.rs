
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
    //println!("shortest str {}", s);
 
    let mut res : Vec<String> =  vec![];

    let mut i = len ;
    while i > 0 {
        let mut j = 0;
        while j < len-i+1  {

            // generating all possible substrings
            // of our reference string arr[0] i.e s
           let stem = &s[j..j+i];

            let mut k = 1;
          //  println!("{} {} = {}", i, j, stem);
            while k < n {
              //  println!("sub={}  in {} ", stem, arr[k]);
                // Check if the generated stem is
                // common to all words
                if ! arr[k].contains(&stem) {
                   // println!("-->{} not found in {}", stem, arr[k]);
                    break;
                }
                k+=1;
            }
           // cout << "looking for " << stem << " and found in " << k << " end " << n << endl;
            // If current substring is present in all strings
            //println!("-->{} == {}", k, n);
            if k == n {
                res.push(stem.to_string());
            }
            
            j += 1;     
        }
        
        if res.len() > 0 {
            break
        }
        i-=1;
    }
 
    return res;
}

fn main() {
    let arr = vec![ "graceful".to_string(), "disgraceful".to_string(),
                       "gracefully".to_string() , "grace".to_string(), "lllasrgraca".to_string()];
    let res = findstem(arr);
    println!("{:?}", res);
    assert_eq!(res, vec!["grac".to_string()]);
    println!("Done.")
}