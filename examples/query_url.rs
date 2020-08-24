use failure::Fallible;

use headless_chrome::{Browser, LaunchOptionsBuilder};

fn query(input: &str) -> Fallible<()> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;

    let http_hostname = "http://".to_string() + &hostname;
    let https_hostname = "https://".to_string() + &hostname;

    tab.navigate_to(&http_hostname)?;
    match tab.wait_for_element("html") {
        Ok(e) => println!("got html"),
        Err(e) => println!("Query failed: {:?}", e),
    }
    println!("hostname: {:?} http done", hostname);

    tab.navigate_to(&https_hostname)?;
    match tab.wait_for_element("html") {
        Ok(e) => println!("got html"),
        Err(e) => println!("Query failed: {:?}", e),
    }
    println!("hostname: {:?} https done", hostname);

    Ok(())
}

fn main() -> Fallible<()> {
    let input = "Elvis Aaron Presley";
    query(input)
}
