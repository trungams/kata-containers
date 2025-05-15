use std::{collections::HashMap, os::fd::RawFd, sync::Arc};

use anyhow::{anyhow, Result};
use arbitrary;
use image_rs::image::ImageClient;
use protocols::agent_ttrpc_async as agent_ttrpc;
use rand::Rng;
use safe_path::scoped_join;
use std::fs;
use std::path::PathBuf;
use ttrpc::{r#async::TtrpcContext as Context, MessageHeader};

const IMAGE_WORK_DIR: &str = "/run/kata-containers/test_image/";
const CONTAINER_BASE_TEST: &str = "/run/kata-containers/testing/";

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

// pulls the container image referenced in `image` using image-rs
// and returns the bundle path containing the rootfs (mounted by
// the underlying snapshotter, overlayfs in this case) & config.json
// Uses anonymous image registry authentication.
pub async fn pull_image(image: &str, cid: &str) -> Result<String> {
    if image.is_empty() || cid.is_empty() {
        return Err(anyhow!("invalid image reference or container id"));
    }

    let mut image_client = ImageClient::new(PathBuf::from(IMAGE_WORK_DIR));
    image_client.config.auth = false;
    image_client.config.security_validate = false;

    fs::create_dir_all(CONTAINER_BASE_TEST)?;

    let bundle_dir = scoped_join(CONTAINER_BASE_TEST, cid)?;
    fs::create_dir_all(bundle_dir.clone())?;

    image_client
        .pull_image(image, &bundle_dir, &None, &None)
        .await?;

    Ok(bundle_dir.as_path().display().to_string())
}
