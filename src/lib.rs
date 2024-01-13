//! Bindings to perfect6502.

#![no_std]

include!("./bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        unsafe {
            let state = initAndResetChip();

            assert_eq!(readA(state), 0x00);

            // Write 0x0200 to the reset vector.
            memory[0xFFFC] = 0x00;
            memory[0xFFFD] = 0x02;

            // Write LDA #$EF to 0x0200.
            memory[0x0200] = 0xA9;
            memory[0x0201] = 0xEF;

            // Execute the 9-cycle reset sequence.
            for _ in 0..9 {
                step(state);
                step(state);
            }

            assert_eq!(readPC(state), 0x0200);

            // Execute LDA #$EF.
            step(state);
            step(state);
            step(state);
            step(state);

            assert_eq!(readA(state), 0xEF);

            destroyChip(state);
        }
    }
}
