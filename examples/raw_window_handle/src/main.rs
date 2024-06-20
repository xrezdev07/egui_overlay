#![windows_subsystem = "windows"] // to turn off console.

use egui_overlay::EguiOverlay;
#[cfg(feature = "three_d")]
use egui_render_three_d::ThreeDBackend as DefaultGfxBackend;
#[cfg(feature = "wgpu")]
use egui_render_wgpu::WgpuBackend as DefaultGfxBackend;
use raw_window_handle::HasWindowHandle;

#[cfg(not(any(feature = "three_d", feature = "wgpu")))]
compile_error!("you must enable either `three_d` or `wgpu` feature to run this example");
fn main() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    // if RUST_LOG is not set, we will use the following filters
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or(EnvFilter::new("debug,wgpu=warn,naga=warn")),
        )
        .init();

    egui_overlay::start(RawWindowHandleExample { frame: 0 });
}

/// This example shows how to get raw window handle from glfw backend.
/// You can use this raw window handle to do platform specific things.
/// For example, you can pass it to the rfd crate to show file dialogs
/// on top of the overlay.
pub struct RawWindowHandleExample {
    pub frame: u64,
}
impl EguiOverlay for RawWindowHandleExample {
    fn gui_run(
        &mut self,
        egui_context: &egui::Context,
        _default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        // just some controls to show how you can use glfw_backend
        egui::Window::new("controls").show(egui_context, |ui| {
            ui.set_width(300.0);
            self.frame += 1;
            ui.label(format!("current frame number: {}", self.frame));
            // sometimes, you want to see the borders to understand where the overlay is.
            let mut borders = glfw_backend.window.is_decorated();
            if ui.checkbox(&mut borders, "window borders").changed() {
                glfw_backend.window.set_decorated(borders);
            }

            ui.label(format!(
                "pixels_per_virtual_unit: {}",
                glfw_backend.physical_pixels_per_virtual_unit
            ));
            ui.label(format!("window scale: {}", glfw_backend.scale));
            ui.label(format!("cursor pos x: {}", glfw_backend.cursor_pos[0]));
            ui.label(format!("cursor pos y: {}", glfw_backend.cursor_pos[1]));

            ui.label(format!(
                "passthrough: {}",
                glfw_backend.window.is_mouse_passthrough()
            ));

            ui.label(format!(
                "raw window handle: {:?}",
                glfw_backend
                    .window
                    .window_handle()
                    .expect("Failed to get raw window handle"),
            ));

            if ui.button("close overlay").clicked() {
                glfw_backend.window.set_should_close(true);
            }
        });

        // here you decide if you want to be passthrough or not.
        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            // we need input, so we need the window to be NOT passthrough
            glfw_backend.set_passthrough(false);
        } else {
            // we don't care about input, so the window can be passthrough now
            glfw_backend.set_passthrough(true)
        }
        egui_context.request_repaint();
    }
}
