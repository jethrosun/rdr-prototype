use failure::Fallible;
use headless_chrome::LaunchOptions;
use headless_chrome::{browser::context::Context, Browser, Tab};
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::vec::Vec;

/// Construct the workload from the session file.
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

            let mut broken_urls = HashSet::new();
            broken_urls.insert("32.wcmcs.net");
            broken_urls.insert("provider-directory.anthem.com");
            broken_urls.insert("kr.sports.yahoo.com");
            broken_urls.insert("desktopfw.weather.com");
            broken_urls.insert("arienh4.net.nyud.net");
            broken_urls.insert("hv3.webstat.com");
            broken_urls.insert("rs.mail.ru");
            broken_urls.insert("arienh4.net.nyud.net");
            broken_urls.insert("apps.facebook.com");
            broken_urls.insert("ads.adultadvertising.net");
            broken_urls.insert("reuters.com");
            broken_urls.insert("pn1.adserver.yahoo.com");
            broken_urls.insert("bbc.co.uk");
            broken_urls.insert("ad.yieldmanager.com");
            broken_urls.insert("wikipedia.org");
            broken_urls.insert("collegehumor.com");
            broken_urls.insert("talenthunter.com");
            broken_urls.insert("naver.com");
            broken_urls.insert("blog.naver.com");
            broken_urls.insert("mads.gamespot.com");
            broken_urls.insert("cyworld.com");
            broken_urls.insert("penciloflight.deviantart.com");
            broken_urls.insert("grad2b.com");
            broken_urls.insert("sports.sina.com.cn");
            broken_urls.insert("diligogames.com");
            broken_urls.insert("llbean.com");
            broken_urls.insert("focusbaiduafp.allyes.com");
            broken_urls.insert("gdx.mlb.com");
            broken_urls.insert("pfp.sina.com.cn");
            broken_urls.insert("bbs.20jh.net");
            broken_urls.insert("kr.yahoo.com");
            broken_urls.insert("limeusa.com");
            broken_urls.insert("l.qq.com");
            broken_urls.insert("co101w.col101.mail.live.com");
            broken_urls.insert("xp.trafficmp.com");
            broken_urls.insert("rockyou.com");
            broken_urls.insert("community.thebump.com");
            broken_urls.insert("simfile.chol.com");
            broken_urls.insert("deviantart.com");
            broken_urls.insert("colbertnation.com");
            broken_urls.insert("hcr.com");
            broken_urls.insert("sportsillustrated.cnn.com");
            broken_urls.insert("lg15.com");
            broken_urls.insert("nate.com");
            broken_urls.insert("talkonsex.com");
            broken_urls.insert("hulu.com");
            broken_urls.insert("proxy.medlib.iupui.edu");
            broken_urls.insert("ad.insightexpressai.com");
            broken_urls.insert("bs.serving-sys.com");
            broken_urls.insert("wpi.renren.com");
            broken_urls.insert("iea.org");
            broken_urls.insert("auth.ulib.iupui.edu");
            broken_urls.insert("womenofyoutube.mevio.com");
            broken_urls.insert("idcs.interclick.com");
            broken_urls.insert("fpsgameservers.com");
            broken_urls.insert("byfiles.storage.live.com");

            broken_urls.insert("imx.comedycentral.com");
            broken_urls.insert("lovine.com");
            broken_urls.insert("stoo.asiae.co.kr");
            broken_urls.insert("bing.com");
            broken_urls.insert("espn-www.baynote.net");
            broken_urls.insert("ad.scanmedios.com");
            broken_urls.insert("graphics.ocsn.com");
            broken_urls.insert("web.ebscohost.com");
            broken_urls.insert("d.tradex.openx.com");
            broken_urls.insert("br.fling.com");
            broken_urls.insert("video-stats.video.google.com");
            broken_urls.insert("dataandsearch.org");
            broken_urls.insert("optimized-by.rubiconproject.com");
            broken_urls.insert("view.atdmt.com");
            broken_urls.insert("fbpr1.farmville.zynga.com");
            broken_urls.insert("ro-co1.exp.msn.com");
            broken_urls.insert("nursingsociety.org");
            broken_urls.insert("ads.brazzers.com");
            broken_urls.insert("google.com");
            broken_urls.insert("widget.chipin.com");
            broken_urls.insert("movie.tudou.com");
            broken_urls.insert("farfesh.com");
            broken_urls.insert("nike.com");
            broken_urls.insert("slis.indiana.edu");
            broken_urls.insert("sesamestats.com");
            broken_urls.insert("apple.com");
            broken_urls.insert("xjjh.com");
            broken_urls.insert("facebook.com");
            broken_urls.insert("srv2.wa.marketingsolutions.yahoo.com");
            broken_urls.insert("hi.csdn.net");
            broken_urls.insert("microsoft.com");
            broken_urls.insert("yahoo.com");
            broken_urls.insert("mitbbs.com");
            broken_urls.insert("images.accessmylibrary.com");
            broken_urls.insert("accessmylibrary.com");
            broken_urls.insert("imeem.com");
            broken_urls.insert("freakingnews.com");
            broken_urls.insert("teendestruction.com");
            broken_urls.insert("picnik.com");
            broken_urls.insert("otniga.blu.livefilestore.com");
            broken_urls.insert("sports.yahoo.com");
            broken_urls.insert("m1.2mdn.net");
            broken_urls.insert("burstbeacon.com");
            broken_urls.insert("static16.photo.sina.com.cn");
            broken_urls.insert("uuseeafp.allyes.com");
            broken_urls.insert("jcmc.indiana.edu");
            broken_urls.insert("v.youku.com");
            broken_urls.insert("cnn.com");
            broken_urls.insert("animalbehavior.org");
            broken_urls.insert("a367.yahoofs.com");
            broken_urls.insert("dvdvideosoft.com");
            broken_urls.insert("promos.fling.com");
            broken_urls.insert("dygod.com");
            broken_urls.insert("megavideo.com");
            broken_urls.insert("renren.com");
            broken_urls.insert("ad.globalinteractive.com");
            broken_urls.insert("isdspeed.qq.com");
            broken_urls.insert("pixel.invitemedia.com");
            broken_urls.insert("youtube.com");
            broken_urls.insert("ad.inven.co.kr");
            broken_urls.insert("t.mookie1.com");
            broken_urls.insert("spitefulcritic.com");
            broken_urls.insert("msn.foxsports.com");
            broken_urls.insert("msn.com");
            broken_urls.insert("ad.adserverplus.com");
            broken_urls.insert("mediaservices.myspace.com");
            broken_urls.insert("pornhub.com");
            broken_urls.insert("ad.trafficmp.com");
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();
            // broken_urls.insert();

            if broken_urls.contains(urls.unwrap()[1].as_str().unwrap()) {
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

    let driver_path = PathBuf::from(r"/usr/bin/chromium-browser");
    // .path(Some(driver_path)) //

    let options = LaunchOptions::default_builder()
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

pub fn user_browse(current_browser: &Browser, hostname: &String) -> Fallible<()> {
    // std::result::Result<(u128), (u128, failure::Error)> {
    let now = Instant::now();

    println!("1");
    let tab = current_browser.new_tab()?;

    let https_hostname = "https://".to_string() + &hostname;
    // let https_hostname = "https://google.com".to_string();
    println!("{:?}", https_hostname);

    // tab.navigate_to(&https_hostname)?.wait_until_navigated()?;
    println!("2");
    tab.navigate_to(&https_hostname)?;

    // tab.wait_until_navigated()?;

    sleep(Duration::from_millis(10));
    // println!("3");
    // tab.wait_until_navigated()?;

    // println!("4");
    // let html = match tab.wait_for_element("html") {
    //     Ok(h) => {
    //         println!("html ok");
    //         ()
    //     }
    //     Err(e) => {
    //         println!("Query failed: {:?}", e);
    //         ()
    //     }
    // };

    // tab.close_target();
    // println!("here");
    // Ok(html)

    Ok(())
}

// pub fn browser_ctx_create() -> Fallible<Context<'static>> {
//     let browser = browser_create().unwrap();
//     let ctx = browser.new_context().unwrap();
//
//     Ok(ctx)
// }

pub fn browser_tab_create() -> Fallible<Arc<Tab>> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;

    // println!("Browser created",);
    Ok(tab)
}

pub fn user_tab_browse(
    current_tab: &Tab,
    hostname: &String,
) -> std::result::Result<(u128), (u128, failure::Error)> {
    let now = Instant::now();
    // println!("Entering user browsing",);
    // Doesn't use incognito mode
    //
    // let current_tab = match current_browser.new_tab() {
    //     Ok(tab) => tab,
    //     Err(e) => return Err((now.elapsed().as_micros(), e)),
    // };

    // Incogeneto mode
    //
    // let incognito_cxt = current_browser.new_context()?;
    // let current_tab: Arc<Tab> = incognito_cxt.new_tab()?;

    let https_hostname = "https://".to_string() + &hostname;

    // wait until navigated or not
    let navigate_to = match current_tab.navigate_to(&https_hostname) {
        Ok(tab) => tab,
        Err(e) => {
            return Err((now.elapsed().as_micros(), e));
        }
    };
    // let _ = current_tab.navigate_to(&https_hostname)?;
    let result = match navigate_to.wait_until_navigated() {
        Ok(_) => Ok(now.elapsed().as_micros()),
        Err(e) => Err((now.elapsed().as_micros(), e)),
    };

    result
}

pub fn simple_user_browse(
    current_browser: &Browser,
    hostname: &String,
) -> std::result::Result<(u128), (u128, failure::Error)> {
    let now = Instant::now();
    // println!("Entering user browsing",);
    // Doesn't use incognito mode
    //
    let current_tab = match current_browser.new_tab() {
        Ok(tab) => tab,
        Err(e) => return Err((now.elapsed().as_micros(), e)),
    };

    // Incogeneto mode
    //
    // let incognito_cxt = current_browser.new_context()?;
    // let current_tab: Arc<Tab> = incognito_cxt.new_tab()?;

    let https_hostname = "https://".to_string() + &hostname;

    // wait until navigated or not
    let result = match current_tab.navigate_to(&https_hostname) {
        Ok(_) => Ok(now.elapsed().as_micros()),
        // Err(e) => Err((now.elapsed().as_micros(), e)),
        Err(e) => Err((now.elapsed().as_micros(), e)),
    };

    // println!("{:?}", result);
    result
}

/// RDR proxy browsing scheduler.
///
///
// 4 [(4636, "fanfiction.net"), (9055, "bs.serving-sys.com")]
pub fn rdr_scheduler(
    now: Instant,
    pivot: &usize,
    num_of_ok: &mut usize,
    num_of_err: &mut usize,
    elapsed_time: &mut Vec<u128>,
    _num_of_users: &usize,
    current_work: Vec<(u64, String, usize)>,
    browser_list: &Vec<Browser>,
) {
    // println!("\npivot: {:?}", pivot);
    // println!("current work {:?}", current_work);

    for (milli, url, user) in current_work.into_iter() {
        println!("User {:?}: milli: {:?} url: {:?}", user, milli, url);
        println!("DEBUG: {:?} {:?}", now.elapsed().as_millis(), milli);

        // if now.elapsed().as_millis() < milli as u128 {
        //     println!("DEBUG: waiting");
        //     let one_millis = Duration::from_millis(1);
        //     std::thread::sleep(one_millis);
        // } else {
        println!("DEBUG: matched");
        match user_browse(&browser_list[user], &url) {
            Ok(elapsed) => {
                println!("ok");
                // *num_of_ok += 1;
                // elapsed_time.push(elapsed);
            }
            // Err((elapsed, e)) => {
            Err(e) => {
                // println!("err");
                // *num_of_err += 1;
                // elapsed_time.push(elapsed);
                // println!("User {} caused an error: {:?}", user, e);
                println!("User {} caused an error", user,);
            }
        }
        println!("match done");
        // }
    }

    // println!(
    //     "(pivot {}) RDR Scheduling: {:?} {:?}",
    //     pivot, num_of_ok, num_of_err
    // );
    // println!("(pivot {}) RDR Elapsed Time:  {:?}", pivot, elapsed_time);
}

pub fn rdr_scheduler_ng(
    now: Instant,
    pivot: &usize,
    num_of_ok: &mut usize,
    num_of_err: &mut usize,
    elapsed_time: &mut Vec<u128>,
    _num_of_users: &usize,
    current_work: Vec<(u64, String, usize)>,
    tab_list: &Vec<Arc<Tab>>,
) {
    // println!("\npivot: {:?}", pivot);
    // println!("current work {:?}", current_work);

    for (milli, url, user) in current_work.into_iter() {
        println!("User {:?}: milli: {:?} url: {:?}", user, milli, url);
        println!("DEBUG: {:?} {:?}", now.elapsed().as_millis(), milli);

        if now.elapsed().as_millis() < milli as u128 {
            println!("DEBUG: waiting");
            let one_millis = Duration::from_millis(1);
            std::thread::sleep(one_millis);
        } else {
            println!("DEBUG: matched");
            match user_tab_browse(&tab_list[user], &url) {
                Ok(elapsed) => {
                    println!("ok");
                    // *num_of_ok += 1;
                    // elapsed_time.push(elapsed);
                }
                Err((elapsed, e)) => {
                    println!("err");
                    // *num_of_err += 1;
                    // elapsed_time.push(elapsed);
                    println!("User {} caused an error: {:?}", user, e);
                    // println!("User {} caused an error", user,);
                }
            }
        }
    }
}
