mod core_singleton;
mod timeline;
pub use timeline::*;

use core_singleton::PaciCore;
use godot::classes::Engine;
use godot::prelude::*;

struct PaciCoreExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PaciCoreExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().register_singleton(
                &PaciCore::class_id().to_string_name(),
                &PaciCore::new_alloc(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = &PaciCore::class_id().to_string_name();

            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                godot_error!("PaciCore singleton not found");
            }
        }
    }
}
