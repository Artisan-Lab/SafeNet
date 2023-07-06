fn to_raw_return(self) -> Self::RawReturn {
    let raw = self.into_raw();

    // Transmute necessary because rustc can't figure out that the
    // types are the same...
    unsafe { std::mem::transmute_copy(&raw) }
}

// https://github.com/Diggsey/rnet/blob/0194780e9cfd46ad095b694736f6eeedb8ab3cda/rnet/src/to_net.rs#L60