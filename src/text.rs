use sdl2::{pixels::Color, render::Canvas, video::Window};

pub struct Text {
    label: String,
    font_path: &'static str,
    font_size : u16,
    text_color: Color,
    position: (i32, i32),
}

impl Text {
    pub fn new(label: String, font_path: &'static str,font_size: u16, position: (i32, i32)) -> Self {
        Self {
            label: label,
            font_path: font_path,
            font_size: font_size,
            text_color : Color::RGB(0, 0, 0),
            position: position,
        }
    }
    pub fn set_label(&mut self, new_label: String) {
        self.label = new_label;
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font(self.font_path, self.font_size).unwrap(); // size 32 px
        let texture_creator = canvas.texture_creator();
        let surface = font
            .render(&self.label)
            .solid(self.text_color) // màu trắng
            .unwrap();

        // Chuyển surface thành texture để vẽ
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let target = sdl2::rect::Rect::new(self.position.0, self.position.1, surface.width(), surface.height());
        let _ = canvas.copy(&texture, None, Some(target));
    }
}
