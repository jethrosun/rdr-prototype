extern crate base64;
extern crate tiny_http;

use failure::Fallible;
use headless_chrome::browser::tab::RequestInterceptionDecision;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::protocol::network::Cookie;
use headless_chrome::protocol::runtime::methods::{RemoteObjectSubtype, RemoteObjectType};
use headless_chrome::protocol::RemoteError;
use headless_chrome::{
    browser::context::Context,
    protocol::browser::{Bounds, WindowState},
    Browser, Tab,
};
use lib::*;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod lib;

fn main() {
    // Workloads:

    // "/home/jethros/dev/projects/silver-octo-spoon/workload_tempaltes/rdr_pvn_workload.json";
    // "/home/jethros/dev/silver-octo-spoon/workload_tempaltes/rdr_pvn_workload.json";
    // let workload_path = "/Users/jethros/dev/pvn/utils/workloads/rdr_pvn_workload.json";
    // "/Users/jethros/dev/pvn/utils/workloads/rdr_pvn_workloads/rdr_pvn_workload_1.json";
    let workload_path =
        "/home/jethros/dev/pvn/utils/workloads/rdr_pvn_workloads/rdr_pvn_workload_5.json";

    let num_of_users = 10;
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

        // let ctx = browser_ctx_create().unwrap();
        // ctx_list.push(ctx);
    }
    println!("{} browsers are created ", num_of_users);

    let mut pivot = 1 as usize;

    let mut num_of_ok = 0;
    let mut num_of_err = 0;
    let mut elapsed_time: Vec<u128> = Vec::new();

    let now = Instant::now();

    loop {
        if now.elapsed().as_secs() == pivot as u64 {
            let min = pivot / 60;
            let rest_sec = pivot % 60;
            println!("\n{:?} min, {:?} second", min, rest_sec);
            match rdr_workload.remove(&pivot) {
                Some(wd) => {
                    rdr_scheduler(
                        now.clone(),
                        &pivot,
                        &mut num_of_ok,
                        &mut num_of_err,
                        &mut elapsed_time,
                        &num_of_users,
                        wd,
                        &browser_list,
                    );
                    // rdr_scheduler(
                    //     now.clone(),
                    //     &pivot,
                    //     &mut num_of_ok,
                    //     &mut num_of_err,
                    //     &mut elapsed_time,
                    //     &num_of_users,
                    //     wd,
                    //     &browser_list,
                    // );
                }
                None => println!("No workload for second {:?}", pivot),
            }
            pivot += 1;
        }
    }
}
