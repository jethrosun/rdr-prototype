use failure::Fallible;

use headless_chrome::{Browser, LaunchOptionsBuilder};

fn query(input: &str) -> Fallible<()> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://kr.msn.com")?;
    // tab.navigate_to("kr.msn.com")?;
    // tab.navigate_to("https://blogs.medicine.iu.edu")?;

    println!("pass");
    // tab.type_str(&input)?.press_key("Enter")?;
    match tab.wait_for_element("html") {
        Err(e) => println!("Query failed: {:?}", e),
        Ok(e) => match e.get_description()?.find(|n| n.node_name == "#text") {
            Some(n) => println!("Result for `{}`: {}", &input, n.node_value),
            None => eprintln!("No shortdescription-node found on page"),
        },
    }
    println!("pass 2");
    Ok(())
}

fn main() -> Fallible<()> {
    let input = "Elvis Aaron Presley";
    query(input)
}
