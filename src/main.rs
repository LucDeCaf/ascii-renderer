use std::io::{stdout, Write};

use crossterm::{
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

trait Drawable {
    fn point_in_self(&self, point: Point<f32>) -> bool;
}

#[derive(Debug, Clone, Copy)]
struct Point<T>(T, T);

#[derive(Debug, Clone)]
struct Circle {
    position: Point<f32>,
    radius: f32,
}

impl Drawable for Circle {
    fn point_in_self(&self, point: Point<f32>) -> bool {
        let x_diff = point.0 - self.position.0;
        let y_diff = point.1 - self.position.1;
        let distance = ((x_diff * x_diff) + (y_diff * y_diff)).sqrt();

        distance <= self.radius
    }
}

struct Renderer {
    options: RendererOptions,
    buffer: Vec<char>,
    drawables: Vec<Box<dyn Drawable>>,
}

struct RendererOptions {
    viewport_width: usize,
    viewport_height: usize,
}

impl Renderer {
    fn new(options: RendererOptions) -> Self {
        Self {
            buffer: vec!['-'; options.viewport_width * options.viewport_height],
            drawables: Vec::new(),
            options,
        }
    }

    fn add_drawable<T: Drawable + 'static>(&mut self, drawable: T) {
        self.drawables.push(Box::new(drawable));
    }

    fn pixels(&self) -> Vec<Point<f32>> {
        let mut pixels =
            Vec::with_capacity(self.options.viewport_width * self.options.viewport_height);

        for x in 0..self.options.viewport_width {
            for y in 0..self.options.viewport_height {
                pixels.push(Point(x as f32, y as f32));
            }
        }

        pixels
    }

    fn lines(&self) -> Vec<String> {
        let mut strings = Vec::new();
        let mut i = 0;

        while i < self.options.viewport_width * self.options.viewport_height {
            strings.push(
                self.buffer[i..i + self.options.viewport_width]
                    .iter()
                    .collect(),
            );
            i += self.options.viewport_width;
        }

        strings
    }

    fn index_f32(&self, point: Point<f32>) -> usize {
        (point.0 + (point.1 * self.options.viewport_width as f32)) as usize
    }

    fn render(&mut self) {
        for point in self.pixels() {
            for shape in self.drawables.iter() {
                if shape.point_in_self(point) {
                    println!("Writing 1 hash at {point:?}");
                    let index = self.index_f32(point);
                    self.buffer[index] = '#';
                }
            }
        }

        for point in self.pixels() {
            if self.buffer[self.index_f32(point)] == '#' {
                println!("Hash found at {point:?}");
            }
        }
    }

    fn draw(&self) -> std::io::Result<()> {
        let mut stdout = stdout();

        queue!(stdout, Clear(ClearType::All))?;

        for line in self.lines() {
            let mut out = String::with_capacity(self.options.viewport_width * 2);
            for c in line.chars() {
                out.push(c);
                out.push(' ');
            }
            out.push('\n');

            queue!(stdout, Print(out))?;
        }

        stdout.flush()?;

        Ok(())
    }
}

fn main() {
    let mut renderer = Renderer::new(RendererOptions {
        viewport_width: 32,
        viewport_height: 18,
    });

    renderer.add_drawable(Circle {
        radius: 10.0,
        position: Point(0.0, 0.0),
    });

    renderer.render();

    if let Err(err) = renderer.draw() {
        println!("Failed to write to buffer: {}", err.to_string());
    }
}