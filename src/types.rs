#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    PreRun,
    DeferringToUI,
    WorldTick,
}
