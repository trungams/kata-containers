// Copyright (c) 2019 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

use clap::Parser;
use nix::sys::reboot::{reboot, RebootMode};
use nix::unistd::{self, sync, Pid};
use std::process::exit;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = kata_agent::AgentOpts::parse();

    if args.version {
        let extra_features = kata_agent::features::get_build_features();

        println!(
            "{} version {} (api version: {}, commit version: {}, type: rust, extra-features: {extra_features:?})",
            kata_agent::NAME,
            kata_agent::version::AGENT_VERSION,
            kata_agent::version::API_VERSION,
            kata_agent::version::VERSION_COMMIT,
        );
        exit(0);
    }

    if let Some(kata_agent::SubCommand::Init {}) = args.subcmd {
        kata_agent::reset_sigpipe();
        rustjail::container::init_child();
        exit(0);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let init_mode = unistd::getpid() == Pid::from_raw(1);
    let result = rt.block_on(kata_agent::real_main(init_mode));

    if init_mode {
        sync();
        let _ = reboot(RebootMode::RB_POWER_OFF);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result};
    use nix::fcntl::OFlag;
    use scopeguard::defer;
    use test_utils::TestUserType;
    use test_utils::{assert_result, skip_if_not_root, skip_if_root};
    use tokio::sync::watch::channel;

    #[tokio::test]
    async fn test_create_logger_task() {
        #[derive(Debug)]
        struct TestData {
            vsock_port: u32,
            test_user: TestUserType,
            result: Result<()>,
        }

        let tests = &[
            TestData {
                // non-root user cannot use privileged vsock port
                vsock_port: 1,
                test_user: TestUserType::NonRootOnly,
                result: Err(anyhow!(nix::errno::Errno::from_i32(libc::EACCES))),
            },
            TestData {
                // passing vsock_port 0 causes logger task to write to stdout
                vsock_port: 0,
                test_user: TestUserType::Any,
                result: Ok(()),
            },
        ];

        for (i, d) in tests.iter().enumerate() {
            if d.test_user == TestUserType::RootOnly {
                skip_if_not_root!();
            } else if d.test_user == TestUserType::NonRootOnly {
                skip_if_root!();
            }

            let msg = format!("test[{}]: {:?}", i, d);
            let (rfd, wfd) = unistd::pipe2(OFlag::O_CLOEXEC).unwrap();
            defer!({
                // XXX: Never try to close rfd, because it will be closed by PipeStream in
                // create_logger_task() and it's not safe to close the same fd twice time.
                unistd::close(wfd).unwrap();
            });

            let (shutdown_tx, shutdown_rx) = channel(true);

            shutdown_tx.send(true).unwrap();
            let result = kata_agent::create_logger_task(rfd, d.vsock_port, shutdown_rx).await;

            let msg = format!("{}, result: {:?}", msg, result);
            assert_result!(d.result, result, msg);
        }
    }
}
