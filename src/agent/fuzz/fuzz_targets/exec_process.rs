#![no_main]

use std::path::PathBuf;

use kata_agent_fuzz::{fuzz_proto, rpc_util};
use lazy_static::lazy_static;
use libfuzzer_sys::{fuzz_target, Corpus};
use tokio;
use ttrpc::{r#async::TtrpcContext, Error as TtrpcError};

lazy_static! {
    static ref SANDBOX_ID: String = rpc_util::random_container_id();
    static ref CONTAINER_ID: String = SANDBOX_ID.clone();
    static ref RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    static ref CTX: TtrpcContext = rpc_util::default_context();
}

fn initialize() {
    kata_agent::fuzz_logic::init_agent().unwrap();
    lazy_static::initialize(&SANDBOX_ID);
    lazy_static::initialize(&CONTAINER_ID);
    lazy_static::initialize(&RT);
    lazy_static::initialize(&CTX);

    let agent = rpc_util::agent_service();

    RT.block_on(async {
        agent
            .create_sandbox(
                &CTX,
                fuzz_proto::FuzzCreateSandboxRequest {
                    sandbox_id: SANDBOX_ID.clone(),
                    ..Default::default()
                }
                .into(),
            )
            .await
            .expect("failed to create sandbox");

        // pull image
        let bundle = rpc_util::pull_image("ghcr.io/linuxcontainers/alpine:latest", &CONTAINER_ID)
            .await
            .expect("failed to pull image");
        let mut oci = fuzz_proto::FuzzSpec::sample_spec();
        let path = PathBuf::from(&bundle)
            .join(&"rootfs")
            .into_os_string()
            .into_string()
            .unwrap();
        oci.root = Some(fuzz_proto::FuzzRoot {
            path: path,
            readonly: true,
            ..Default::default()
        });

        agent
            .create_container(
                &CTX,
                fuzz_proto::FuzzCreateContainerRequest {
                    container_id: CONTAINER_ID.clone(),
                    oci: Some(oci),
                    ..Default::default()
                }
                .into(),
            )
            .await
            .expect("failed to create container");
    });
}

fuzz_target!(
    init: {
        initialize();
    },
    |data: &[u8]| -> Corpus {
    let agent = rpc_util::agent_service();

    match RT.block_on(async {
        agent.exec_process(&CTX, fuzz_proto::FuzzExecProcessRequest {
            container_id: CONTAINER_ID.clone(),
            exec_id: CONTAINER_ID.clone(),
            process: Some(fuzz_proto::FuzzProcess {
                args: String::from_utf8_lossy(data).to_string().split_whitespace().map(|s| s.to_string()).collect(),
                ..Default::default()
            }),
            ..Default::default()
        }.into()).await?;

        Ok::<(), TtrpcError>(())
    }) {
        Ok(_) => Corpus::Keep,
        _ => Corpus::Reject,
    }

});
