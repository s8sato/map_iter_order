use dashmap::DashMap;
use std::sync::Arc;

fn main() {
    let map = Arc::new(DashMap::new());
    for i in 0..9 {
        map.insert(i, "value");
    }
    map.iter().skip(1).take(5).for_each(|entry| {
        println!("{}", entry.key());
    })
}
