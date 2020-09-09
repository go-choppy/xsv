use std::borrow::Cow;

use csv;
use tabwriter::TabWriter;

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;

static USAGE: &'static str = "
Format the fields' name as the snake case in the CSV data.

This is useful when you want to import a proper csv file to database

Usage:
    xsv snake [options] [<input>]

Common options:
    -h, --help             Display this message.
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(Deserialize)]
struct Args {
  arg_input: Option<String>,
  flag_output: Option<String>,
  flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
  let args: Args = util::get_args(USAGE, argv)?;
  let rconfig = Config::new(&args.arg_input)
    .delimiter(args.flag_delimiter)
    .no_headers(true);
  let wconfig = Config::new(&args.flag_output).delimiter(args.flag_delimiter);

  let tw = TabWriter::new(wconfig.io_writer()?);
  let mut wtr = wconfig.from_writer(tw);
  let mut rdr = rconfig.reader()?;

  let mut record = csv::ByteRecord::new();
  let mut i = 0;
  while rdr.read_byte_record(&mut record)? {
    if i == 0 {
      wtr.write_record(
        record
          .iter()
          .map(|f| util::to_snake(Cow::Borrowed(f)))
      )?;
    } else {
      wtr.write_record(&record)?;
    }

    i = i + 1;
  }
  wtr.flush()?;
  Ok(())
}
