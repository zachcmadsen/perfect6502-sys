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

            // Write 0x0200 to the reset vector.
            memory[0xfffc] = 0x00;
            memory[0xfffd] = 0x02;

            // Write lda #$ef to 0x0200.
            memory[0x0200] = 0xa9;
            memory[0x0201] = 0xef;

            // Execute the 9-cycle reset sequence.
            for _ in 0..9 {
                step(state);
                step(state);
            }

            assert_eq!(readPC(state), 0x0200);

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
