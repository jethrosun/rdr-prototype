use crate::unresolvable::curate_unresolvable_records;
use failure::Fallible;
// use headless_chrome::LaunchOptions;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{browser::context::Context, Browser, Tab};
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::vec::Vec;
use std::{thread, time};

use resolv::record::MX;
use resolv::{Class, RecordType, Resolver};

///
/// https://kbknapp.github.io/doapi-rs/docs/serde/json/index.html

/// Construct the workload from the session file.
///
/// https://kbknapp.github.io/doapi-rs/docs/serde/json/index.html
pub fn rdr_load_workload(
    file_path: String,
    num_of_secs: usize,
    num_of_user: usize,
) -> Result<HashMap<usize, Vec<(u64, String, usize)>>> {
    // time in second, workload in that second
    // let mut workload = HashMap::<usize, HashMap<usize, Vec<(u64, String)>>>::with_capacity(num_of_secs);
    let mut workload = HashMap::<usize, Vec<(u64, String, usize)>>::with_capacity(num_of_secs);

    let file = File::open(file_path).expect("file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");
    // println!("{:?}", json_data);

    for sec in 0..num_of_secs {
        // println!("sec {:?}", sec,);
        // user, workload for that user
        //
        // sec = 266
        // sec_wd = {'86': [39, 'thumb.brandreachsys.com'], '23': [42, 'facebook.com'], '84': [86, 'ynetnews.com'], '33': [284, 'techbargains.com'], '9': [309, 'bing.com'], '76': [357, 'eventful.com'], '43': [365, 'ad.yieldmanager.com'], '63': [468, 'ads.brazzers.com'], '72': [520, 'sidereel.com'], '57': [586, 'daum.net'], '81': [701, 'target.com'], '95': [732, 'lezhin.com'], '88': [802, 'nba.com'], '49': [827, 'web4.c2.cyworld.com'], '27': [888, 'hv3.webstat.com'], '98': [917, 'youtube.com']}
        // let mut sec_wd = HashMap::<usize, Vec<(u64, String)>>::with_capacity(100);
        // we keep track of the millisecond appeared
        let mut millis: Vec<(u64, String, usize)> = Vec::new();

        // println!("{:?} {:?}", sec, json_data.get(sec.to_string()));
        let urls_now = match json_data.get(sec.to_string()) {
            Some(val) => val,
            None => continue,
        };
        // println!("{:?}", urls_now);
        for user in 0..num_of_user {
            let urls = match urls_now.get(user.to_string()) {
                Some(val) => val.as_array(),
                None => continue,
            };
            // println!("{:?}", urls.unwrap());

            // let unreachable_records = curate_unreachable_records();
            let unresolvable_records = curate_unresolvable_records();

            if unresolvable_records.contains(urls.unwrap()[1].as_str().unwrap())
            // || unreachable_records.contains(urls.unwrap()[1].as_str().unwrap())
            {
                continue;
            } else {
                millis.push((
                    urls.unwrap()[0].as_u64().unwrap(),
                    urls.unwrap()[1].as_str().unwrap().to_string(),
                    user,
                ));
            }
        }
        millis.sort();

        // sec_wd.insert(millis);

        // {'96': [53, 'video.od.visiblemeasures.com'],
        //  '52': [104, 'drift.qzone.qq.com'],
        //  '15': [153, 'club.myce.com'],
        //  '78': [180, 'ad.admediaprovider.com'],
        //  '84': [189, 'inkido.indiana.edu'],
        //  '34': [268, 'waterjet.net'],
        //  '97': [286, 'apple.com'],
        //  '6': [317, 'southparkstudios.com'],
        //  '14': [362, 'en.wikipedia.org'],
        //  '27': [499, 'google.com'],
        //  '42': [619, 'womenofyoutube.mevio.com'],
        //  '75': [646, 'news.msn.co.kr'],
        //  '30': [750, 'hcr.com'],
        //  '61': [759, 'blogs.medicine.iu.edu'],
        //  '70': [815, 'mini.pipi.cn'],
        //  '54': [897, 'msn.foxsports.com'],
        //  '29': [926, 'target.com']}
        // if sec == 599 {
        //     println!("{:?}, ", millis);
        // }
        // println!("\n{:?} {:?}", sec, millis);
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
pub fn simple_user_browse(current_browser: &Browser, hostname: &String) -> Fallible<()> {
    let now = Instant::now();
    println!("hostname: {:?}", hostname);
    let current_tab = match current_browser.new_tab() {
        Ok(tab) => tab,
        Err(e) => {
            println!("RDR Tab failed: {:?}", hostname);
            // println!("RDR Tab Error: timeout{:?}", e);
            match e {
                ConnectionClosed => println!("got connection closed"),
                Timeout => println!("got timeout"),
                _ => println!("unknown"),
            }
            return Ok(());
        }
    };

    let http_hostname = "http://".to_string() + &hostname;

    current_tab.navigate_to(&http_hostname)?;

    Ok(())
}

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

pub fn rdr_scheduler_ng(
    pivot: &usize,
    current_work: Vec<(u64, String, usize)>,
    browser_list: &Vec<Browser>,
) -> Result<()> {
    // println!("\npivot: {:?}", pivot);
    // println!("current work {:?}", current_work);

    for (milli, url, user) in current_work.into_iter() {
        println!("User {:?}: milli: {:?} url: {:?}", user, milli, url);

        simple_user_browse(&browser_list[user], &url);
    }
    Ok(())
}
