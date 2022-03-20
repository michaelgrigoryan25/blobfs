#[cfg(not(target_env = "msvc"))]
#[global_allocator]
#[doc(hidden)]
#[used]
/// Using a custom jemalloc fork to achieve higher server-side performance
/// on supported systems.
static ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Running initialization checks and doing some logging...
    hashoo::init()?;

    Ok(())
}
