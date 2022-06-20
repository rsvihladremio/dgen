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

use std::error::Error;
use std::thread;

use crate::fakes::Strategy;
use crate::writer;

fn run(
    file_name_prefix: String,
    records: u64,
    strategy: &dyn Strategy,
    custom_columns: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();
    for files in 1..10 {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let files_int8: u8 = files;
            let files_str: String = files_int8.to_string();
            let sep: String = String::from("-");
            let full_path: String =
                [file_name_prefix, sep, files_str, String::from(".csv")].concat();
            writer::output_csv(full_path, records, strategy).unwrap()
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
