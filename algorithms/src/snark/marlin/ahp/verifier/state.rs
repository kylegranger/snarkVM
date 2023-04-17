// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use core::marker::PhantomData;

use crate::{
    fft::EvaluationDomain,
    snark::marlin::{
        ahp::verifier::{FirstMessage, SecondMessage, ThirdMessage},
        CircuitId,
        MarlinMode,
    },
};
use snarkvm_fields::PrimeField;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct CircuitSpecificState<F: PrimeField> {
    pub(crate) input_domain: EvaluationDomain<F>,
    pub(crate) constraint_domain: EvaluationDomain<F>,
    pub(crate) non_zero_a_domain: EvaluationDomain<F>,
    pub(crate) non_zero_b_domain: EvaluationDomain<F>,
    pub(crate) non_zero_c_domain: EvaluationDomain<F>,

    /// The number of instances being proved in this batch.
    pub(in crate::snark::marlin) batch_size: usize,
}
/// State of the AHP verifier.
#[derive(Debug)]
pub struct State<F: PrimeField, MM: MarlinMode> {
    pub(crate) circuit_specific_states: BTreeMap<CircuitId, CircuitSpecificState<F>>,
    pub(crate) max_constraint_domain: EvaluationDomain<F>,
    pub(crate) largest_non_zero_domain: EvaluationDomain<F>,

    pub(crate) first_round_message: Option<FirstMessage<F>>,
    pub(crate) second_round_message: Option<SecondMessage<F>>,
    pub(crate) third_round_message: Option<ThirdMessage<F>>,

    pub(crate) gamma: Option<F>,
    pub(crate) mode: PhantomData<MM>,
}
