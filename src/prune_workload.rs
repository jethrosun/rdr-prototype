use std::collections::HashSet;

pub fn curate_broken_records() -> HashSet<&'static str> {
    let mut broken_urls = HashSet::new();
    broken_urls.insert("arienh4.net.nyud.net");

    return broken_urls;
}
