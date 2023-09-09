use super::super::types::RunState;
use specs::prelude::World;

pub trait UI {
    type Context;
    type ContextResult<Context>;

    fn build_context() -> Self::ContextResult<Self::Context>;

    fn defer_to(ctx: &mut Self::Context, world: &mut World) -> RunState;
}
