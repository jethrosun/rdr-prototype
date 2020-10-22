use crate::unresolvable::curate_unresolvable_records;
use failure::Fallible;
// use headless_chrome::LaunchOptions;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{browser::context::Context, Browser, Tab};
use resolv::record::MX;
use resolv::{Class, RecordType, Resolver};
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::vec::Vec;
use std::{thread, time};

pub fn resolve_dns(
    now: Instant,
    pivot: &usize,
    _num_of_users: &usize,
    current_work: Vec<(u64, String, usize)>,
) {
    for (milli, url, user) in current_work.into_iter() {
        println!("{:?}", url);
        // You must create a mutable resolver object to hold the context.
        let mut resolver = Resolver::new().unwrap();

        // .query() and .search() are the main interfaces to the resolver.
        let mut response = resolver.query(url.as_bytes(), Class::IN, RecordType::MX);

        match response {
            Ok(ref val) => {
                // You can iterate through answers as follows.  You must specify the
                // type of record.  A run-time error will occur if the records
                // decoded are of the wrong type.
                for answer in response.unwrap().answers::<MX>() {
                    println!("{:?}", answer);
                }
            }
            Err(e) => println!("fail to resolve {:?} bar", url),
        }
        let ten_millis = time::Duration::from_millis(100);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
    }
}

pub fn rdr_read_rand_seed(num_of_users: usize, iter: String) -> Result<Vec<i64>> {
    let rand_seed_file = "/home/jethros/dev/pvn/utils/rand_number/rand.json";
    let mut rand_vec = Vec::new();
    let file = File::open(rand_seed_file).expect("rand seed file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");

    match json_data.get("rdr") {
        Some(rdr_data) => match rdr_data.get(&num_of_users.clone().to_string()) {
            Some(setup_data) => match setup_data.get(iter.clone().to_string()) {
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

            let mut broken_urls = curate_unresolvable_records();

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

// /usr/bin/chromedriver
// /usr/bin/chromium-browser
pub fn browser_create() -> Fallible<Browser> {
    let timeout = Duration::new(1000, 0);

    // let options = LaunchOptions::default_builder()
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
    hostname: &String,
    user: &i64,
) -> Fallible<(usize, u128)> {
    let now = Instant::now();
    let tabs = current_browser.get_tabs().lock().unwrap();
    let current_tab = tabs.iter().next().unwrap();
    let http_hostname = "http://".to_string() + &hostname;

    current_tab.navigate_to(&http_hostname)?;

    Ok((1, now.elapsed().as_millis()))
}

/// Simple user browse.
pub fn simple_user_browse_old(
    current_browser: &Browser,
    hostname: &String,
    user: &i64,
) -> Fallible<(usize, u128)> {
    let now = Instant::now();
    let current_tab = match current_browser.new_tab() {
        Ok(tab) => tab,
        Err(e) => match e {
            Timeout => {
                // thread::sleep(Duration::from_millis(300));
                thread::sleep(Duration::from_secs(1));

                let t = match current_browser.new_tab() {
                    Ok(tab) => tab,
                    Err(e) => {
                        println!(
                            "RDR Tab timeout after the second try for hostname: {:?} and user: {}",
                            hostname, user
                        );
                        return Ok((3, now.elapsed().as_millis()));
                    }
                };
                t
            }
            _ => {
                println!(
                    "RDR Tab failed for unknown reason hostname: {:?} and user: {}",
                    hostname, user
                );
                return Ok((2, now.elapsed().as_millis()));
            }
        },
    };

    let http_hostname = "http://".to_string() + &hostname;

    current_tab.navigate_to(&http_hostname)?;

    Ok((1, now.elapsed().as_millis()))
}

/// RDR proxy browsing scheduler.
pub fn rdr_scheduler_ng(
    pivot: &usize,
    rdr_users: &Vec<i64>,
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
                        println!(
                            "Closed: User browsing failed for url {} with user {}",
                            url, user
                        );
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
