//! Bindings to perfect6502.

#![no_std]

include!("./bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_and_lda() {
        unsafe {
            let state = initAndResetChip();

            assert_eq!(readA(state), 0x00);

            // Write 0x3000 to the reset vector.
            memory[0xfffc] = 0x00;
            memory[0xfffd] = 0x30;

            // Write lda #$ef to 0x3000.
            memory[0x3000] = 0xa9;
            memory[0x3001] = 0xef;

            // Execute the 9-cycle reset sequence.
            for _ in 0..9 {
                step(state);
                step(state);
            }

            assert_eq!(readPC(state), 0x3000);

            // Execute lda #$ef.
            step(state);
            step(state);
            step(state);
            step(state);

            assert_eq!(readA(state), 0xef);

            destroyChip(state);
        }
    }
}
