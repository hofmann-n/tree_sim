use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

pub struct UiState {
    text: imgui::ImString,
}

impl UiState {
    pub fn new() -> UiState {
        UiState {
            text: imgui::ImString::new("test buffer"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
    show_popup: bool,
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> Self {
        // Create the imgui object
        let mut imgui = imgui::Context::create();

        imgui.io_mut().key_map[Key::Tab as usize] = ggez::input::keyboard::KeyCode::Tab as u32;
        imgui.io_mut().key_map[Key::LeftArrow as usize] =
            ggez::input::keyboard::KeyCode::Left as u32;
        imgui.io_mut().key_map[Key::RightArrow as usize] =
            ggez::input::keyboard::KeyCode::Right as u32;
        imgui.io_mut().key_map[Key::UpArrow as usize] = ggez::input::keyboard::KeyCode::Up as u32;
        imgui.io_mut().key_map[Key::DownArrow as usize] =
            ggez::input::keyboard::KeyCode::Down as u32;
        imgui.io_mut().key_map[Key::PageUp as usize] =
            ggez::input::keyboard::KeyCode::PageUp as u32;
        imgui.io_mut().key_map[Key::PageDown as usize] =
            ggez::input::keyboard::KeyCode::PageDown as u32;
        imgui.io_mut().key_map[Key::Home as usize] = ggez::input::keyboard::KeyCode::Home as u32;
        imgui.io_mut().key_map[Key::End as usize] = ggez::input::keyboard::KeyCode::End as u32;
        imgui.io_mut().key_map[Key::Insert as usize] =
            ggez::input::keyboard::KeyCode::Insert as u32;
        imgui.io_mut().key_map[Key::Delete as usize] =
            ggez::input::keyboard::KeyCode::Delete as u32;
        imgui.io_mut().key_map[Key::Backspace as usize] =
            ggez::input::keyboard::KeyCode::Back as u32;
        imgui.io_mut().key_map[Key::Space as usize] = ggez::input::keyboard::KeyCode::Space as u32;
        imgui.io_mut().key_map[Key::Enter as usize] = ggez::input::keyboard::KeyCode::Return as u32;
        imgui.io_mut().key_map[Key::Escape as usize] =
            ggez::input::keyboard::KeyCode::Escape as u32;
        imgui.io_mut().key_map[Key::KeyPadEnter as usize] =
            ggez::input::keyboard::KeyCode::NumpadEnter as u32;

        let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

        // Shaders
        let shaders = {
            let version = gfx_device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                Shaders::GlSl130
            } else {
                Shaders::GlSl110
            }
        };

        // Renderer
        let renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

        // Create instace
        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            show_popup: false,
        }
    }

    pub fn render(&mut self, ctx: &mut Context, ui_state: &mut UiState) {
        // Update mouse
        self.update_mouse();

        // Create new frame
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let (draw_width, draw_height) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [draw_width, draw_height];
        self.imgui.io_mut().display_framebuffer_scale = [1.0, 1.0];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();

        // Various ui things
        {
            Window::new(im_str!("Hello World"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("こんにちは世界！"));
                    ui.text(im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            Window::new(im_str!("Hello World2"))
                .size([200.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.input_text(im_str!("test test"), &mut ui_state.text)
                        .build();
                });
        }

        // Render
        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer
            .render(
                &mut *factory,
                encoder,
                &mut RenderTargetView::new(render_target.clone()),
                draw_data,
            )
            .unwrap();
    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos =
            [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
        self.mouse_state.pressed = pressed;

        if pressed.0 {
            self.show_popup = false;
        }
    }

    pub fn text_input(&mut self, character: char) {
        self.imgui.io_mut().add_input_character(character);
    }

    pub fn update_key(
        &mut self,
        keycode: ggez::input::keyboard::KeyCode,
        pressed: bool,
        keymods: ggez::input::keyboard::KeyMods,
    ) {
        self.set_mod(keymods);
        self.imgui.io_mut().keys_down[keycode as usize] = pressed;
    }

    fn set_mod(&mut self, keymods: ggez::input::keyboard::KeyMods) {
        let ctrl = keymods.intersects(ggez::input::keyboard::KeyMods::CTRL);
        let shift = keymods.intersects(ggez::input::keyboard::KeyMods::SHIFT);
        let alt = keymods.intersects(ggez::input::keyboard::KeyMods::ALT);
        let logo = keymods.intersects(ggez::input::keyboard::KeyMods::LOGO);

        self.imgui.io_mut().key_ctrl = ctrl;
        self.imgui.io_mut().key_alt = alt;
        self.imgui.io_mut().key_shift = shift;
        self.imgui.io_mut().key_super = logo;
    }
}
