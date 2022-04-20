#[cfg(all(feature = "mimalloc"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use profile::memory_usage;
fn main() {
    
    let before = memory_usage().allocated;
    println!("Before {:?}",before.to_string());
    let v: Vec<_> = (1..100000000).collect();
    // println!("{:?}", v);
    println!("After {:?}",(memory_usage().allocated-before).to_string());
}