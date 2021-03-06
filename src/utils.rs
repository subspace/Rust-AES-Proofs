use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AesImplementation {
    /// AVX-512 Vector AES instruction set
    VAes,
}

/// Inspect what special instruction sets are available for AES on this machine (software fallback
/// is always available regardless)
pub fn aes_implementations_available() -> HashSet<AesImplementation> {
    let mut implementations = HashSet::<AesImplementation>::new();

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512vaes") {
            implementations.insert(AesImplementation::VAes);
        }
    }

    return implementations;
}
