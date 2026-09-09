use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StateId(usize);

impl StateId {
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransitionId(usize);

impl TransitionId {
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transition {
    pub id: TransitionId,
    pub from: StateId,
    pub to: StateId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionGraph {
    state_names: Vec<String>,
    transitions: Vec<Transition>,
    outgoing: Vec<Vec<TransitionId>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionGraphError {
    NoStates,
    EmptyStateName {
        state: usize,
    },
    DuplicateStateName {
        state: usize,
    },
    StateOutsideGraph {
        transition: usize,
        state: usize,
    },
    SelfTransition {
        transition: usize,
        state: usize,
    },
    DuplicateTransition {
        transition: usize,
        from: usize,
        to: usize,
    },
}

impl TransitionGraph {
    /// Builds the `fmsm` transition matrix in transition-vector order.
    ///
    /// Transition IDs are deliberately generated from the vector position.
    /// Gapped, duplicated, or mismatched IDs therefore cannot be represented.
    pub fn new(
        state_names: Vec<String>,
        transitions: Vec<(usize, usize)>,
    ) -> Result<Self, TransitionGraphError> {
        if state_names.is_empty() {
            return Err(TransitionGraphError::NoStates);
        }
        let mut names = BTreeSet::new();
        for (state, name) in state_names.iter().enumerate() {
            if name.trim().is_empty() {
                return Err(TransitionGraphError::EmptyStateName { state });
            }
            if !names.insert(name.clone()) {
                return Err(TransitionGraphError::DuplicateStateName { state });
            }
        }

        let mut pairs = BTreeSet::new();
        let mut outgoing = vec![Vec::new(); state_names.len()];
        let mut validated = Vec::with_capacity(transitions.len());
        for (index, (from, to)) in transitions.into_iter().enumerate() {
            if from >= state_names.len() {
                return Err(TransitionGraphError::StateOutsideGraph {
                    transition: index,
                    state: from,
                });
            }
            if to >= state_names.len() {
                return Err(TransitionGraphError::StateOutsideGraph {
                    transition: index,
                    state: to,
                });
            }
            if from == to {
                return Err(TransitionGraphError::SelfTransition {
                    transition: index,
                    state: from,
                });
            }
            if !pairs.insert((from, to)) {
                return Err(TransitionGraphError::DuplicateTransition {
                    transition: index,
                    from,
                    to,
                });
            }
            let id = TransitionId(index);
            outgoing[from].push(id);
            validated.push(Transition {
                id,
                from: StateId(from),
                to: StateId(to),
            });
        }
        Ok(Self {
            state_names,
            transitions: validated,
            outgoing,
        })
    }

    pub fn state_count(&self) -> usize {
        self.state_names.len()
    }

    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    pub fn states(&self) -> impl Iterator<Item = StateId> + '_ {
        (0..self.state_count()).map(StateId)
    }

    pub fn state_name(&self, state: StateId) -> &str {
        &self.state_names[state.0]
    }

    pub fn state_by_name(&self, name: &str) -> Option<StateId> {
        self.state_names
            .iter()
            .position(|value| value == name)
            .map(StateId)
    }

    pub fn absorbing_states(&self) -> Vec<StateId> {
        self.states()
            .filter(|state| self.is_absorbing(*state))
            .collect()
    }

    pub fn transient_states(&self) -> Vec<StateId> {
        self.states()
            .filter(|state| !self.is_absorbing(*state))
            .collect()
    }

    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    pub fn outgoing(&self, state: StateId) -> &[TransitionId] {
        &self.outgoing[state.0]
    }

    pub fn is_absorbing(&self, state: StateId) -> bool {
        self.outgoing[state.0].is_empty()
    }

    pub fn state(&self, index: usize) -> Option<StateId> {
        (index < self.state_count()).then_some(StateId(index))
    }

    pub(crate) fn transition(&self, id: TransitionId) -> &Transition {
        &self.transitions[id.0]
    }
}
