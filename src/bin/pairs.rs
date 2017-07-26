
extern crate allpairs;

fn main() {
    let os = vec!["windows","linux","mac"];
    let browser = vec!["firefox","ie","chrome","opera"];
    let country = vec!["canada","usa","mexico"];
    let gender = vec!["male","female"];

    let vectors = vec![gender, country, os, browser];
    
    let result = allpairs::all_pairs(vectors);

    println!("Produced {} pairwise combinations (instead of {}):", result.all_pairs.len(), result.combination);
    for e in result.all_pairs {
        println!("{:?}", e);
    }
}