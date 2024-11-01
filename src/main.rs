use std::{error::Error, fs::File, io::{Cursor, Read, Write}, path::PathBuf};

use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(version = "0.1.0", about = "Convert tsv to csv.", long_about = None)]
struct Args{
    #[arg(short, long)]
    input: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();

    let input = {
        let mut input = String::new();

        if let Some(path) = args.input {
            File::open(path)?.read_to_string(&mut input)?;
        } else {
            std::io::stdin().read_to_string(&mut input)?;
        }

        input
    };

    let output = {
        let mut output = vec![];

        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(Cursor::new(input))
            ;

        {
            let mut wtr = csv::Writer::from_writer(Cursor::new(&mut output));

            for result in rdr.into_records() {
                let record = result?;
                wtr.write_record(record.iter())?;
            }
        }

        output
    };

    if let Some(path) = args.output {
        File::create(path)?.write_all(&output)?;
    } else {
        std::io::stdout().write_all(&output)?;
    }

    Ok(())
}
