use std::path::Path;

use util::{self, CargoResult, internal, ChainError};

pub struct Rustc {
    pub verbose_version: String,
    pub host: String,
    /// Backwards compatibility: does this compiler support `--cap-lints` flag?
    pub cap_lints: bool,
}

impl Rustc {
    /// Run the compiler at `path` to learn various pieces of information about
    /// it.
    ///
    /// If successful this function returns a description of the compiler along
    /// with a list of its capabilities.
    pub fn new<P: AsRef<Path>>(path: P) -> CargoResult<Rustc> {
        let mut cmd = util::process(path.as_ref());
        cmd.arg("-vV");

        let mut first = cmd.clone();
        first.arg("--cap-lints").arg("allow");

        let (cap_lints, output) = match first.exec_with_output() {
            Ok(output) => (true, output),
            Err(..) => (false, try!(cmd.exec_with_output())),
        };

        let verbose_version = try!(String::from_utf8(output.stdout).map_err(|_| {
            internal("rustc -v didn't return utf8 output")
        }));

        let host = {
            let triple = verbose_version.lines().find(|l| {
                l.starts_with("host: ")
            }).map(|l| &l[6..]);
            let triple = try!(triple.chain_error(|| {
                internal("rustc -v didn't have a line for `host:`")
            }));
            triple.to_string()
        };

        Ok(Rustc {
            verbose_version: verbose_version,
            host: host,
            cap_lints: cap_lints,
        })
    }
}
