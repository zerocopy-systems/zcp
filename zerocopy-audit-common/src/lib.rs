#![no_std]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LatencyEvent {
    pub pid: u32,
    pub t1_net_rx: u64,
    pub t2_sched_wakeup: u64,
    pub t3_sched_switch: u64,
    pub t4_tcp_recvmsg: u64,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for LatencyEvent {}
