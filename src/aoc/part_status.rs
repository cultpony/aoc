#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub enum PartStatus {
    /// This Part has been done and does not require solving again
    Completed,
    /// The Part has not been done and requires solving but a solution exists
    MissingWithSolution,
    /// The Part has not been solved and requires writing a solution still
    MissingWithNoSolution,
    /// The Part is not available yet (solving another part first)
    Unavailable,
    /// The Part is unlocked in the future
    Locked,
}

impl std::fmt::Display for PartStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartStatus::Completed => f.write_str("Completed"),
            PartStatus::MissingWithSolution => f.write_str("Missing"),
            PartStatus::MissingWithNoSolution => f.write_str("Missing (no solution)"),
            PartStatus::Unavailable => f.write_str("Unavailable"),
            PartStatus::Locked => f.write_str("Locked"),
        }
    }
}
