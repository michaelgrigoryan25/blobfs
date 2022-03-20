use clap::StructOpt;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
#[used]
static ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
#[allow(named_asm_labels)]
async fn main() -> anyhow::Result<()> {
    let args = hashoo::Args::parse();

    std::fs::metadata(&args.config).expect("configuration path is invalid");
    std::fs::metadata(&args.destination).expect("upload destination is invalid");

    Ok(())
}
