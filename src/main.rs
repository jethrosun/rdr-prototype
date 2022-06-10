extern crate anyhow;
extern crate tiny_http;

use failure::Fallible;
use headless_chrome::Browser;
use std::collections::HashMap;
use std::time::Instant;
use utils::*;
mod unresolvable;
mod utils;

fn main() -> Fallible<()> {
    let num_of_users = 40;
    let iter = 0;
    let rdr_users = rdr_read_rand_seed(num_of_users, iter).unwrap();

    // States that this NF needs to maintain.
    //
    // The RDR proxy network function needs to maintain a list of active headless browsers. This is
    // for the purpose of simulating multi-container extension in Firefox and multiple users. We
    // also need to maintain a content cache for the bulk HTTP request and response pairs.
    // let workload_path = "/Users/jethros/dev/pvn/utils/workloads/rdr_pvn_workloads/rdr_pvn_workload_"
    //     .to_owned()
    //     + &iter.to_string() + ".json";
    // let workload_path = "/net/data/pvn/dev/pvn/utils/workloads/rdr_pvn_workloads/rdr_pvn_workload_5.json";
    let workload_path = "rdr_pvn_workload.json";
    // println!("{:?}", workload_path);
    let num_of_secs = 180;

    let mut rdr_workload =
        rdr_load_workload(workload_path.to_string(), num_of_secs, rdr_users.clone()).unwrap();
    println!("Workload is generated",);
    // println!("workload: {:?}", rdr_workload);

    // Browser list.
    let mut browser_list: HashMap<i64, Browser> = HashMap::new();

    for user in &rdr_users {
        let browser = browser_create().unwrap();
        browser_list.insert(*user, browser);
    }
    println!("{} browsers are created ", num_of_users);

    let _pivot = 1_usize;

    // Metrics for measurement
    let mut elapsed_time = Vec::new();
    let mut num_of_ok = 0;
    let mut num_of_err = 0;
    let mut num_of_timeout = 0;
    let mut num_of_closed = 0;
    let mut num_of_visit = 0;

    let now = Instant::now();
    println!("Timer started");

    // Scheduling browsing jobs.
    // FIXME: This is not ideal as we are not actually schedule browse.
    for cur_time in 0..180 {
        if rdr_workload.contains_key(&cur_time) {
            // println!("pivot {:?}", cur_time);
            let min = cur_time / 60;
            let rest_sec = cur_time % 60;
            if let Some(wd) = rdr_workload.remove(&cur_time) {
                println!("{:?} min, {:?} second", min, rest_sec);
                if let Some((oks, errs, timeouts, closeds, visits, elapsed)) =
                    rdr_scheduler_ng(&cur_time, &rdr_users, wd, &browser_list)
                {
                    num_of_ok += oks;
                    num_of_err += errs;
                    num_of_timeout += timeouts;
                    num_of_closed += closeds;
                    num_of_visit += visits;
                    elapsed_time.push(elapsed);
                }
            }
        }

        println!("Time elapsed: {:?}", now.elapsed().as_secs());
    }
    println!("Metric: num_of_oks: {:?}, num_of_errs: {:?}, num_of_timeout: {:?}, num_of_closed: {:?}, num_of_visit: {:?}",
                    num_of_ok, num_of_err, num_of_timeout, num_of_closed, num_of_visit,
                );
    println!("Metric: Browsing Time: {:?}\n", elapsed_time);
    Ok(())
}
