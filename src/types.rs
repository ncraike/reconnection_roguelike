#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RunState {
    PreRun,
    DeferringToUI,
    WorldTick,
}
