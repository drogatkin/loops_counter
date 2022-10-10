fn  findstem(mut arr:Vec<String>) -> Vec<String>
{   // other shorter impl: https://gist.github.com/Coutlaw/cac31e5639236f99282813c32e6ed43d

    let mut res : Vec<String> =  vec![];
    let n = arr.len();
    
    if n == 0 { return res }
    else if n == 1 {res.push(arr[0].to_string()); return res };
   
    // Determine size of the array
    arr.sort_by(|l, r| { // l.size() < r.size()
       let l_len = l.len();
       let r_len = r.len();
       l_len.cmp(&r_len)
    });
    
   
    // Take first word from array as reference

    let s = &arr[0];

    let len = s.len();
    //println!("shortest str {}", s);
 
    

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
                    break
                }
                k+=1
            }
           // cout << "looking for " << stem << " and found in " << k << " end " << n << endl;
            // If current substring is present in all strings
            //println!("-->{} == {}", k, n);
            if k == n {
                res.push(stem.to_string())
            }
            
            j += 1    
        }
        
        if res.len() > 0 {
            break
        }
        i-=1
    }
 
    res
}

fn main() {
    let arr = vec![ "graceful".to_string(), "disgraceful".to_string(),
                       "gracefully".to_string() , "grace".to_string(), "lllasrgraca".to_string()];
    let res = findstem(arr);
   // println!("{:?}", res);
    assert_eq!(res, vec!["grac".to_string()]);
    let arr2 = vec!["kokobokafghtoka".to_string(), "gggtokavvboka".to_string(), "termtokagopabokah".to_string()];
    assert_eq!(findstem(arr2), vec!["toka".to_string(), "boka".to_string()]);
    let arr3 = vec!["only one".to_string()];
    assert_eq!(vec![arr3[0].to_string()], findstem(arr3));
    println!("Done.")
}