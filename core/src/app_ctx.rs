use sodium::SodiumCtx;

pub struct AppCtx {
    pub sodium_ctx: SodiumCtx
}

impl AppCtx {
    pub fn new() -> AppCtx {
        AppCtx {
            sodium_ctx: SodiumCtx::new()
        }
    }
}