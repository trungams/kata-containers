use libfuzzer_sys::arbitrary::Arbitrary;
use protobuf::{EnumOrUnknown, MessageField};
use protocols::agent as agent_proto;
use protocols::oci as oci_proto;
use protocols::types as types_proto;
use std::collections::HashMap;

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

impl From<FuzzFSGroupChangePolicy> for types_proto::FSGroupChangePolicy {
    fn from(f: FuzzFSGroupChangePolicy) -> Self {
        match f {
            FuzzFSGroupChangePolicy::Always => Self::Always,
            FuzzFSGroupChangePolicy::OnRootMismatch => Self::OnRootMismatch,
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzStorage {
    pub driver: String,
    pub driver_options: Vec<String>,
    pub source: String,
    pub fstype: String,
    pub options: Vec<String>,
    pub mount_point: String,
    pub fs_group: FuzzFSGroup,
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
            fs_group: MessageField::some(f.fs_group.into()),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzProcess {
    pub terminal: bool,
    pub console_size: FuzzBox,
    pub user: FuzzUser,
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub cwd: String,
    pub capabilities: FuzzLinuxCapabilities,
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
            ConsoleSize: MessageField::some(f.console_size.into()),
            User: MessageField::some(f.user.into()),
            Args: f.args.clone(),
            Env: f.env.clone(),
            Cwd: f.cwd.clone(),
            Capabilities: MessageField::some(f.capabilities.into()),
            Rlimits: f.rlimits.into_iter().map(Into::into).collect(),
            NoNewPrivileges: f.no_new_privileges,
            ApparmorProfile: f.apparmor_profile.clone(),
            OOMScoreAdj: f.oom_score_adj,
            SelinuxLabel: f.selinux_label.clone(),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzLinuxResources {
    pub devices: Vec<FuzzLinuxDeviceCgroup>,
    pub memory: FuzzLinuxMemory,
    pub cpu: FuzzLinuxCPU,
    pub pids: FuzzLinuxPids,
    pub block_io: FuzzLinuxBlockIO,
    pub hugepage_limits: Vec<FuzzLinuxHugepageLimit>,
    pub network: FuzzLinuxNetwork,
}

impl From<FuzzLinuxResources> for oci_proto::LinuxResources {
    fn from(f: FuzzLinuxResources) -> Self {
        Self {
            Devices: f.devices.into_iter().map(Into::into).collect(),
            Memory: MessageField::some(f.memory.into()),
            CPU: MessageField::some(f.cpu.into()),
            Pids: MessageField::some(f.pids.into()),
            BlockIO: MessageField::some(f.block_io.into()),
            HugepageLimits: f.hugepage_limits.into_iter().map(Into::into).collect(),
            Network: MessageField::some(f.network.into()),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

impl From<FuzzErrnoRet> for oci_proto::linux_syscall::ErrnoRet {
    fn from(f: FuzzErrnoRet) -> Self {
        match f {
            FuzzErrnoRet::Errnoret(x) => Self::Errnoret(x),
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzLinux {
    pub uid_mappings: Vec<FuzzLinuxIDMapping>,
    pub gid_mappings: Vec<FuzzLinuxIDMapping>,
    pub sysctl: HashMap<String, String>,
    pub resources: FuzzLinuxResources,
    pub cgroups_path: String,
    pub namespaces: Vec<FuzzLinuxNamespace>,
    pub devices: Vec<FuzzLinuxDevice>,
    pub seccomp: FuzzLinuxSeccomp,
    pub rootfs_propagation: String,
    pub masked_paths: Vec<String>,
    pub readonly_paths: Vec<String>,
    pub mount_label: String,
    pub intel_rdt: FuzzLinuxIntelRdt,
}

impl From<FuzzLinux> for oci_proto::Linux {
    fn from(f: FuzzLinux) -> Self {
        Self {
            UIDMappings: f.uid_mappings.into_iter().map(Into::into).collect(),
            GIDMappings: f.gid_mappings.into_iter().map(Into::into).collect(),
            Sysctl: f.sysctl.clone(),
            Resources: MessageField::some(f.resources.into()),
            CgroupsPath: f.cgroups_path.clone(),
            Namespaces: f.namespaces.into_iter().map(Into::into).collect(),
            Devices: f.devices.into_iter().map(Into::into).collect(),
            Seccomp: MessageField::some(f.seccomp.into()),
            RootfsPropagation: f.rootfs_propagation.clone(),
            MaskedPaths: f.masked_paths.clone(),
            ReadonlyPaths: f.readonly_paths.clone(),
            MountLabel: f.mount_label.clone(),
            IntelRdt: MessageField::some(f.intel_rdt.into()),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzSpec {
    pub version: String,
    pub process: FuzzProcess,
    pub root: FuzzRoot,
    pub host_name: String,
    pub mounts: Vec<FuzzMount>,
    pub hooks: FuzzHooks,
    pub annotations: HashMap<String, String>,
    pub linux: FuzzLinux,
    // pub solaris: FuzzSolaris,
    // pub windows: FuzzWindows,
}

impl From<FuzzSpec> for oci_proto::Spec {
    fn from(f: FuzzSpec) -> Self {
        Self {
            Version: f.version.clone(),
            Process: MessageField::some(f.process.into()),
            Root: MessageField::some(f.root.into()),
            Hostname: f.host_name.clone(),
            Mounts: f.mounts.into_iter().map(Into::into).collect(),
            Hooks: MessageField::some(f.hooks.into()),
            Annotations: f.annotations.clone(),
            Linux: MessageField::some(f.linux.into()),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
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

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzCreateContainerRequest {
    pub container_id: String,
    pub exec_id: String,
    pub string_user: FuzzStringUser,
    pub devices: Vec<FuzzDevice>,
    pub storages: Vec<FuzzStorage>,
    pub oci: FuzzSpec,
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
            string_user: MessageField::some(f.string_user.into()),
            devices: f.devices.into_iter().map(Into::into).collect(),
            storages: f.storages.into_iter().map(Into::into).collect(),
            OCI: MessageField::some(f.oci.into()),
            sandbox_pidns: f.sandbox_pidns,
            shared_mounts: f.shared_mounts.into_iter().map(Into::into).collect(),
            stdin_port: f.stdin_port,
            stdout_port: f.stdout_port,
            stderr_port: f.stderr_port,
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzRemoveContainerRequest {
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
