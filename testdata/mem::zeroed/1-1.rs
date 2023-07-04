/*
From: https://github.com/firecracker-microvm/firecracker/blob/d9cec899f85b278a12f21946f5faf531f85c4798/src/vmm/src/io_uring/restriction.rs#L32
*/

fn from(restriction: &Restriction) -> Self {
        use Restriction::*;

        // SAFETY: Safe because it only contains integer values.
        let mut instance: Self = unsafe { std::mem::zeroed() };

        match restriction {
            AllowOpCode(opcode) => {
                instance.opcode = bindings::IORING_RESTRICTION_SQE_OP as u16;
                instance.__bindgen_anon_1.sqe_op = *opcode as u8;
            }
            RequireFixedFds => {
                instance.opcode = bindings::IORING_RESTRICTION_SQE_FLAGS_REQUIRED as u16;
                instance.__bindgen_anon_1.sqe_flags = 1 << bindings::IOSQE_FIXED_FILE_BIT;
            }
        };

        instance
    }