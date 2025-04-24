#![no_main]

#[macro_use]
extern crate lazy_static;

use arbitrary::Arbitrary;
use kata_agent_fuzz::fuzz_proto_types as fuzz_types;
use libfuzzer_sys::fuzz_target;
use protocols::agent_ttrpc_async as agent_ttrpc;
use std::collections::HashMap;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use tokio;
use ttrpc::proto::MessageHeader;
use ttrpc::r#async::TtrpcContext as Context;

lazy_static! {
    static ref DEFAULT_CONTEXT: Context = Context {
        fd: -1 as RawFd,
        mh: MessageHeader::default(),
        metadata: HashMap::new(),
        timeout_nano: 0,
    };
}

fn agent_service() -> Arc<Box<dyn agent_ttrpc::AgentService + Send + Sync>> {
    kata_agent::fuzz_logic::AGENT_SERVICE
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .clone()
}

#[derive(Arbitrary, Debug, Clone)]
enum Op {
    CreateContainer(fuzz_types::FuzzCreateContainerRequest),
    RemoveContainer(fuzz_types::FuzzRemoveContainerRequest),
}

const OPS_COUNT: usize = 20;

fuzz_target!(
    init: {
        lazy_static::initialize(&DEFAULT_CONTEXT);
        kata_agent::fuzz_logic::init_agent().unwrap();
    },
    |ops: Vec<Op>| {

    let agent = agent_service();

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let ctx = &DEFAULT_CONTEXT;
    let mut requests = vec![];
    for (idx, op) in ops.into_iter().enumerate() {
        // println!("{:?}", op);
        requests.push(op.clone());
        rt.block_on(match op {
            Op::CreateContainer(req) => agent.create_container(ctx, req.into()),
            Op::RemoveContainer(req) => agent.remove_container(ctx, req.into())
        }).unwrap();
        if idx + 1 == OPS_COUNT {
            break;
        }
    }
    println!("end of fuzz target run: {:?}", requests);
});
