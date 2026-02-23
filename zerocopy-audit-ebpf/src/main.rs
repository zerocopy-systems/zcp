#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{map, kprobe, tracepoint},
    maps::{HashMap, PerfEventArray},
    programs::{ProbeContext, TracePointContext},
    helpers::{bpf_get_current_pid_tgid, bpf_ktime_get_ns},
};
use zerocopy_audit_common::LatencyEvent;

#[map]
static TARGET_PID: HashMap<u32, u32> = HashMap::with_max_entries(1, 0);

#[map]
static EVENTS: PerfEventArray<LatencyEvent> = PerfEventArray::with_max_entries(1024, 0);

#[map]
static START_TIMES: HashMap<u32, LatencyEvent> = HashMap::with_max_entries(1024, 0);

#[tracepoint]
pub fn audit_net_rx(_ctx: TracePointContext) -> u32 {
    let _pid = bpf_get_current_pid_tgid() as u32;
    let _time = bpf_ktime_get_ns();
    0
}

#[tracepoint]
pub fn audit_sched_wakeup(ctx: TracePointContext) -> u32 {
    let pid = unsafe { core::ptr::read_unaligned((ctx.as_ptr() as *const u8).add(16) as *const u32) };
    
    if unsafe { TARGET_PID.get(&pid).is_some() } {
        let time = unsafe { bpf_ktime_get_ns() };
        let mut event = LatencyEvent {
            pid,
            t1_net_rx: 0,
            t2_sched_wakeup: time,
            t3_sched_switch: 0,
            t4_tcp_recvmsg: 0,
        };
        let _ = unsafe { START_TIMES.insert(&pid, &event, 0) };
    }
    0
}

#[tracepoint]
pub fn audit_sched_switch(ctx: TracePointContext) -> u32 {
    let next_pid = unsafe { core::ptr::read_unaligned((ctx.as_ptr() as *const u8).add(40) as *const u32) };
    
    if unsafe { TARGET_PID.get(&next_pid).is_some() } {
        if let Some(mut event) = unsafe { START_TIMES.get(&next_pid) }.copied() {
            event.t3_sched_switch = unsafe { bpf_ktime_get_ns() };
            let _ = unsafe { START_TIMES.insert(&next_pid, &event, 0) };
        }
    }
    0
}

#[kprobe]
pub fn audit_tcp_recvmsg(ctx: ProbeContext) -> u32 {
    let pid = unsafe { bpf_get_current_pid_tgid() as u32 };
    
    if unsafe { TARGET_PID.get(&pid).is_some() } {
        if let Some(mut event) = unsafe { START_TIMES.get(&pid) }.copied() {
            event.t4_tcp_recvmsg = unsafe { bpf_ktime_get_ns() };
            unsafe { EVENTS.output(&ctx, &event, 0) };
            let _ = unsafe { START_TIMES.remove(&pid) };
        }
    }
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
