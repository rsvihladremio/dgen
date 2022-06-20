/*   Copyright 2022 Ryan SVIHLA

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/
use fake::faker::address::raw::*;
use fake::faker::company::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::prelude::ThreadRng;
use rand::Rng;
use uuid::Uuid;

pub trait Strategy: Sync + 'static {
    fn headers(&self) -> Vec<String>;
    fn get_row(&self) -> Vec<String>;
}

pub fn NewContractStrategy() -> ContactStrategy {
    let mut rng = rand::thread_rng();

    return ContactStrategy {
        rng: rand::thread_rng(),
    };
}
pub struct ContactStrategy {
    pub rng: ThreadRng,
}

impl Strategy for ContactStrategy {
    fn headers(&self) -> Vec<String> {
        return vec![
            "First Name".to_string(),
            "Last Name".to_string(),
            "Username".to_string(),
            "User Agent".to_string(),
            "Employer".to_string(),
            "Email".to_string(),
            "Street".to_string(),
            "City".to_string(),
            "State".to_string(),
            "Postal Code".to_string(),
            "Country".to_string(),
            "Orders".to_string(),
            "Order Amount".to_string(),
        ];
    }

    fn get_row(&self) -> Vec<String> {
        let first_name: String = FirstName(FR_FR).fake();
        let last_name: String = LastName(FR_FR).fake();

        let username: String = Uuid::new_v4().to_string();
        let useragent: String = UserAgent(FR_FR).fake();
        let employer: String = CompanyName(FR_FR).fake();
        let email: String = FreeEmail(FR_FR).fake();
        let street: String = StreetName(FR_FR).fake();
        let city: String = CityName(FR_FR).fake();
        let state: String = StateName(FR_FR).fake();
        let post_code: String = PostCode(FR_FR).fake();
        let country: String = CountryCode(FR_FR).fake();
        let orders: u32 = self.rng.gen_range(0..200);
        let total_order_amount: f64 = self.rng.gen_range(0.0..1000000.0);
        return vec![
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
        ];
    }
}
unsafe impl Sync for ContactStrategy {}

pub fn NewTimeSeriesStrategy() -> TimeSeriesStrategy {
    let mut rng = rand::thread_rng();

    return TimeSeriesStrategy {
        rng: rand::thread_rng(),
    };
}
pub struct TimeSeriesStrategy {
    pub rng: ThreadRng,
}
impl Strategy for TimeSeriesStrategy {
    fn headers(&self) -> Vec<String> {
        return vec![
            "time_bucket".to_string(),
            "location".to_string(),
            "device".to_string(),
            "avg".to_string(),
            "min".to_string(),
            "max".to_string(),
        ];
    }

    fn get_row(&self) -> Vec<String> {
        return vec![];
    }
}
unsafe impl Sync for TimeSeriesStrategy {}
