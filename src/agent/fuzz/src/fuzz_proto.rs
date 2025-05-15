use crate::rpc_util;
use libfuzzer_sys::arbitrary::Arbitrary;
use protobuf::{EnumOrUnknown, MessageField};
use protocols::agent as agent_proto;
use protocols::oci as oci_proto;
use protocols::types as types_proto;
use std::collections::HashMap;

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzStringUser {
    pub uid: String,
    pub gid: String,
    pub additional_gids: Vec<String>,
}

impl From<FuzzStringUser> for agent_proto::StringUser {
    fn from(f: FuzzStringUser) -> Self {
        Self {
            uid: f.uid.clone(),
            gid: f.gid.clone(),
            additionalGids: f.additional_gids.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzDevice {
    pub id: String,
    pub type_: String,
    pub vm_path: String,
    pub container_path: String,
    pub options: Vec<String>,
}

impl From<FuzzDevice> for agent_proto::Device {
    fn from(f: FuzzDevice) -> Self {
        Self {
            id: f.id.clone(),
            type_: f.type_.clone(),
            vm_path: f.vm_path.clone(),
            container_path: f.container_path.clone(),
            options: f.options.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
pub enum FuzzFSGroupChangePolicy {
    Always = 0,
    OnRootMismatch = 1,
}

impl Default for FuzzFSGroupChangePolicy {
    fn default() -> Self {
        FuzzFSGroupChangePolicy::Always
    }
}

impl From<FuzzFSGroupChangePolicy> for types_proto::FSGroupChangePolicy {
    fn from(f: FuzzFSGroupChangePolicy) -> Self {
        match f {
            FuzzFSGroupChangePolicy::Always => Self::Always,
            FuzzFSGroupChangePolicy::OnRootMismatch => Self::OnRootMismatch,
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzFSGroup {
    pub group_id: u32,
    pub group_change_policy: FuzzFSGroupChangePolicy,
}

impl From<FuzzFSGroup> for agent_proto::FSGroup {
    fn from(f: FuzzFSGroup) -> Self {
        Self {
            group_id: f.group_id,
            group_change_policy: EnumOrUnknown::new(f.group_change_policy.into()),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzStorage {
    pub driver: String,
    pub driver_options: Vec<String>,
    pub source: String,
    pub fstype: String,
    pub options: Vec<String>,
    pub mount_point: String,
    pub fs_group: Option<FuzzFSGroup>,
}

impl From<FuzzStorage> for agent_proto::Storage {
    fn from(f: FuzzStorage) -> Self {
        Self {
            driver: f.driver.clone(),
            driver_options: f.driver_options.clone(),
            source: f.source.clone(),
            fstype: f.fstype.clone(),
            options: f.options.clone(),
            mount_point: f.mount_point.clone(),
            fs_group: MessageField::from_option(f.fs_group.map(Into::into)),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzBox {
    pub height: u32,
    pub width: u32,
}

impl From<FuzzBox> for oci_proto::Box {
    fn from(f: FuzzBox) -> Self {
        Self {
            Height: f.height,
            Width: f.width,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzUser {
    pub uid: u32,
    pub gid: u32,
    pub additional_gids: Vec<u32>,
    pub username: String,
}

impl From<FuzzUser> for oci_proto::User {
    fn from(f: FuzzUser) -> Self {
        Self {
            UID: f.uid,
            GID: f.gid,
            AdditionalGids: f.additional_gids.clone(),
            Username: f.username.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxCapabilities {
    pub bounding: Vec<String>,
    pub effective: Vec<String>,
    pub inheritable: Vec<String>,
    pub permitted: Vec<String>,
    pub ambient: Vec<String>,
}

impl From<FuzzLinuxCapabilities> for oci_proto::LinuxCapabilities {
    fn from(f: FuzzLinuxCapabilities) -> Self {
        Self {
            Bounding: f.bounding.clone(),
            Effective: f.effective.clone(),
            Inheritable: f.inheritable.clone(),
            Permitted: f.permitted.clone(),
            Ambient: f.ambient.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzPOSIXRlimit {
    pub type_: String,
    pub hard: u64,
    pub soft: u64,
}

impl From<FuzzPOSIXRlimit> for oci_proto::POSIXRlimit {
    fn from(f: FuzzPOSIXRlimit) -> Self {
        Self {
            Type: f.type_.clone(),
            Hard: f.hard,
            Soft: f.soft,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzProcess {
    pub terminal: bool,
    pub console_size: Option<FuzzBox>,
    pub user: Option<FuzzUser>,
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub cwd: String,
    pub capabilities: Option<FuzzLinuxCapabilities>,
    pub rlimits: Vec<FuzzPOSIXRlimit>,
    pub no_new_privileges: bool,
    pub apparmor_profile: String,
    pub oom_score_adj: i64,
    pub selinux_label: String,
}

impl From<FuzzProcess> for oci_proto::Process {
    fn from(f: FuzzProcess) -> Self {
        Self {
            Terminal: f.terminal,
            ConsoleSize: MessageField::from_option(f.console_size.map(Into::into)),
            User: MessageField::from_option(f.user.map(Into::into)),
            Args: f.args.clone(),
            Env: f.env.clone(),
            Cwd: f.cwd.clone(),
            Capabilities: MessageField::from_option(f.capabilities.map(Into::into)),
            Rlimits: f.rlimits.into_iter().map(Into::into).collect(),
            NoNewPrivileges: f.no_new_privileges,
            ApparmorProfile: f.apparmor_profile.clone(),
            OOMScoreAdj: f.oom_score_adj,
            SelinuxLabel: f.selinux_label.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzRoot {
    pub path: String,
    pub readonly: bool,
}

impl From<FuzzRoot> for oci_proto::Root {
    fn from(f: FuzzRoot) -> Self {
        Self {
            Path: f.path.clone(),
            Readonly: f.readonly,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzMount {
    pub destination: String,
    pub source: String,
    pub type_: String,
    pub options: Vec<String>,
}

impl From<FuzzMount> for oci_proto::Mount {
    fn from(f: FuzzMount) -> Self {
        Self {
            destination: f.destination.clone(),
            source: f.source.clone(),
            type_: f.type_.clone(),
            options: f.options.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzHook {
    pub path: String,
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub timeout: i64,
}

impl From<FuzzHook> for oci_proto::Hook {
    fn from(f: FuzzHook) -> Self {
        Self {
            Path: f.path.clone(),
            Args: f.args.clone(),
            Env: f.env.clone(),
            Timeout: f.timeout,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzHooks {
    pub pre_start: Vec<FuzzHook>,
    pub post_start: Vec<FuzzHook>,
    pub post_stop: Vec<FuzzHook>,
    pub create_runtime: Vec<FuzzHook>,
    pub create_container: Vec<FuzzHook>,
    pub start_container: Vec<FuzzHook>,
}

impl From<FuzzHooks> for oci_proto::Hooks {
    fn from(f: FuzzHooks) -> Self {
        Self {
            Prestart: f.pre_start.into_iter().map(Into::into).collect(),
            Poststart: f.post_start.into_iter().map(Into::into).collect(),
            Poststop: f.post_stop.into_iter().map(Into::into).collect(),
            CreateRuntime: f.create_runtime.into_iter().map(Into::into).collect(),
            CreateContainer: f.create_container.into_iter().map(Into::into).collect(),
            StartContainer: f.start_container.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxIDMapping {
    pub host_id: u32,
    pub container_id: u32,
    pub size: u32,
}

impl From<FuzzLinuxIDMapping> for oci_proto::LinuxIDMapping {
    fn from(f: FuzzLinuxIDMapping) -> Self {
        Self {
            HostID: f.host_id,
            ContainerID: f.container_id,
            Size: f.size,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxDeviceCgroup {
    pub allow: bool,
    pub type_: String,
    pub major: i64,
    pub minor: i64,
    pub access: String,
}

impl From<FuzzLinuxDeviceCgroup> for oci_proto::LinuxDeviceCgroup {
    fn from(f: FuzzLinuxDeviceCgroup) -> Self {
        Self {
            Allow: f.allow,
            Type: f.type_.clone(),
            Major: f.major,
            Minor: f.minor,
            Access: f.access.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxMemory {
    pub limit: i64,
    pub reservation: i64,
    pub swap: i64,
    pub kernel: i64,
    pub kernel_tcp: i64,
    pub swappiness: u64,
    pub disable_oom_killer: bool,
}

impl From<FuzzLinuxMemory> for oci_proto::LinuxMemory {
    fn from(f: FuzzLinuxMemory) -> Self {
        Self {
            Limit: f.limit,
            Reservation: f.reservation,
            Swap: f.swap,
            Kernel: f.kernel,
            KernelTCP: f.kernel_tcp,
            Swappiness: f.swappiness,
            DisableOOMKiller: f.disable_oom_killer,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxCPU {
    pub shares: u64,
    pub quota: i64,
    pub period: u64,
    pub realtime_runtime: i64,
    pub realtime_period: u64,
    pub cpus: String,
    pub mems: String,
}

impl From<FuzzLinuxCPU> for oci_proto::LinuxCPU {
    fn from(f: FuzzLinuxCPU) -> Self {
        Self {
            Shares: f.shares,
            Quota: f.quota,
            Period: f.period,
            RealtimeRuntime: f.realtime_runtime,
            RealtimePeriod: f.realtime_period,
            Cpus: f.cpus.clone(),
            Mems: f.mems.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxPids {
    pub limit: i64,
}

impl From<FuzzLinuxPids> for oci_proto::LinuxPids {
    fn from(f: FuzzLinuxPids) -> Self {
        Self {
            Limit: f.limit,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxWeightDevice {
    pub major: i64,
    pub minor: i64,
    pub weight: u32,
    pub leaf_weight: u32,
}

impl From<FuzzLinuxWeightDevice> for oci_proto::LinuxWeightDevice {
    fn from(f: FuzzLinuxWeightDevice) -> Self {
        Self {
            Major: f.major,
            Minor: f.minor,
            Weight: f.weight,
            LeafWeight: f.leaf_weight,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxThrottleDevice {
    pub major: i64,
    pub minor: i64,
    pub rate: u64,
}

impl From<FuzzLinuxThrottleDevice> for oci_proto::LinuxThrottleDevice {
    fn from(f: FuzzLinuxThrottleDevice) -> Self {
        Self {
            Major: f.major,
            Minor: f.minor,
            Rate: f.rate,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxBlockIO {
    pub weight: u32,
    pub leaf_weight: u32,
    pub weight_device: Vec<FuzzLinuxWeightDevice>,
    pub throttle_read_bps_device: Vec<FuzzLinuxThrottleDevice>,
    pub throttle_write_bps_device: Vec<FuzzLinuxThrottleDevice>,
    pub throttle_read_iops_device: Vec<FuzzLinuxThrottleDevice>,
    pub throttle_write_iops_device: Vec<FuzzLinuxThrottleDevice>,
}

impl From<FuzzLinuxBlockIO> for oci_proto::LinuxBlockIO {
    fn from(f: FuzzLinuxBlockIO) -> Self {
        Self {
            Weight: f.weight,
            LeafWeight: f.leaf_weight,
            WeightDevice: f.weight_device.into_iter().map(Into::into).collect(),
            ThrottleReadBpsDevice: f
                .throttle_read_bps_device
                .into_iter()
                .map(Into::into)
                .collect(),
            ThrottleWriteBpsDevice: f
                .throttle_write_bps_device
                .into_iter()
                .map(Into::into)
                .collect(),
            ThrottleReadIOPSDevice: f
                .throttle_read_iops_device
                .into_iter()
                .map(Into::into)
                .collect(),
            ThrottleWriteIOPSDevice: f
                .throttle_write_iops_device
                .into_iter()
                .map(Into::into)
                .collect(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxHugepageLimit {
    pub pagesize: String,
    pub limit: u64,
}

impl From<FuzzLinuxHugepageLimit> for oci_proto::LinuxHugepageLimit {
    fn from(f: FuzzLinuxHugepageLimit) -> Self {
        Self {
            Pagesize: f.pagesize.clone(),
            Limit: f.limit,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxInterfacePriority {
    pub name: String,
    pub priority: u32,
}

impl From<FuzzLinuxInterfacePriority> for oci_proto::LinuxInterfacePriority {
    fn from(f: FuzzLinuxInterfacePriority) -> Self {
        Self {
            Name: f.name.clone(),
            Priority: f.priority,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxNetwork {
    pub class_id: u32,
    pub priorities: Vec<FuzzLinuxInterfacePriority>,
}

impl From<FuzzLinuxNetwork> for oci_proto::LinuxNetwork {
    fn from(f: FuzzLinuxNetwork) -> Self {
        Self {
            ClassID: f.class_id,
            Priorities: f.priorities.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxResources {
    pub devices: Vec<FuzzLinuxDeviceCgroup>,
    pub memory: Option<FuzzLinuxMemory>,
    pub cpu: Option<FuzzLinuxCPU>,
    pub pids: Option<FuzzLinuxPids>,
    pub block_io: Option<FuzzLinuxBlockIO>,
    pub hugepage_limits: Vec<FuzzLinuxHugepageLimit>,
    pub network: Option<FuzzLinuxNetwork>,
}

impl From<FuzzLinuxResources> for oci_proto::LinuxResources {
    fn from(f: FuzzLinuxResources) -> Self {
        Self {
            Devices: f.devices.into_iter().map(Into::into).collect(),
            Memory: MessageField::from_option(f.memory.map(Into::into)),
            CPU: MessageField::from_option(f.cpu.map(Into::into)),
            Pids: MessageField::from_option(f.pids.map(Into::into)),
            BlockIO: MessageField::from_option(f.block_io.map(Into::into)),
            HugepageLimits: f.hugepage_limits.into_iter().map(Into::into).collect(),
            Network: MessageField::from_option(f.network.map(Into::into)),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxNamespace {
    pub type_: String,
    pub path: String,
}

impl From<FuzzLinuxNamespace> for oci_proto::LinuxNamespace {
    fn from(f: FuzzLinuxNamespace) -> Self {
        Self {
            Type: f.type_.clone(),
            Path: f.path.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxDevice {
    pub path: String,
    pub type_: String,
    pub major: i64,
    pub minor: i64,
    pub filemode: u32,
    pub uid: u32,
    pub gid: u32,
}

impl From<FuzzLinuxDevice> for oci_proto::LinuxDevice {
    fn from(f: FuzzLinuxDevice) -> Self {
        Self {
            Path: f.path.clone(),
            Type: f.type_.clone(),
            Major: f.major.clone(),
            Minor: f.minor.clone(),
            FileMode: f.filemode,
            UID: f.uid,
            GID: f.gid,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
pub enum FuzzErrnoRet {
    Errnoret(u32),
}

impl Default for FuzzErrnoRet {
    fn default() -> Self {
        FuzzErrnoRet::Errnoret(0)
    }
}

impl From<FuzzErrnoRet> for oci_proto::linux_syscall::ErrnoRet {
    fn from(f: FuzzErrnoRet) -> Self {
        match f {
            FuzzErrnoRet::Errnoret(x) => Self::Errnoret(x),
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxSeccompArg {
    pub index: u64,
    pub value: u64,
    pub value_two: u64,
    pub op: String,
}

impl From<FuzzLinuxSeccompArg> for oci_proto::LinuxSeccompArg {
    fn from(f: FuzzLinuxSeccompArg) -> Self {
        Self {
            Index: f.index,
            Value: f.value,
            ValueTwo: f.value_two,
            Op: f.op.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxSyscall {
    pub names: Vec<String>,
    pub action: String,
    pub args: Vec<FuzzLinuxSeccompArg>,
    pub errnoret: Option<FuzzErrnoRet>,
}

impl From<FuzzLinuxSyscall> for oci_proto::LinuxSyscall {
    fn from(f: FuzzLinuxSyscall) -> Self {
        Self {
            Names: f.names.clone(),
            Action: f.action.clone(),
            Args: f.args.into_iter().map(Into::into).collect(),
            ErrnoRet: match f.errnoret {
                Some(e) => Some(e.into()),
                None => None,
            },
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxSeccomp {
    pub default_action: String,
    pub architectures: Vec<String>,
    pub flags: Vec<String>,
    pub syscalls: Vec<FuzzLinuxSyscall>,
}

impl From<FuzzLinuxSeccomp> for oci_proto::LinuxSeccomp {
    fn from(f: FuzzLinuxSeccomp) -> Self {
        Self {
            DefaultAction: f.default_action.clone(),
            Architectures: f.architectures.clone(),
            Flags: f.flags.clone(),
            Syscalls: f.syscalls.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinuxIntelRdt {
    pub l3_cache_schema: String,
}

impl From<FuzzLinuxIntelRdt> for oci_proto::LinuxIntelRdt {
    fn from(f: FuzzLinuxIntelRdt) -> Self {
        Self {
            L3CacheSchema: f.l3_cache_schema.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzLinux {
    pub uid_mappings: Vec<FuzzLinuxIDMapping>,
    pub gid_mappings: Vec<FuzzLinuxIDMapping>,
    pub sysctl: HashMap<String, String>,
    pub resources: Option<FuzzLinuxResources>,
    pub cgroups_path: String,
    pub namespaces: Vec<FuzzLinuxNamespace>,
    pub devices: Vec<FuzzLinuxDevice>,
    pub seccomp: Option<FuzzLinuxSeccomp>,
    pub rootfs_propagation: String,
    pub masked_paths: Vec<String>,
    pub readonly_paths: Vec<String>,
    pub mount_label: String,
    pub intel_rdt: Option<FuzzLinuxIntelRdt>,
}

impl From<FuzzLinux> for oci_proto::Linux {
    fn from(f: FuzzLinux) -> Self {
        Self {
            UIDMappings: f.uid_mappings.into_iter().map(Into::into).collect(),
            GIDMappings: f.gid_mappings.into_iter().map(Into::into).collect(),
            Sysctl: f.sysctl.clone(),
            Resources: MessageField::from_option(f.resources.map(Into::into)),
            CgroupsPath: f.cgroups_path.clone(),
            Namespaces: f.namespaces.into_iter().map(Into::into).collect(),
            Devices: f.devices.into_iter().map(Into::into).collect(),
            Seccomp: MessageField::from_option(f.seccomp.map(Into::into)),
            RootfsPropagation: f.rootfs_propagation.clone(),
            MaskedPaths: f.masked_paths.clone(),
            ReadonlyPaths: f.readonly_paths.clone(),
            MountLabel: f.mount_label.clone(),
            IntelRdt: MessageField::from_option(f.intel_rdt.map(Into::into)),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzSpec {
    pub version: String,
    pub process: Option<FuzzProcess>,
    pub root: Option<FuzzRoot>,
    pub host_name: String,
    pub mounts: Vec<FuzzMount>,
    pub hooks: Option<FuzzHooks>,
    pub annotations: HashMap<String, String>,
    pub linux: Option<FuzzLinux>,
    // pub solaris: Option<FuzzSolaris>,
    // pub windows: Option<FuzzWindows>,
}

impl FuzzSpec {
    pub fn sample_spec() -> Self {
        Self {
            version: String::from("1.1.0-rc-1-test"),
            process: Some(FuzzProcess {
                terminal: false,
                user: Some(FuzzUser {
                    uid: 0,
                    gid: 0,
                    additional_gids: vec![],
                    username: String::from("root"),
                }),
                args: vec![String::from("/bin/sh")],
                env: vec![
                    String::from(
                        "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    ),
                    String::from("TERM=xterm"),
                ],
                cwd: String::from("/"),
                capabilities: Some(FuzzLinuxCapabilities {
                    bounding: vec![
                        String::from("CAP_AUDIT_WRITE"),
                        String::from("CAP_KILL"),
                        String::from("CAP_NET_BIND_SERVICE"),
                    ],
                    effective: vec![
                        String::from("CAP_AUDIT_WRITE"),
                        String::from("CAP_KILL"),
                        String::from("CAP_NET_BIND_SERVICE"),
                    ],
                    inheritable: vec![
                        String::from("CAP_AUDIT_WRITE"),
                        String::from("CAP_KILL"),
                        String::from("CAP_NET_BIND_SERVICE"),
                    ],
                    permitted: vec![
                        String::from("CAP_AUDIT_WRITE"),
                        String::from("CAP_KILL"),
                        String::from("CAP_NET_BIND_SERVICE"),
                    ],
                    ambient: vec![
                        String::from("CAP_AUDIT_WRITE"),
                        String::from("CAP_KILL"),
                        String::from("CAP_NET_BIND_SERVICE"),
                    ],
                }),
                rlimits: vec![FuzzPOSIXRlimit {
                    type_: String::from("RLIMIT_NOFILE"),
                    hard: 1024,
                    soft: 1024,
                }],
                no_new_privileges: true,
                ..Default::default()
            }),
            root: Some(FuzzRoot {
                path: String::from("rootfs"),
                readonly: true,
            }),
            mounts: vec![
                FuzzMount {
                    destination: String::from("/proc"),
                    type_: String::from("proc"),
                    source: String::from("proc"),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                    ],
                },
                FuzzMount {
                    destination: String::from("/dev"),
                    type_: String::from("tmpfs"),
                    source: String::from("tmpfs"),
                    options: vec![
                        String::from("nosuid"),
                        String::from("strictatime"),
                        String::from("mode=755"),
                        String::from("size=65536k"),
                    ],
                },
                FuzzMount {
                    destination: String::from("/dev/pts"),
                    type_: String::from("devpts"),
                    source: String::from("devpts"),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("newinstance"),
                        String::from("ptmxmode=0666"),
                        String::from("mode=0620"),
                        String::from("gid=5"),
                    ],
                },
                FuzzMount {
                    destination: String::from("/dev/shm"),
                    type_: String::from("tmpfs"),
                    source: String::from("shm"),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                        String::from("mode=1777"),
                        String::from("size=65536k"),
                    ],
                },
                FuzzMount {
                    destination: String::from("/dev/mqueue"),
                    type_: String::from("mqueue"),
                    source: String::from("mqueue"),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                    ],
                },
                FuzzMount {
                    destination: String::from("/sys"),
                    type_: String::from("sysfs"),
                    source: String::from("sysfs"),
                    options: vec![
                        String::from("nosuid"),
                        String::from("noexec"),
                        String::from("nodev"),
                        String::from("ro"),
                    ],
                },
            ],
            // missing annotations
            linux: Some(FuzzLinux {
                namespaces: vec![
                    FuzzLinuxNamespace {
                        type_: String::from("pid"),
                        ..Default::default()
                    },
                    FuzzLinuxNamespace {
                        type_: String::from("network"),
                        ..Default::default()
                    },
                    FuzzLinuxNamespace {
                        type_: String::from("ipc"),
                        ..Default::default()
                    },
                    FuzzLinuxNamespace {
                        type_: String::from("uts"),
                        ..Default::default()
                    },
                    FuzzLinuxNamespace {
                        type_: String::from("mount"),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl From<FuzzSpec> for oci_proto::Spec {
    fn from(f: FuzzSpec) -> Self {
        Self {
            Version: f.version.clone(),
            Process: MessageField::from_option(f.process.map(Into::into)),
            Root: MessageField::from_option(f.root.map(Into::into)),
            Hostname: f.host_name.clone(),
            Mounts: f.mounts.into_iter().map(Into::into).collect(),
            Hooks: MessageField::from_option(f.hooks.map(Into::into)),
            Annotations: f.annotations.clone(),
            Linux: MessageField::from_option(f.linux.map(Into::into)),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzSharedMount {
    pub name: String,
    pub src_ctr: String,
    pub src_path: String,
    pub dst_ctr: String,
    pub dst_path: String,
}

impl From<FuzzSharedMount> for agent_proto::SharedMount {
    fn from(f: FuzzSharedMount) -> Self {
        Self {
            name: f.name.clone(),
            src_ctr: f.src_ctr.clone(),
            src_path: f.src_path.clone(),
            dst_ctr: f.dst_ctr.clone(),
            dst_path: f.dst_path.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzKernelModule {
    pub name: String,
    pub parameters: Vec<String>,
}

impl From<FuzzKernelModule> for agent_proto::KernelModule {
    fn from(f: FuzzKernelModule) -> Self {
        Self {
            name: f.name.clone(),
            parameters: f.parameters.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzCreateContainerRequest {
    #[arbitrary(with = rpc_util::arbitrary_container_id)]
    pub container_id: String,
    pub exec_id: String,
    pub string_user: Option<FuzzStringUser>,
    pub devices: Vec<FuzzDevice>,
    pub storages: Vec<FuzzStorage>,
    pub oci: Option<FuzzSpec>,
    pub sandbox_pidns: bool,
    pub shared_mounts: Vec<FuzzSharedMount>,
    pub stdin_port: u32,
    pub stdout_port: u32,
    pub stderr_port: u32,
}

impl From<FuzzCreateContainerRequest> for agent_proto::CreateContainerRequest {
    fn from(f: FuzzCreateContainerRequest) -> Self {
        Self {
            container_id: f.container_id.clone(),
            exec_id: f.exec_id.clone(),
            string_user: MessageField::from_option(f.string_user.map(Into::into)),
            devices: f.devices.into_iter().map(Into::into).collect(),
            storages: f.storages.into_iter().map(Into::into).collect(),
            OCI: MessageField::from_option(f.oci.map(Into::into)),
            sandbox_pidns: f.sandbox_pidns,
            shared_mounts: f.shared_mounts.into_iter().map(Into::into).collect(),
            stdin_port: f.stdin_port,
            stdout_port: f.stdout_port,
            stderr_port: f.stderr_port,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzRemoveContainerRequest {
    #[arbitrary(with = rpc_util::arbitrary_container_id)]
    pub container_id: String,
    pub timeout: u32,
}

impl From<FuzzRemoveContainerRequest> for agent_proto::RemoveContainerRequest {
    fn from(f: FuzzRemoveContainerRequest) -> Self {
        Self {
            container_id: f.container_id.clone(),
            timeout: f.timeout,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzExecProcessRequest {
    #[arbitrary(with = rpc_util::arbitrary_container_id)]
    pub container_id: String,
    pub exec_id: String,
    pub string_user: Option<FuzzStringUser>,
    pub process: Option<FuzzProcess>,
}

impl From<FuzzExecProcessRequest> for agent_proto::ExecProcessRequest {
    fn from(f: FuzzExecProcessRequest) -> Self {
        Self {
            container_id: f.container_id.clone(),
            exec_id: f.exec_id.clone(),
            string_user: MessageField::from_option(f.string_user.map(Into::into)),
            process: MessageField::from_option(f.process.map(Into::into)),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzStartContainerRequest {
    pub container_id: String,
}

impl From<FuzzStartContainerRequest> for agent_proto::StartContainerRequest {
    fn from(f: FuzzStartContainerRequest) -> Self {
        Self {
            container_id: f.container_id.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Default, Debug, Clone)]
pub struct FuzzCreateSandboxRequest {
    pub hostname: String,
    pub dns: Vec<String>,
    pub storages: Vec<FuzzStorage>,
    pub sandbox_pidns: bool,
    pub sandbox_id: String,
    pub guest_hook_path: String,
    pub kernel_modules: Vec<FuzzKernelModule>,
}

impl From<FuzzCreateSandboxRequest> for agent_proto::CreateSandboxRequest {
    fn from(f: FuzzCreateSandboxRequest) -> Self {
        Self {
            hostname: f.hostname.clone(),
            dns: f.dns.clone(),
            storages: f.storages.into_iter().map(Into::into).collect(),
            sandbox_pidns: f.sandbox_pidns,
            sandbox_id: f.sandbox_id.clone(),
            guest_hook_path: f.guest_hook_path.clone(),
            kernel_modules: f.kernel_modules.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}
