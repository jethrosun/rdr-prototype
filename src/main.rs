extern crate base64;
extern crate tiny_http;

use failure::Fallible;
use headless_chrome::browser::tab::RequestInterceptionDecision;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::protocol::network::Cookie;
use headless_chrome::protocol::runtime::methods::{RemoteObjectSubtype, RemoteObjectType};
use headless_chrome::protocol::RemoteError;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{
    browser::context::Context,
    protocol::browser::{Bounds, WindowState},
    Browser, Tab,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::{Duration, Instant};
use utils::*;

mod unresolvable;
mod utils;

fn main() -> Fallible<()> {
    // Workloads:

    let workload_path =
        "/home/jethros/dev/pvn/utils/workloads/rdr_pvn_workloads/rdr_pvn_workload_3.json";

    let num_of_users = 100;
    let num_of_secs = 600;

    let rdr_users = rdr_read_rand_seed(100, 3.to_string()).unwrap();
    let mut rdr_workload =
        rdr_load_workload(workload_path.to_string(), num_of_secs, rdr_users.clone()).unwrap();
    println!("Workload is generated",);

    // Browser list.
    let mut browser_list: HashMap<i64, Browser> = HashMap::new();
    for user in &rdr_users {
        let browser = browser_create().unwrap();
        browser_list.insert(*user, browser);
    }
    println!("{} browsers are created ", num_of_users);

    let mut pivot = 1 as usize;

    // Metrics for measurement
    let mut elapsed_time = Vec::new();
    let mut num_of_ok = 0;
    let mut num_of_err = 0;
    let mut num_of_timeout = 0;
    let mut num_of_closed = 0;
    let mut num_of_visit = 0;

    let now = Instant::now();

    // Scheduling browsing jobs.
    // FIXME: This is not ideal as we are not actually schedule browse.
    for cur_time in 0..610 {
        if rdr_workload.contains_key(&cur_time) {
            println!("pivot {:?}", cur_time);
            let min = cur_time / 60;
            let rest_sec = cur_time % 60;
            println!("{:?} min, {:?} second", min, rest_sec);
            match rdr_workload.remove(&cur_time) {
                Some(wd) => match rdr_scheduler_ng(&cur_time, &rdr_users, wd, &browser_list) {
                    Some((oks, errs, timeouts, closeds, visits, elapsed)) => {
                        num_of_ok += oks;
                        num_of_err += errs;
                        num_of_timeout += timeouts;
                        num_of_closed += closeds;
                        num_of_visit += visits;
                        elapsed_time.push(elapsed);
                    }
                    None => println!("No workload for second {:?}", cur_time),
                },
                None => println!("No workload for second {:?}", cur_time),
            }
        }

        println!("Time elapsed: {:?}", now.elapsed().as_secs());
    }
    Ok(())
}
