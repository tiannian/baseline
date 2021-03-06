use baseline::Context;

#[cfg(all(
    feature = "vsdb-backend",
    feature = "sha3-backend",
    feature = "smol-backend"
))]
pub type VsDBWithSha3Context<T> = Context<
    bs3vsdb::VsDbBackend,
    sha3::Sha3_256,
    crate::typedef::SmolAsyncRuntime,
    T,
>;
