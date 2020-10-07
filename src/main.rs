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
use std::fs;
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
        "/net/data/pvn/dev/utils/workloads/rdr_pvn_workloads/rdr_pvn_workload_5.json";

    let num_of_users = 100;
    let num_of_secs = 600;

    let mut rdr_workload =
        rdr_load_workload(workload_path.to_string(), num_of_secs, num_of_users).unwrap();
    println!("Workload is generated",);

    // Browser list.
    let mut browser_list: Vec<Browser> = Vec::new();
    // Tab list
    let mut tab_list: Vec<Arc<Tab>> = Vec::new();
    // Context list
    let mut ctx_list: Vec<Arc<Context>> = Vec::new();

    for _ in 0..num_of_users {
        let browser = browser_create().unwrap();
        browser_list.push(browser);
    }
    println!("{} browsers are created ", num_of_users);

    let mut pivot = 1 as usize;

    let mut num_of_ok = 0;
    let mut num_of_err = 0;
    let mut elapsed_time: Vec<u128> = Vec::new();

    let now = Instant::now();

    for pivot in 0..610 {
        let min = pivot / 60;
        let rest_sec = pivot % 60;
        println!("\n{:?} min, {:?} second", min, rest_sec);
        match rdr_workload.remove(&pivot) {
            Some(wd) => {
                rdr_scheduler_ng(&pivot, wd, &browser_list);
            }
            None => println!("No workload for second {:?}", pivot),
        }
    }

    println!("Time elapsed: {:?}", now.elapsed().as_secs());
    Ok(())
}
