use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn chaikin(points: Vec<Point>) -> Vec<Point> {
    // q = 3/4 * p1 + 1/4 * p2  (1/4 point)
    // r = 1/4 * p1 + 3/4 * p2  (3/4 point)
    let mut res = Vec::new();

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        let new_q = Point {
            x: 0.75 * p1.x + 0.25 * p2.y,
            y: 0.75 * p1.x + 0.25 * p2.y,
        };

        let new_r = Point {
            x: 0.25 * p1.x + 0.75 * p2.y,
            y: 0.25 * p1.x + 0.75 * p2.y,
        };

        res.push(new_q);
        res.push(new_r);
    }

    res
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points: Vec<Point> = vec![];

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (m_x, m_y) = mouse_position();
            points.push(Point { x: (m_x), y: (m_y) });
        }
        clear_background(BLACK);


        


        for point in &points {
            draw_circle(point.x, point.y, 3.0, WHITE);
        }

        next_frame().await;
    }
}
