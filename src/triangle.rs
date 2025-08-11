use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
pub struct Triangle {
    vertices: (Point, Point, Point, Point),
    fill_color: Option<Color>,
    fill: bool,
}
#[inline]
fn cross(o: Point, a: Point, b: Point) -> i32 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}
impl Triangle {
    pub fn new(vertices: (Point, Point, Point), fill: bool, fill_color: Option<Color>) -> Self {
        Self {
            vertices: (vertices.0, vertices.1, vertices.2, vertices.0),
            fill_color,
            fill,
        }
    }
    pub fn contain_point(&mut self, point: Point) -> bool {
        let c1 = cross(self.vertices.0, self.vertices.1, point);
        let c2 = cross(self.vertices.0, self.vertices.2, point);
        let c3 = cross(self.vertices.1, self.vertices.2, point);

        // Kiểm tra tất cả cùng >= 0 hoặc tất cả <= 0
        (c1 >= 0 && c2 >= 0 && c3 >= 0) || (c1 <= 0 && c2 <= 0 && c3 <= 0)
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        let vertices = self.vertices;
        if self.fill {
            let fill_color = self.fill_color.unwrap();
            canvas.set_draw_color(fill_color);
            // Filled triangle
            canvas
                .filled_trigon(
                    vertices.0.x as i16,
                    vertices.0.y as i16,
                    vertices.1.x as i16,
                    vertices.1.y as i16,
                    vertices.2.x as i16,
                    vertices.2.y as i16,
                    fill_color,
                )
                .unwrap();
        }
        let color = Color::RGB(0, 0, 0);
        canvas.set_draw_color(color);
        let _ = canvas
            .trigon(
                vertices.0.x as i16,
                vertices.0.y as i16,
                vertices.1.x as i16,
                vertices.1.y as i16,
                vertices.2.x as i16,
                vertices.2.y as i16,
                color,
            )
            .unwrap();
    }
}
