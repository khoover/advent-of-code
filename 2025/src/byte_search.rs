#[cfg(target_arch = "aarch64")]
unsafe fn find_aarch64(mut arr: &[u8], target: u8) -> Option<usize> {
    use std::arch::is_aarch64_feature_detected;

    let mut base_index = 0;

    if is_aarch64_feature_detected!("neon") {
        use core::arch::aarch64::*;

        let (chunks, remainder) = arr.as_chunks::<64>();
        unsafe {
            let target_vec = vdupq_n_u8(target);
            for (offset, chunk) in (0..arr.len()).step_by(64).zip(chunks) {
                let data = vld1q_u8_x4(chunk.as_ptr());
                let data = [data.0, data.1, data.2, data.3];
                let eq_results = data.map(|v| vceqq_u8(v, target_vec));
                for (i, v) in eq_results.into_iter().enumerate() {
                    if vmaxvq_u8(v) != 0 {
                        let inner_offset = i * 16;
                        let reinterpreted = vreinterpretq_u64_u8(v);
                        let nonzero_byte_index =
                            vgetq_lane_u64(reinterpreted, 0).trailing_zeros() / 8;
                        if nonzero_byte_index != 8 {
                            return Some(offset + inner_offset + nonzero_byte_index as usize);
                        }
                        let nonzero_byte_index =
                            vgetq_lane_u64(reinterpreted, 1).trailing_zeros() / 8;
                        debug_assert!(nonzero_byte_index < 8);
                        return Some(offset + inner_offset + 8 + nonzero_byte_index as usize);
                    }
                }
            }
        }

        base_index = chunks.len() * 64;
        arr = remainder;
    }

    arr.iter()
        .position(|x| *x == target)
        .map(|idx| idx + base_index)
}

#[cfg(target_arch = "x86_64")]
unsafe fn find_x86(mut arr: &[u8], target: u8) -> Option<usize> {
    use std::arch::is_x86_feature_detected;

    let mut base_index = 0;

    if is_x86_feature_detcted!("sse2") {
        use core::arch::x86_64::*;

        let (chunks, remainder) = arr.as_chunks::<64>();

        unsafe {
            let target_vec = _mm_set1_epi8(target as i8);
            for (offset, chunk) in (0..arr.len()).step_by(64).zip(chunks) {
                let data: [__m128i; 4] = std::array::from_fn(|i| {
                    _mm_loadu_si128(&chunk[i * 16] as *const u8 as *const __m128i)
                });
                let eq_results = data.map(|v| _mm_cmpeq_epi8(target_vec, v));
                let bitsets = eq_results.map(|v| _mm_movemask_epi8(v));
                for (i, bitset) in bitsets.into_iter().enumerate() {
                    if bitset != 0 {
                        return Some(offset + i * 16 + bitset.trailing_zeros() as usize);
                    }
                }
            }
        }

        base_index = chunks.len() * 64;
        arr = remainder;
    }
    arr.iter()
        .position(|x| *x == target)
        .map(|idx| idx + base_index)
}

pub fn find(arr: &[u8], target: u8) -> Option<usize> {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        return find_aarch64(arr, target);
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        return find_x86(arr, target);
    }

    arr.iter().position(|x| *x == target)
}
