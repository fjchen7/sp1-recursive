//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_core::{utils::BabyBearBlake3, SP1ProofWithIO, SP1Prover, SP1Stdin, SP1Verifier};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf-plain");

pub fn main() {
    let n = sp1_zkvm::io::read::<u32>();
    let ret = {
        let mut stdin = SP1Stdin::new();
        stdin.write(&n);
        let mut proof: SP1ProofWithIO<BabyBearBlake3> = {
            let path = "../../script/proof-with-io_plain.json";
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

        // Verify proof.
        SP1Verifier::verify(ELF, &proof).is_ok()
    };
    sp1_zkvm::io::write(&ret);
}
