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

use crate::fakes::Strategy;

// simple wrapper function for csv writer code, that takes a file name, the headers,
// the number of records too add and a function to get rows to add
pub fn output_csv(
    file_name: String,
    records_to_add: u64,
    strategy: &dyn Strategy,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name)?;
    wtr.write_record(&strategy.headers())?;
    for i in 1..records_to_add {
        let row = strategy.get_row();
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    return Ok(());
}
