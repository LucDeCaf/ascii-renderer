use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

use ascii_renderer::vector2::Vector2;

trait Drawable {
    fn point_in_self(&self, point: &Vector2<f32>) -> bool;
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_vector_f32(&self) -> Vector2<f32> {
        match self {
            Direction::Up => Vector2(0.0, 1.0),
            Direction::Down => Vector2(0.0, -1.0),
            Direction::Left => Vector2(-1.0, 0.0),
            Direction::Right => Vector2(1.0, 0.0),
        }
    }
}

#[derive(Debug, Clone)]
struct Rect {
    position: Vector2<f32>,
    width: f32,
    height: f32,
}

impl Drawable for Rect {
    fn point_in_self(&self, point: &Vector2<f32>) -> bool {
        let max_x = self.position.0 + self.width as f32;
        let max_y = self.position.1 + self.height as f32;

        (self.position.0..max_x).contains(&point.0) && (self.position.1..max_y).contains(&point.1)
    }
}

#[derive(Debug, Clone)]
struct Circle {
    position: Vector2<f32>,
    radius: f32,
}

impl Drawable for Circle {
    fn point_in_self(&self, point: &Vector2<f32>) -> bool {
        let x_diff = point.0 - self.position.0;
        let y_diff = point.1 - self.position.1;
        let distance = ((x_diff * x_diff) + (y_diff * y_diff)).sqrt();

        distance <= self.radius
    }
}

struct Renderer<'a> {
    options: RendererOptions,
    position: Vector2<f32>,
    buffer: Vec<char>,
    drawables: Vec<&'a dyn Drawable>,
}

struct RendererOptions {
    viewport_width: usize,
    viewport_height: usize,
}

impl<'a> Renderer<'a> {
    fn new(options: RendererOptions) -> Self {
        Self {
            buffer: vec![' '; options.viewport_width * options.viewport_height],
            position: Vector2(0.0, 0.0),
            drawables: Vec::new(),
            options,
        }
    }

    fn bbox(&self) -> Rect {
        Rect {
            position: self.position.clone()
                - Vector2(
                    (self.options.viewport_width / 2) as f32,
                    (self.options.viewport_height / 2) as f32,
                ),
            width: self.options.viewport_width as f32,
            height: self.options.viewport_height as f32,
        }
    }

    fn walk(&mut self, direction: Direction, distance: f32) {
        self.position += direction.as_vector_f32() * distance;
    }

    fn add_drawable<T: Drawable>(&mut self, drawable: &'a T) {
        self.drawables.push(drawable);
    }

    fn local_pixels(&self) -> Vec<Vector2<f32>> {
        let mut pixels =
            Vec::with_capacity(self.options.viewport_width * self.options.viewport_height);

        for x in 0..self.options.viewport_width {
            for y in 0..self.options.viewport_height {
                pixels.push(Vector2(x as f32, y as f32));
            }
        }

        pixels
    }

    fn global_pixels(&self) -> Vec<Vector2<f32>> {
        let start_x = self.position.0;
        let start_y = self.position.1;
        let max_x = start_x + self.options.viewport_width as f32;
        let max_y = start_y + self.options.viewport_height as f32;

        let mut pixels =
            Vec::with_capacity(self.options.viewport_width * self.options.viewport_height);

        let mut x = start_x;
        let mut y;

        while x <= max_x {
            y = start_y;
            while y <= max_y {
                pixels.push(Vector2(x, y));
                y += 1.0;
            }
            x += 1.0;
        }

        pixels
    }

    fn global_position_of(&self, point: &Vector2<f32>) -> Vector2<f32> {
        Vector2(self.position.0 + point.0, self.position.1 - point.1)
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

    fn index_f32(&self, point: &Vector2<f32>) -> usize {
        (point.0 + (point.1 * self.options.viewport_width as f32)) as usize
    }

    fn render(&mut self) {
        // Clear buffer
        self.buffer.fill(' ');

        // Render content
        for point in self.local_pixels() {
            for shape in self.drawables.iter() {
                let global_pos = self.global_position_of(&point);
                if shape.point_in_self(&global_pos) {
                    let index = self.index_f32(&point);
                    self.buffer[index] = '#';
                }
            }
        }
    }

    #[allow(unused)]
    fn draw_standard_terminal(&self) -> std::io::Result<()> {
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

    fn draw(&self) -> std::io::Result<()> {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;

        for line in self.lines() {
            let mut out = String::with_capacity(self.options.viewport_width * 2);

            for c in line.chars() {
                out.push(c);
                out.push(' ');
            }

            execute!(stdout, Print(out), MoveToNextLine(1))?;
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let size = size()?;

    let mut renderer = Renderer::new(RendererOptions {
        viewport_width: size.1 as usize,
        viewport_height: size.0 as usize - 8, // Leave room for output text
    });

    let circle = Circle {
        radius: 10.0,
        position: Vector2(7.0, 3.0),
    };
    renderer.add_drawable(&circle);

    enable_raw_mode()?;

    renderer.render();
    renderer.draw()?;

    'main: loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => match c {
                        'q' => break 'main,
                        _ => (),
                    },
                    KeyCode::Up => renderer.walk(Direction::Up, 1.0),
                    KeyCode::Down => renderer.walk(Direction::Down, 1.0),
                    KeyCode::Left => renderer.walk(Direction::Left, 1.0),
                    KeyCode::Right => renderer.walk(Direction::Right, 1.0),
                    _ => (),
                }
            }
            renderer.render();
            renderer.draw()?;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
