use rayon::prelude::*;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

mod ice;

use ice::{Outcome, TestResult, ICE};

static ICES_PATH: &str = "ices";

fn test(path: &Path) -> Result<(), Box<dyn Error>> {
    let failed = Arc::new(AtomicBool::new(false));
    let result_lock = Arc::new(Mutex::new(()));
    ice::discover(path)?
        .par_iter()
        .map(|ice| {
            let result = ice.test().map_err(|e| e.to_string())?;
            report_result(result_lock.clone(), &ice, &result);
            if result.outcome() != Outcome::ICEd {
                failed.store(true, Ordering::SeqCst);
            }
            Ok(())
        })
        .collect::<Result<Vec<()>, String>>()?;

    if failed.load(Ordering::SeqCst) {
        return Err("some ICEs are now fixed!".into());
    }
    Ok(())
}

fn report_result(lock: Arc<Mutex<()>>, ice: &ICE, res: &TestResult) {
    // Prevent multiple results being reported at the same time
    let _ = lock.lock();

    match res.outcome() {
        Outcome::NoError => println!("{}: fixed with no errors", ice.path().display()),
        Outcome::Errored => println!("{}: fixed with errors", ice.path().display()),
        Outcome::ICEd => println!("{}: ICEd", ice.path().display()),
    }
    if res.outcome() != Outcome::ICEd {
        println!("=== stdout ===");
        println!("{}", res.stdout());
        println!("=== stderr ===");
        println!("{}", res.stderr());
        println!("==============");
    }
}

fn main() {
    if let Err(err) = test(&Path::new(ICES_PATH)) {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}
