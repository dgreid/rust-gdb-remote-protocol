extern crate env_logger;
extern crate gdb_remote_protocol;

use gdb_remote_protocol::{
    process_packets_from, ContinueStatus, Error, Handler, ProcessType, StopReason,
};
use std::net::TcpListener;

struct NoopHandler;

impl Handler for NoopHandler {
    fn attached(&self, _pid: Option<u64>) -> Result<ProcessType, Error> {
        Ok(ProcessType::Created)
    }

    fn halt_reason(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(5))
    }

    fn interrupt(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(5))
    }

    fn read_general_registers(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![0; 64])
    }

    fn cont(&self, _addr: Option<u64>) -> Result<ContinueStatus, Error> {
        Ok(ContinueStatus {})
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    drop(env_logger::init());
    let listener = TcpListener::bind("0.0.0.0:2424").unwrap();
    println!("Listening on port 2424");
    for res in listener.incoming() {
        println!("Got connection");
        if let Ok(stream) = res {
            let h = NoopHandler;
            process_packets_from(stream.try_clone().unwrap(), stream, h);
        }
        println!("Connection closed");
    }
}
