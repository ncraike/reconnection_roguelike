#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum UITask {
    GetPlayerAction,
    ShowWorldEvent,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RunState {
    PreRun,
    DeferToUIFor(UITask),
    WorldTick,
}
