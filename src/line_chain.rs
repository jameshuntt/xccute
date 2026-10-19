//! Declared lines run one after another, each read by its exit-status policy,
//! with steps that hold a resource while their lines run.
//!
//! A [`LineChain`] is the chain the fleet walks: each line an argv
//! ([`RuntimeOperation`], from a command's preview) with the
//! [`ExitStatusPolicy`] its status is decided by, walked from the first by the
//! plan's transitions (Continue to the next step, Stop, JumpTo a named step).
//! A step can also be a [`Hold`]: its resource taken, its own lines walked
//! while it is held, its rollback walked when one of them fails, the resource
//! let go after. What each line ended with, and what was decided, is kept.

use std::process::Command;

use crate::contract::{CommandPreview, CompositeShellCommand};
use crate::hold::Hold;
use crate::runtime::{ExitDisposition, ExitStatusDecision, ExitStatusPolicy, RuntimeExitStatus, RuntimeOperation, RuntimeOperationPlan};

/// One step: a line and how its status is read, or a hold around lines.
#[derive(Debug, Clone)]
pub enum LineStep {
    Line {
        operation: RuntimeOperation,
        policy: ExitStatusPolicy,
        /// The codes the line accepts (its `ok_codes`); 0 for a bare preview.
        ok_codes: Vec<i32>,
    },
    Hold(Hold),
}

impl LineStep {
    pub fn id(&self) -> &str {
        match self {
            LineStep::Line { operation, .. } => &operation.logical_id,
            LineStep::Hold(hold) => &hold.id,
        }
    }
}

/// One step walked: its name (a hold's lines as `hold/line`, its rollback as
/// `hold/rollback/line`), what it ended with (`None`: a signal, a line that
/// never started, or a hold, which has no status of its own), and what was
/// decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Walked {
    pub id: String,
    pub code: Option<i32>,
    pub decision: ExitStatusDecision,
}

/// Lines and holds, walked in order by their policies.
#[derive(Debug, Clone)]
pub struct LineChain {
    pub id: String,
    steps: Vec<LineStep>,
}

impl LineChain {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), steps: Vec::new() }
    }

    /// The chain with `line` next, named `id`, its status read by `policy`;
    /// 0 is the code it accepts.
    pub fn then(mut self, id: &str, line: &CommandPreview, policy: ExitStatusPolicy) -> Self {
        self.steps.push(LineStep::Line { operation: RuntimeOperation::from_preview(id, line), policy, ok_codes: vec![0] });
        self
    }

    /// The chain with `command` next, named `id`: its preview, the codes it
    /// accepts and the statuses it names as its policy
    /// ([`ExitStatusPolicy::of`]).
    pub fn then_command<C: CompositeShellCommand + ?Sized>(mut self, id: &str, command: &C) -> Self {
        self.steps.push(LineStep::Line {
            operation: RuntimeOperation::from_preview(id, &command.preview()),
            policy: ExitStatusPolicy::of(command),
            ok_codes: command.ok_codes().to_vec(),
        });
        self
    }

    /// The chain with `hold` next: its resource held while its lines run.
    pub fn hold(mut self, hold: Hold) -> Self {
        self.steps.push(LineStep::Hold(hold));
        self
    }

    pub fn steps(&self) -> &[LineStep] {
        &self.steps
    }

    /// Every line the chain can run, in order, a hold's lines and its rollback
    /// named as they are walked: the plan its receipts are digests of.
    pub fn plan(&self) -> RuntimeOperationPlan {
        let mut plan = RuntimeOperationPlan::new(self.id.clone());
        for (id, operation) in self.lines("") {
            plan = plan.then(RuntimeOperation { logical_id: id, ..operation.clone() });
        }
        plan
    }

    /// Each step's dry run, in order: its name and the line as it runs; a hold
    /// as `hold RESOURCE for 60 s`, then its lines and its rollback.
    pub fn shown(&self) -> Vec<(String, String)> {
        let mut shown = Vec::new();
        self.show("", &mut shown);
        shown
    }

    fn show(&self, prefix: &str, out: &mut Vec<(String, String)>) {
        for step in &self.steps {
            match step {
                LineStep::Line { operation, .. } => out.push((format!("{prefix}{}", operation.logical_id), operation.display.clone())),
                LineStep::Hold(hold) => {
                    let (timeout, _) = hold.wait();
                    out.push((format!("{prefix}{}", hold.id), format!("hold {} for {}", hold.resource().name(), crate::hold::how_long(timeout))));
                    hold.lines().show(&format!("{prefix}{}/", hold.id), out);
                    if let Some(rollback) = hold.rollback_lines() {
                        rollback.show(&format!("{prefix}{}/rollback/", hold.id), out);
                    }
                }
            }
        }
    }

    fn lines(&self, prefix: &str) -> Vec<(String, &RuntimeOperation)> {
        let mut out = Vec::new();
        for step in &self.steps {
            match step {
                LineStep::Line { operation, .. } => out.push((format!("{prefix}{}", operation.logical_id), operation)),
                LineStep::Hold(hold) => {
                    out.extend(hold.lines().lines(&format!("{prefix}{}/", hold.id)));
                    if let Some(rollback) = hold.rollback_lines() {
                        out.extend(rollback.lines(&format!("{prefix}{}/rollback/", hold.id)));
                    }
                }
            }
        }
        out
    }

    /// The chain walked from its first step: `run` runs a line and answers its
    /// exit status (`None` for a signal or a line that never started), its
    /// policy decides, and the decision names the next step. Every step
    /// walked, in order.
    pub fn walk(&self, mut run: impl FnMut(&RuntimeOperation) -> Option<i32>) -> Result<Vec<Walked>, String> {
        self.walk_with(&mut run)
    }

    /// The chain walked, each line run as its own process ([`run_line`]).
    pub fn run(&self) -> Result<Vec<Walked>, String> {
        self.walk(run_line)
    }

    /// [`LineChain::walk`] with the runner borrowed, as a hold walks its lines.
    pub fn walk_with(&self, run: &mut dyn FnMut(&RuntimeOperation) -> Option<i32>) -> Result<Vec<Walked>, String> {
        let mut walked = Vec::new();
        let mut steps_walked = 0usize;
        let mut next = if self.steps.is_empty() { None } else { Some(0) };
        while let Some(index) = next {
            steps_walked += 1;
            if steps_walked > self.steps.len() * 4 {
                return Err(format!("{}: the chain walked {} steps and did not end", self.id, steps_walked - 1));
            }
            let step = &self.steps[index];
            let decision = match step {
                LineStep::Line { operation, policy, ok_codes } => {
                    let code = run(operation);
                    let decision = policy.decide(RuntimeExitStatus::new(code, code.is_some_and(|c| ok_codes.contains(&c))));
                    walked.push(Walked { id: operation.logical_id.clone(), code, decision: decision.clone() });
                    decision
                }
                LineStep::Hold(hold) => {
                    let held = hold.walk(run)?;
                    let named = |prefix: String, w: Walked| Walked { id: format!("{prefix}{}", w.id), ..w };
                    walked.extend(held.walked.iter().cloned().map(|w| named(format!("{}/", hold.id), w)));
                    walked.extend(held.rolled_back.iter().cloned().map(|w| named(format!("{}/rollback/", hold.id), w)));
                    let decision = hold.decision(&held);
                    walked.push(Walked { id: hold.id.clone(), code: None, decision: decision.clone() });
                    decision
                }
            };
            next = match &decision.disposition {
                ExitDisposition::Continue => (index + 1 < self.steps.len()).then_some(index + 1),
                ExitDisposition::Stop => None,
                ExitDisposition::JumpTo(target) => Some(
                    self.steps
                        .iter()
                        .position(|step| step.id() == target)
                        .ok_or_else(|| format!("{}: no step {target} to jump to", self.id))?,
                ),
            };
        }
        Ok(walked)
    }
}

/// A line run as its own process, argv only (`Command::new(program).args(argv)`):
/// its exit code, `None` for a signal or a line that could not start.
pub fn run_line(operation: &RuntimeOperation) -> Option<i32> {
    Command::new(&operation.program).args(&operation.argv).status().ok().and_then(|status| status.code())
}
