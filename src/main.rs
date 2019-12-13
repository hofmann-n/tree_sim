//! The simplest possible example that does something.

mod imgui_wrapper;

use imgui_wrapper::{ImGuiWrapper, UiState};

use ggez;
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    ui_state: UiState,
}

impl MainState {
    fn new(mut ctx: &mut Context) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let ui_state = UiState::new();
        let s = MainState {
            pos_x: 0.0,
            imgui_wrapper,
            ui_state,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Tab
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::Insert
            | KeyCode::Delete
            | KeyCode::Back
            | KeyCode::Space
            | KeyCode::Return
            | KeyCode::Escape
            | KeyCode::NumpadEnter => self.imgui_wrapper.update_key(keycode, true, keymods),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        match keycode {
            KeyCode::Tab
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::Insert
            | KeyCode::Delete
            | KeyCode::Back
            | KeyCode::Space
            | KeyCode::Return
            | KeyCode::Escape
            | KeyCode::NumpadEnter => self.imgui_wrapper.update_key(keycode, false, keymods),
            _ => (),
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {
        self.imgui_wrapper.text_input(_character);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(0.0, 0.0),
                100.0,
                2.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, (na::Point2::new(self.pos_x, 380.0),))?;
        }

        {
            self.imgui_wrapper.render(ctx, &mut self.ui_state);
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple test window", "ggez");
    let (ref mut ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
