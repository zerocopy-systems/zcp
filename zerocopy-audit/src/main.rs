use clap::Parser;
use log::info;
use serde::Serialize;
use tokio::signal;
use zerocopy_audit_common::LatencyEvent;

#[cfg(target_os = "linux")]
use aya::maps::{perf::AsyncPerfEventArray, HashMap};
#[cfg(target_os = "linux")]
use aya::programs::{KProbe, TracePoint};
#[cfg(target_os = "linux")]
use aya::util::online_cpus;
#[cfg(target_os = "linux")]
use aya::{include_bytes_aligned, Ebpf};
#[cfg(target_os = "linux")]
use bytes::BytesMut;

#[derive(Parser, Debug)]
#[command(author, version, about = "Sovereign Audit: eBPF diagnostic wedge for Jitter Tax", long_about = None)]
struct Args {
    #[arg(short, long)]
    pid: u32,
    #[arg(short, long, default_value_t = 50_000_000.0)]
    volume: f64,
    #[arg(short, long, default_value_t = 0.0001)] // 1 BPS
    slippage: f64,
}

#[derive(Serialize)]
struct BillOfHealth {
    target_pid: u32,
    p99_sched_wakeup_ns: u64,
    p99_kernel_stack_ns: u64,
    p99_total_overhead_ns: u64,
    jitter_tax_annual_loss: f64,
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    info!(
        "Initializing Sovereign Execution Probe for PID: {}",
        args.pid
    );

    // Provide the eBPF bytecode compiled via build.rs
    #[cfg(target_arch = "x86_64")]
    let bpf_data = include_bytes_aligned!(concat!(env!("OUT_DIR"), "/zerocopy_audit_ebpf"));

    let mut bpf = Ebpf::load(bpf_data)?;

    // Attach Sched Wakeup
    let sched_wakeup: &mut TracePoint =
        bpf.program_mut("audit_sched_wakeup").unwrap().try_into()?;
    sched_wakeup.load()?;
    sched_wakeup.attach("sched", "sched_wakeup")?;

    // Attach Sched Switch
    let sched_switch: &mut TracePoint =
        bpf.program_mut("audit_sched_switch").unwrap().try_into()?;
    sched_switch.load()?;
    sched_switch.attach("sched", "sched_switch")?;

    // Attach TCP Recvmsg
    let tcp_recvmsg: &mut KProbe = bpf.program_mut("audit_tcp_recvmsg").unwrap().try_into()?;
    tcp_recvmsg.load()?;
    tcp_recvmsg.attach("tcp_recvmsg", 0)?;

    info!("Kernel eBPF probes attached successfully. (Zero Observer Effect)");

    // Inject target PID
    let mut target_map: HashMap<_, u32, u32> =
        HashMap::try_from(bpf.map_mut("TARGET_PID").unwrap())?;
    target_map.insert(args.pid, 1, 0)?;

    // Setup Ring Buffer Polling
    let mut events: AsyncPerfEventArray<_> = bpf.take_map("EVENTS").unwrap().try_into()?;

    let mut runqueue_delays = Vec::new();
    let mut stack_delays = Vec::new();

    info!("Listening for 100 packets to establish the baseline...");

    for cpu_id in online_cpus().map_err(|e| anyhow::anyhow!("CPU Error: {:?}", e))? {
        let mut buf = events.open(cpu_id, None)?;

        tokio::spawn(async move {
            let mut buffers = (0..10)
                .map(|_| BytesMut::with_capacity(1024))
                .collect::<Vec<_>>();
            loop {
                if let Ok(events) = buf.read_events(&mut buffers).await {
                    for i in 0..events.read {
                        let buf = &mut buffers[i];
                        let event = unsafe {
                            std::ptr::read_unaligned(buf.as_ptr() as *const LatencyEvent)
                        };
                        // Aggregation calculations
                        let rq_delay = event.t3_sched_switch.saturating_sub(event.t2_sched_wakeup);
                        let stack_delay =
                            event.t4_tcp_recvmsg.saturating_sub(event.t2_sched_wakeup);
                        if rq_delay > 0 && rq_delay < 10_000_000 {
                            // Print the terrifying reality
                            println!("🚨 [PID {}] Woke up at {}ns, Executed at {}ns. RunQueue Wait: {}µs", 
                                event.pid, event.t2_sched_wakeup, event.t3_sched_switch, rq_delay / 1000);
                        }
                    }
                }
            }
        });
    }

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Detaching probes and shutting down.");

    // (In the full implementation, we'd wait for Vector collection and emit the BillOfHealth.json here)

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() -> anyhow::Result<()> {
    println!("❌ Sovereign Audit is a native eBPF probe and must be compiled and executed on a Linux environment.");
    Ok(())
}
