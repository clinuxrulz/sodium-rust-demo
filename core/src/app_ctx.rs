use drivers::DisplayDriver;

use sodium::SodiumCtx;

pub struct AppCtx {
    pub sodium_ctx: SodiumCtx,
    pub display_driver: Box<DisplayDriver>
}

impl AppCtx {
    pub fn new<DD: DisplayDriver + 'static>(display_driver: DD) -> AppCtx {
        AppCtx {
            sodium_ctx: SodiumCtx::new(),
            display_driver: Box::new(display_driver)
        }
    }
}
