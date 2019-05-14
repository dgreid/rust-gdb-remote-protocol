extern crate env_logger;
extern crate gdb_remote_protocol;

use gdb_remote_protocol::{
    process_packets_from, ContinueStatus, Error, Handler, ProcessType, StopReason,
};

struct NoopHandler;

impl Handler for NoopHandler {
    fn attached(&self, _pid: Option<u64>) -> Result<ProcessType, Error> {
        Ok(ProcessType::Created)
    }

    fn halt_reason(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(5))
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    drop(env_logger::init());
    let h = NoopHandler;
    let stdin_handle = std::io::stdin();
    let stdin = stdin_handle.lock();
    let stdout_handle = std::io::stdout();
    let stdout = stdout_handle.lock();
    process_packets_from(stdin, stdout, h);
}
