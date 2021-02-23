use exitfailure::ExitFailure;
use snmp_walker::{self, Params};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let args = Params::from_args();
    snmp_walker::run(args)?;

    Ok(())
}
