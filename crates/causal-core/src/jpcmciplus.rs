//! Tigramite J-PCMCI+ constraint preparation and discovery (GPL-3.0).

use crate::parcorr::Node;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeClass {
    System,
    TimeContext,
    SpaceContext,
    TimeDummy,
    SpaceDummy,
}

impl NodeClass {
    fn dummy(self) -> bool {
        matches!(self, Self::TimeDummy | Self::SpaceDummy)
    }
    fn context(self) -> bool {
        matches!(self, Self::TimeContext | Self::SpaceContext)
    }
}

/// Ordered entries preserve Python dictionary iteration order. Marks are the
/// upstream three-character link assumptions, including their middle certainty mark.
type LinkAssumptions = Vec<Vec<(Node, [u8; 3])>>;

/// `assume_exogenous_context` followed by `clean_link_assumptions`.
/// Input shape and link validity belong to the caller's graph boundary.
fn prepare_context_links(classes: &[NodeClass], links: &LinkAssumptions) -> LinkAssumptions {
    links
        .iter()
        .enumerate()
        .map(|(j, parents)| {
            let target = classes[j];
            if target.dummy() {
                return Vec::new();
            }
            parents
                .iter()
                .filter_map(|&(node, mut mark)| {
                    let (i, lag) = node;
                    let source = classes[i];
                    if lag < 0 && (source.dummy() || source == NodeClass::SpaceContext) {
                        return None;
                    }
                    if target.context() && (source == NodeClass::System || node == (j, 0)) {
                        return None;
                    }
                    if (target == NodeClass::SpaceContext && source == NodeClass::TimeContext)
                        || (target == NodeClass::TimeContext
                            && source == NodeClass::SpaceContext
                            && lag == 0)
                    {
                        return None;
                    }
                    if target == NodeClass::System && (source.context() || source.dummy()) {
                        mark[0] = b'-';
                        mark[2] = b'>';
                    }
                    Some((node, mark))
                })
                .collect()
        })
        .collect()
}

/// `remove_dummy_link_assumptions`: remove contemporaneous dummy parents.
fn without_dummy_links(classes: &[NodeClass], links: &LinkAssumptions) -> LinkAssumptions {
    links
        .iter()
        .enumerate()
        .map(|(j, parents)| {
            parents
                .iter()
                .copied()
                .filter(|&(node, _)| classes[j].dummy() || node.1 != 0 || !classes[node.0].dummy())
                .collect()
        })
        .collect()
}

fn require(parents: &mut Vec<(Node, [u8; 3])>, node: Node) {
    if let Some((_, mark)) = parents.iter_mut().find(|(candidate, _)| *candidate == node) {
        *mark = *b"-->";
    } else {
        parents.push((node, *b"-->"));
    }
}

/// `add_found_context_link_assumptions`: freeze discovered context parents
/// before the dummy search, preserving insertion order.
fn with_context_parents(
    classes: &[NodeClass],
    links: &LinkAssumptions,
    found: &[Vec<Node>],
) -> LinkAssumptions {
    let mut out = links.clone();
    for (j, class) in classes.iter().enumerate() {
        if class.context() {
            out[j].clear();
        }
    }
    for (j, class) in classes.iter().enumerate() {
        if !class.dummy() {
            out[j].retain(|(node, _)| !classes[node.0].context());
            for &node in &found[j] {
                require(&mut out[j], node);
            }
        }
    }
    out
}

/// `clean_system_link_assumptions`: retain discovered contextual parents
/// as required links for the final system search.
fn system_links(
    classes: &[NodeClass],
    links: &LinkAssumptions,
    contexts: &[Vec<Node>],
    dummies: &[Vec<Node>],
) -> LinkAssumptions {
    let mut out = links.clone();
    for (j, class) in classes.iter().enumerate() {
        if *class == NodeClass::System {
            out[j].retain(|(node, _)| classes[node.0] == NodeClass::System);
            for &node in contexts[j].iter().chain(&dummies[j]) {
                require(&mut out[j], node);
            }
        } else {
            out[j].clear();
        }
    }
    out
}

/// Upstream input marks; conflict marks are outputs, not assumptions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assumption {
    RequiredUndirected,
    OptionalUndirected,
    RequiredForward,
    OptionalForward,
    RequiredReverse,
    OptionalReverse,
}

impl Assumption {
    pub fn parse(mark: &str) -> Result<Self, ConstraintError> {
        match mark {
            "o-o" => Ok(Self::RequiredUndirected),
            "o?o" => Ok(Self::OptionalUndirected),
            "-->" => Ok(Self::RequiredForward),
            "-?>" => Ok(Self::OptionalForward),
            "<--" => Ok(Self::RequiredReverse),
            "<?-" => Ok(Self::OptionalReverse),
            other => Err(ConstraintError::InvalidMark(other.to_owned())),
        }
    }

    fn bytes(self) -> [u8; 3] {
        *match self {
            Self::RequiredUndirected => b"o-o",
            Self::OptionalUndirected => b"o?o",
            Self::RequiredForward => b"-->",
            Self::OptionalForward => b"-?>",
            Self::RequiredReverse => b"<--",
            Self::OptionalReverse => b"<?-",
        }
    }
}

/// Untrusted transport input. `ContextSearch::new` is its validation boundary.
#[derive(Clone, Debug)]
pub struct NodeInput {
    pub class: NodeClass,
    pub parents: Vec<(Node, Assumption)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintError {
    EmptyGraph,
    LagRange(usize),
    InvalidMark(String),
    InvalidNode {
        target: usize,
        parent: Node,
    },
    InvalidLag {
        target: usize,
        parent: Node,
        max_lag: usize,
    },
    DuplicateParent {
        target: usize,
        parent: Node,
    },
    ParentRows {
        expected: usize,
        actual: usize,
    },
    WrongParentClass {
        target: usize,
        parent: Node,
        expected: ParentRole,
    },
    IneligibleParent {
        target: usize,
        parent: Node,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParentRole {
    Context,
    Dummy,
}

#[derive(Debug)]
struct Prepared {
    classes: Vec<NodeClass>,
    links: LinkAssumptions,
    max_lag: usize,
}

/// Prepared constraints for the observed-context search, not a discovery result.
///
/// Dummy parents cannot be supplied before observed-context parents:
/// ```compile_fail,E0599
/// use causal_wasm::jpcmciplus::ContextSearch;
/// fn skip_contexts(search: ContextSearch) {
///     search.with_dummies(vec![]);
/// }
/// ```
#[derive(Debug)]
pub struct ContextSearch {
    prepared: Prepared,
    links: LinkAssumptions,
}

/// Observed context parents have been supplied; dummy parents do not exist yet.
#[derive(Debug)]
pub struct DummySearch {
    prepared: Prepared,
    contexts: Vec<Vec<Node>>,
    links: LinkAssumptions,
}

/// Both context searches have supplied parents for the final system search.
/// Construction must go through the checked phase transitions:
/// ```compile_fail,E0451
/// use causal_wasm::jpcmciplus::SystemSearch;
/// let search = SystemSearch {
///     links: vec![], contexts: vec![], dummies: vec![],
/// };
/// ```
#[derive(Debug)]
pub struct SystemSearch {
    links: LinkAssumptions,
    contexts: Vec<Vec<Node>>,
    dummies: Vec<Vec<Node>>,
}

fn validate_nodes(
    nodes: &[Node],
    target: usize,
    n: usize,
    max_lag: usize,
) -> Result<(), ConstraintError> {
    let mut seen = std::collections::HashSet::new();
    for &parent in nodes {
        if parent.0 >= n {
            return Err(ConstraintError::InvalidNode { target, parent });
        }
        if parent.1 > 0 || parent.1.unsigned_abs() as usize > max_lag {
            return Err(ConstraintError::InvalidLag {
                target,
                parent,
                max_lag,
            });
        }
        if !seen.insert(parent) {
            return Err(ConstraintError::DuplicateParent { target, parent });
        }
    }
    Ok(())
}

impl Prepared {
    fn validate_parents(
        &self,
        found: &[Vec<Node>],
        role: ParentRole,
    ) -> Result<(), ConstraintError> {
        if found.len() != self.classes.len() {
            return Err(ConstraintError::ParentRows {
                expected: self.classes.len(),
                actual: found.len(),
            });
        }
        for (target, parents) in found.iter().enumerate() {
            validate_nodes(parents, target, self.classes.len(), self.max_lag)?;
            for &parent in parents {
                let class_matches = match role {
                    ParentRole::Context => self.classes[parent.0].context(),
                    ParentRole::Dummy => self.classes[parent.0].dummy(),
                };
                if !class_matches {
                    return Err(ConstraintError::WrongParentClass {
                        target,
                        parent,
                        expected: role,
                    });
                }
                if !self.links[target].iter().any(|(node, _)| *node == parent) {
                    return Err(ConstraintError::IneligibleParent { target, parent });
                }
            }
        }
        Ok(())
    }
}

impl ContextSearch {
    pub fn new(nodes: Vec<NodeInput>, max_lag: usize) -> Result<Self, ConstraintError> {
        if nodes.is_empty() {
            return Err(ConstraintError::EmptyGraph);
        }
        if max_lag > i32::MAX as usize {
            return Err(ConstraintError::LagRange(max_lag));
        }
        for (target, node) in nodes.iter().enumerate() {
            let parents: Vec<Node> = node.parents.iter().map(|(parent, _)| *parent).collect();
            validate_nodes(&parents, target, nodes.len(), max_lag)?;
        }
        let classes: Vec<_> = nodes.iter().map(|node| node.class).collect();
        let raw = nodes
            .into_iter()
            .map(|node| {
                node.parents
                    .into_iter()
                    .map(|(parent, mark)| (parent, mark.bytes()))
                    .collect()
            })
            .collect();
        let links = prepare_context_links(&classes, &raw);
        let search_links = without_dummy_links(&classes, &links);
        Ok(Self {
            prepared: Prepared {
                classes,
                links,
                max_lag,
            },
            links: search_links,
        })
    }

    pub fn links(&self) -> &[Vec<(Node, [u8; 3])>] {
        &self.links
    }

    /// Imports search output after validation; this does not run CI tests.
    pub fn with_contexts(self, found: Vec<Vec<Node>>) -> Result<DummySearch, ConstraintError> {
        self.prepared
            .validate_parents(&found, ParentRole::Context)?;
        let links = with_context_parents(&self.prepared.classes, &self.prepared.links, &found);
        Ok(DummySearch {
            prepared: self.prepared,
            contexts: found,
            links,
        })
    }
}

impl DummySearch {
    pub fn links(&self) -> &[Vec<(Node, [u8; 3])>] {
        &self.links
    }

    pub fn with_dummies(self, found: Vec<Vec<Node>>) -> Result<SystemSearch, ConstraintError> {
        self.prepared.validate_parents(&found, ParentRole::Dummy)?;
        let links = system_links(
            &self.prepared.classes,
            &self.prepared.links,
            &self.contexts,
            &found,
        );
        Ok(SystemSearch {
            links,
            contexts: self.contexts,
            dummies: found,
        })
    }
}

impl SystemSearch {
    pub fn links(&self) -> &[Vec<(Node, [u8; 3])>] {
        &self.links
    }
    pub fn context_parents(&self) -> &[Vec<Node>] {
        &self.contexts
    }
    pub fn dummy_parents(&self) -> &[Vec<Node>] {
        &self.dummies
    }
}

#[cfg(test)]
#[path = "jpcmciplus_oracle_tests.rs"]
mod oracle_tests;

mod alpha;
mod search;
pub use alpha::{
    optimize_reused_alpha, optimize_source_alpha, AlphaAssumptions, AlphaGrid, AlphaHistory,
    AlphaSelection,
};
pub use search::{
    run, run_with_assumptions, run_with_options, ColliderRule, ConditionalIndependence, Discovery,
    Fdr, FixedCi, Options, RunError, TestResult,
};
