use openvm_algebra_circuit::*;
use openvm_circuit::arch::{InitFileGenerator, SystemConfig};
use openvm_circuit_derive::VmConfig;
use openvm_ecc_circuit::*;
use openvm_rv32im_circuit::*;
use openvm_stark_backend::p3_field::PrimeField32;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, VmConfig, Serialize, Deserialize)]
pub struct Rv32PairingConfig {
    #[system]
    pub system: SystemConfig,
    #[extension]
    pub base: Rv32I,
    #[extension]
    pub mul: Rv32M,
    #[extension]
    pub io: Rv32Io,
    #[extension]
    pub modular: ModularExtension,
    #[extension]
    pub fp2: Fp2Extension,
    #[extension]
    pub weierstrass: WeierstrassExtension,
    #[extension]
    pub pairing: PairingExtension,
}

impl Rv32PairingConfig {
    pub fn new(curves: Vec<PairingCurve>, complex_struct_names: Vec<String>) -> Self {
        let modulus_primes: Vec<_> = curves
            .iter()
            .map(|c| c.curve_config().modulus.clone())
            .collect();
        let mut modulus_and_scalar_primes = modulus_primes.clone();
        modulus_and_scalar_primes.extend(curves.iter().map(|c| c.curve_config().scalar.clone()));
        Self {
            system: SystemConfig::default().with_continuations(),
            base: Default::default(),
            mul: Default::default(),
            io: Default::default(),
            modular: ModularExtension::new(modulus_and_scalar_primes),
            fp2: Fp2Extension::new(
                complex_struct_names
                    .into_iter()
                    .zip(modulus_primes)
                    .collect(),
            ),
            weierstrass: WeierstrassExtension::new(
                curves.iter().map(|c| c.curve_config()).collect(),
            ),
            pairing: PairingExtension::new(curves),
        }
    }
}

impl InitFileGenerator for Rv32PairingConfig {
    fn generate_init_file_contents(&self) -> Option<String> {
        Some(format!(
            "// This file is automatically generated by cargo openvm. Do not rename or edit.\n{}\n{}\n{}\n",
            self.modular.generate_moduli_init(),
            self.fp2.generate_complex_init(&self.modular),
            self.weierstrass.generate_sw_init()
        ))
    }
}
