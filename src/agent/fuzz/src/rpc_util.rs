use std::{collections::HashMap, os::fd::RawFd, sync::Arc};

use arbitrary;
use protocols::agent_ttrpc_async as agent_ttrpc;
use rand::Rng;
use ttrpc::{r#async::TtrpcContext as Context, MessageHeader};

pub fn default_context() -> Context {
    Context {
        fd: -1 as RawFd,
        mh: MessageHeader::default(),
        metadata: HashMap::new(),
        timeout_nano: 0,
    }
}

pub fn agent_service() -> Arc<Box<dyn agent_ttrpc::AgentService + Send + Sync>> {
    kata_agent::fuzz_logic::AGENT_SERVICE
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .clone()
}

pub fn arbitrary_container_id(u: &mut arbitrary::Unstructured) -> arbitrary::Result<String> {
    let bytes = u.bytes(32)?;
    let s = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(s)
}

pub fn random_container_id() -> String {
    const CHARSET: &[u8] = b"abcdef0123456789";
    let mut rng = rand::thread_rng();

    let id: String = (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    id
}
