use valid_selectors::xpath;

fn main() {
    let all_links: &str = xpath!("//a");
    assert_eq!(all_links, "//a");
}