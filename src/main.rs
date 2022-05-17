use fake::faker::address::raw::*;
use fake::faker::company::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::Rng;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::thread;

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();

    for files in 1..10 {
        let handle = thread::spawn( move || {
            let file_path: OsString = get_first_arg()
            .unwrap_or_else(|error| panic!("unable to get first argument:  {:?}", error));
            let mut rng = rand::thread_rng();
            let files_int8: u8 = files;
            let files_str: String = files_int8.to_string();
            let sep: String = String::from("-");
            let file_path_str_ref: &str = file_path.to_str().unwrap();
            let file_path_str: String = String::from(file_path_str_ref);
            let full_path: String = [file_path_str, sep, files_str, String::from(".csv")].concat();
            let mut wtr = csv::Writer::from_path(full_path).unwrap();
            wtr.write_record(&[
                "First Name",
                "Last Name",
                "Username",
                "User Agent",
                "Employer",
                "Email",
                "Street",
                "City",
                "State",
                "Postal Code",
                "Country",
                "Orders",
                "Order Amount",
            ])
            .expect("unable to write header");
            let mut i = 0;
            while i < 1000000 {
                let first_name: String = FirstName(FR_FR).fake();
                let last_name: String = LastName(FR_FR).fake();
                let username: String = Username(FR_FR).fake();
                let useragent: String = UserAgent(FR_FR).fake();
                let employer: String = CompanyName(FR_FR).fake();
                let email: String = FreeEmail(FR_FR).fake();
                let street: String = StreetName(FR_FR).fake();
                let city: String = CityName(FR_FR).fake();
                let state: String = StateName(FR_FR).fake();
                let post_code: String = PostCode(FR_FR).fake();
                let country: String = CountryCode(FR_FR).fake();
                let orders: u32 = rng.gen_range(0..200);
                let total_order_amount: f64 = rng.gen_range(0.0..1000000.0);
                wtr.write_record(&[
                    first_name,
                    last_name,
                    username,
                    useragent,
                    employer,
                    email,
                    street,
                    city,
                    state,
                    post_code,
                    country,
                    orders.to_string(),
                    total_order_amount.to_string(),
                ])
                .expect("unable to write row");
                i = i + 1
            }
            wtr.flush().expect("unable to flush");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
