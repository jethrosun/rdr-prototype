//! Utils functions for the PVN RDR NF.
use crate::unresolvable::curate_unresolvable_records;
use failure::Fallible;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use serde_json::{from_reader, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::time::{Duration, Instant};
use std::vec::Vec;

/// Construct the workload from the session file.
///
/// https://kbknapp.github.io/doapi-rs/docs/serde/json/index.html
pub fn rdr_load_workload(
    file_path: String,
    num_of_secs: usize,
    rdr_users: Vec<i64>,
) -> serde_json::Result<HashMap<usize, Vec<(u64, String, i64)>>> {
    // time in second, workload in that second
    let mut workload = HashMap::<usize, Vec<(u64, String, i64)>>::with_capacity(rdr_users.len());

    let file = File::open(file_path).expect("file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");

    for sec in 0..num_of_secs {
        let mut millis: Vec<(u64, String, i64)> = Vec::new();

        let urls_now = match json_data.get(sec.to_string()) {
            Some(val) => val,
            None => continue,
        };
        for user in &rdr_users {
            let urls = match urls_now.get(user.to_string()) {
                Some(val) => val.as_array(),
                None => continue,
            };

            let broken_urls = curate_unresolvable_records();

            if broken_urls.contains(urls.unwrap()[1].as_str().unwrap()) {
                continue;
            } else {
                millis.push((
                    urls.unwrap()[0].as_u64().unwrap(),
                    urls.unwrap()[1].as_str().unwrap().to_string(),
                    *user as i64,
                ));
            }
        }
        millis.sort();

        workload.insert(sec, millis);
    }
    Ok(workload)
}

/// Retrieve the number of users based on our setup configuration.
pub fn rdr_retrieve_users(rdr_setup: usize) -> Option<usize> {
    let mut map = HashMap::new();
    // map.insert(1, 2);
    // map.insert(2, 4);
    // map.insert(3, 8);
    // map.insert(4, 12);
    // map.insert(5, 16);
    // map.insert(6, 20);

    // map.insert(1, 1);
    // map.insert(2, 2);
    // map.insert(3, 4);
    // map.insert(4, 6);
    // map.insert(5, 8);
    // map.insert(6, 10);

    map.insert(1, 5);
    map.insert(2, 10);
    map.insert(3, 20);
    map.insert(4, 40);
    map.insert(5, 80);
    map.insert(6, 100);

    map.remove(&rdr_setup)
}

/// Read the pregenerated randomness seed from file.
pub fn rdr_read_rand_seed(num_of_users: usize, iter: usize) -> Result<Vec<i64>> {
    let rand_seed_file = "/home/jethros/dev/pvn/utils/rand_number/rand.json";
    let mut rand_vec = Vec::new();
    let file = File::open(rand_seed_file).expect("rand seed file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");

    match json_data.get("rdr") {
        Some(rdr_data) => match rdr_data.get(&num_of_users.clone().to_string()) {
            Some(setup_data) => match setup_data.get(iter.to_string()) {
                Some(data) => {
                    for x in data.as_array().unwrap() {
                        rand_vec.push(x.as_i64().unwrap());
                        // println!("RDR user: {:?}", x.as_i64().unwrap());
                    }
                }
                None => println!(
                    "No rand data for iter {:?} for users {:?}",
                    iter, num_of_users
                ),
            },
            None => println!("No rand data for users {:?}", num_of_users),
        },
        None => println!("No rdr data in the rand seed file"),
    }
    println!(
        "Fetch rand seed for num_of_users: {:?}, iter: {:?}.\nrdr users: {:?}",
        num_of_users, iter, rand_vec
    );
    Ok(rand_vec)
}

/// Create the browser for RDR proxy (user browsing).
///
/// FIXME: Instead of using the particular forked branch we want to eventually use the official
/// headless chrome create but set those parameters correctly here.
pub fn browser_create() -> Fallible<Browser> {
    // /usr/bin/chromedriver
    // /usr/bin/chromium-browser

    let timeout = Duration::new(1000, 0);

    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .idle_browser_timeout(timeout)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    // let tab = browser.wait_for_initial_tab()?;
    // tab.set_default_timeout(std::time::Duration::from_secs(100));

    // println!("Browser created",);
    Ok(browser)
}

/// Simple user browse.
pub fn simple_user_browse(
    current_browser: &Browser,
    hostname: &str,
    _user: &i64,
) -> Fallible<(usize, u128)> {
    let now = Instant::now();
    let tabs = current_browser.get_tabs().lock().unwrap();
    let current_tab = tabs.iter().next().unwrap();
    let http_hostname = "http://".to_string() + &hostname;

    current_tab.navigate_to(&http_hostname)?;

    Ok((1, now.elapsed().as_millis()))
}

/// RDR proxy browsing scheduler.
#[allow(non_snake_case)]
#[allow(unreachable_patterns)]
pub fn rdr_scheduler_ng(
    _pivot: &usize,
    rdr_users: &[i64],
    current_work: Vec<(u64, String, i64)>,
    browser_list: &HashMap<i64, Browser>,
) -> Option<(usize, usize, usize, usize, usize, usize)> {
    let mut num_of_ok = 0;
    let mut num_of_err = 0;
    let mut num_of_timeout = 0;
    let mut num_of_closed = 0;
    let mut num_of_visit = 0;
    let mut elapsed_time = Vec::new();

    for (milli, url, user) in current_work.into_iter() {
        println!("User {:?}: milli: {:?} url: {:?}", user, milli, url);

        if rdr_users.contains(&user) {
            match simple_user_browse(&browser_list[&user], &url, &user) {
                Ok((val, t)) => match val {
                    // ok
                    1 => {
                        num_of_ok += 1;
                        num_of_visit += 1;
                        elapsed_time.push(t as usize);
                    }
                    // err
                    2 => {
                        num_of_err += 1;
                        num_of_visit += 1;
                        elapsed_time.push(t as usize);
                    }
                    // timeout
                    3 => {
                        num_of_timeout += 1;
                        num_of_visit += 1;
                        elapsed_time.push(t as usize);
                    }
                    _ => println!("Error: unknown user browsing error type"),
                },
                Err(e) => match e {
                    ConnectionClosed => {
                        num_of_closed += 1;
                        num_of_visit += 1;
                    }
                    _ => {
                        println!(
                            "User browsing failed for url {} with user {} :{:?}",
                            url, user, e
                        );
                        num_of_err += 1;
                        num_of_visit += 1;
                    }
                },
            }
        }
    }

    let total = elapsed_time.iter().sum();

    if num_of_visit > 0 {
        Some((
            num_of_ok,
            num_of_err,
            num_of_timeout,
            num_of_closed,
            elapsed_time.len(),
            total,
        ))
    } else {
        None
    }
}
