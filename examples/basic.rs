use profile::memory_usage;
fn main() {
    
    let before = memory_usage().allocated;
    println!("Before {:?}",before.to_string());
    let result = 2 + 2;
    println!("After {:?}",memory_usage().allocated.to_string());
    assert_eq!(result, 4);
}