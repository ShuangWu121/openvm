use openvm_algebra_circuit::*;
use openvm_circuit::arch::{InitFileGenerator, SystemConfig};
use openvm_circuit_derive::VmConfig;
use openvm_rv32im_circuit::*;
use openvm_stark_backend::p3_field::PrimeField32;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, VmConfig, Serialize, Deserialize)]
pub struct Rv32WeierstrassConfig {
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
    pub weierstrass: WeierstrassExtension,
}

impl Rv32WeierstrassConfig {
    pub fn new(curves: Vec<CurveConfig>) -> Self {
        let primes: Vec<_> = curves
            .iter()
            .flat_map(|c| [c.modulus.clone(), c.scalar.clone()])
            .collect();
        Self {
            system: SystemConfig::default().with_continuations(),
            base: Default::default(),
            mul: Default::default(),
            io: Default::default(),
            modular: ModularExtension::new(primes),
            weierstrass: WeierstrassExtension::new(curves),
        }
    }
}

impl InitFileGenerator for Rv32WeierstrassConfig {
    fn generate_init_file_contents(&self) -> Option<String> {
        Some(format!(
            "// This file is automatically generated by cargo openvm. Do not rename or edit.\n{}\n{}\n",
            self.modular.generate_moduli_init(),
            self.weierstrass.generate_sw_init()
        ))
    }
}
