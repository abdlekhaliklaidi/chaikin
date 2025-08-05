use macroquad::prelude::*;

#[derive(Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn chaikin(points: &[Point]) -> Vec<Point> {
    let mut new_points = Vec::new();
    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];

        let q = Point::new(0.75 * p0.x + 0.25 * p1.x, 0.75 * p0.y + 0.25 * p1.y);
        let r = Point::new(0.25 * p0.x + 0.75 * p1.x, 0.25 * p0.y + 0.75 * p1.y);

        new_points.push(q);
        new_points.push(r);
    }
    new_points
}

#[macroquad::main("Chaikin")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut animating = false;
    let mut steps: Vec<Vec<Point>> = Vec::new();
    let mut current_step = 0;
    let mut frame_timer = 0.0;

    let mut dragging_index: Option<usize> = None;

    loop {
        clear_background(BLACK);

        // Draw instructions
        draw_text("Left click: add point | Enter: play | C: clear | ESC: quit | Drag points", 10.0, 20.0, 20.0, GRAY);

        let mouse = mouse_position();

        // Handle adding point
        if is_mouse_button_pressed(MouseButton::Left) && !animating {
            let new_point = Point::new(mouse.0, mouse.1);
            control_points.push(new_point);
        }

        // Handle dragging points
        if is_mouse_button_pressed(MouseButton::Left) && !animating {
            for (i, point) in control_points.iter().enumerate() {
                if point.distance_to(&Point::new(mouse.0, mouse.1)) < 10.0 {
                    dragging_index = Some(i);
                    break;
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(i) = dragging_index {
                control_points[i] = Point::new(mouse.0, mouse.1);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging_index = None;
        }

        // Start animation
        if is_key_pressed(KeyCode::Enter) && !control_points.is_empty() {
            steps.clear();
            steps.push(control_points.clone());
            for _ in 0..7 {
                let last = steps.last().unwrap();
                steps.push(chaikin(last));
            }
            animating = true;
            current_step = 0;
            frame_timer = 0.0;
        }

        // Clear points
        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            steps.clear();
            animating = false;
        }

        // Quit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Draw points and lines
        let points_to_draw = if animating {
            frame_timer += get_frame_time();
            if frame_timer > 0.8 {
                current_step = (current_step + 1) % steps.len();
                frame_timer = 0.0;
            }
            &steps[current_step]
        } else {
            &control_points
        };

        // Draw lines
        for i in 0..points_to_draw.len() - 1 {
            let p0 = &points_to_draw[i];
            let p1 = &points_to_draw[i + 1];
            draw_line(p0.x, p0.y, p1.x, p1.y, 2.0, WHITE);
        }

        // Draw points
        for point in points_to_draw {
            draw_circle(point.x, point.y, 5.0, RED);
        }

        next_frame().await
    }
}
