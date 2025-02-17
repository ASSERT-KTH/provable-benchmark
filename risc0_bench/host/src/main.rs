// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    GUEST_BENCH_ELF, GUEST_BENCH_ID
};
use risc0_zkvm::{get_prover_server, ExecutorEnv, ExecutorImpl, VerifierContext, ProverOpts, Session};
use serde::Serialize;
use serde_with::{serde_as, DurationMicroSeconds};

use std::{
    //path::Path,
    time::{Instant, Duration} //For timekeeping
};

use human_repr::HumanDuration; // Crate for human representations of durations and bytesizes

use csv::Writer;

#[serde_as]
#[derive(Serialize)]
struct Records {
    #[serde_as(as = "DurationMicroSeconds")]
    exec_duration: Duration,
    #[serde_as(as = "DurationMicroSeconds")]
    prove_duration: Duration,
    #[serde_as(as = "DurationMicroSeconds")]
    verify_duration: Duration
}

impl Records{
    fn new()-> Self{
        Records {
            exec_duration: Duration::default(),
            prove_duration: Duration::default(),
            verify_duration: Duration::default()
        }
    }
}

fn main() {
    let mut record = Records::new();
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();


    // For example:
    let input: u32 = 1000;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let mut exec = ExecutorImpl::from_elf(env, GUEST_BENCH_ELF).unwrap();
    let start = Instant::now();
    let session = exec.run().unwrap();
    let elapsed = start.elapsed();
    println!("Executing with input size: {}", input.to_string());
    println!("Execution time: {}", elapsed.human_duration().to_string());
    record.exec_duration = elapsed;


    // Obtain the default prover.
    let prover = get_prover_server(&ProverOpts::composite()).unwrap();
    let ctx = VerifierContext::default();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_start = Instant::now();
    let receipt = prover
        .prove_session(&ctx, &session)
        .unwrap()
        .receipt;
    let proof_duration = prove_start.elapsed();
    println!("Proving time: {}", proof_duration.human_duration().to_string());
    record.prove_duration = proof_duration;


    let verify_start = Instant::now();
    receipt
        .verify(GUEST_BENCH_ID)
        .unwrap();
    let verify_duration = verify_start.elapsed();
    println!("Verification time: {}", verify_duration.human_duration().to_string());
    record.verify_duration = verify_duration;

    let mut wtr = csv::Writer::from_path("bench.csv").unwrap();
    wtr.serialize(&record).expect("Could not serialize");
    wtr.flush().expect("Could not flush");

}
