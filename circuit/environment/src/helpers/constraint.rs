// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{prelude::*, *};
use snarkvm_fields::PrimeField;

#[derive(Clone, Debug)]
pub(crate) struct Constraint<F: PrimeField>(
    pub(crate) Scope,
    pub(crate) LinearCombination<F>,
    pub(crate) LinearCombination<F>,
    pub(crate) LinearCombination<F>,
);

impl<F: PrimeField> Constraint<F> {
    /// Returns the number of gates consumed by this constraint.
    pub(crate) fn num_gates(&self) -> u64 {
        let (a, b, c) = (&self.1, &self.2, &self.3);
        1 + a.num_additions() + b.num_additions() + c.num_additions()
    }

    /// Returns `true` if the constraint is satisfied.
    pub(crate) fn is_satisfied(&self) -> bool {
        let (scope, a, b, c) = (&self.0, &self.1, &self.2, &self.3);
        let a = a.value();
        let b = b.value();
        let c = c.value();

        match a * b == c {
            true => true,
            false => {
                eprintln!("Failed constraint at {scope}:\n\t({a} * {b}) != {c}");
                false
            }
        }
    }

    /// Returns a reference to the terms `(a, b, c)`.
    pub(crate) fn to_terms(&self) -> (&LinearCombination<F>, &LinearCombination<F>, &LinearCombination<F>) {
        (&self.1, &self.2, &self.3)
    }
}

impl<F: PrimeField> Display for Constraint<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (scope, a, b, c) = (&self.0, &self.1, &self.2, &self.3);
        let a = a.value();
        let b = b.value();
        let c = c.value();

        match (a * b) == c {
            true => write!(f, "Constraint {scope}:\n\t{a} * {b} == {c}\n"),
            false => write!(f, "Constraint {scope}:\n\t{a} * {b} != {c} (Unsatisfied)\n"),
        }
    }
}
