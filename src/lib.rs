//! Bindings to perfect6502.

#![no_std]

include!("./bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lda_imm() {
        unsafe {
            let state = initAndResetChip();
            assert!(!state.is_null());

            // Write 0x0200 to the reset vector.
            memory[0xFFFC] = 0x00;
            memory[0xFFFD] = 0x02;

            // Write LDA #$EF to 0x0200.
            memory[0x0200] = 0xA9;
            memory[0x0201] = 0xEF;

            let sp = readSP(state);

            // Execute the 9-cycle reset sequence.
            for _ in 0..9 {
                step(state);
                step(state);
            }

            // The stack pointer is decremented three times during the reset
            // sequence.
            assert_eq!(readSP(state), sp - 3);

            // PC is at the reset vector.
            assert_eq!(readPC(state), 0x0200);

            let x = readX(state);
            let y = readY(state);

            // Execute the first cycle of LDA #$EF.
            step(state);
            step(state);

            assert_eq!(readIR(state), 0xA9);
            assert_eq!(readRW(state), 1);
            assert_eq!(readAddressBus(state), 0x201);
            assert_eq!(readDataBus(state), 0xEF);
            
            // Execute the second cycle of LDA #$EF.
            step(state);
            step(state);

            assert_eq!(readA(state), 0xEF);

            // X and Y are unchanged.
            assert_eq!(readX(state), x);
            assert_eq!(readY(state), y);

            // The Zero flag is clear.
            assert_eq!(readP(state) & 0x02, 0);

            // There was 22 half cycles, i.e., we called step 22 times.
            assert_eq!(cycle, 22);

            destroyChip(state);
        }
    }
}
