# ASCII Renderer

A simplistic 2D polygon renderer written in Rust.

## Usage

1. Clone the project locally

```sh
git clone https://github.com/LucDeCaf/ascii-renderer.git
cd ascii-renderer
```

2. Run the project
```sh
cargo run
```

3. Navigate the environment using the arrow keys and quit the program using `q`.

## Customization

In `src/main.rs`, add a new `Rect` as follows:

```rs
fn main() -> std::io::Result<()> {
  // ... set up renderer

  let new_rectangle = Rect {
    position: Vector2(0.0, 0.0),
    width: 10.0,
    height: 10.0,
  };
  renderer.add_drawable(&new_rectangle);

  // ... main loop, rendering, raw mode
}
```

To create custom drawable structs, implement the `Drawable` trait.

- `bbox` should represent the smallest rectangle that can be drawn around the shape and is used for optimisations
- `point_in_self` should return whether or not a given point is bounded by the shape.

Eg. To implement the `Drawable` trait for a circle struct:

```rs
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

    fn bbox(&self) -> Rect {
        Rect {
            width: self.radius * 2.0,
            height: self.radius * 2.0,
            position: self.position.clone(),
        }
    }
}
```
