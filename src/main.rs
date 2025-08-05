use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

fn chaikin(points: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    // q = 3/4 * p1 + 1/4 * p2  (1/4 point)
    // r = 1/4 * p1 + 3/4 * p2  (3/4 point)
    let mut res = Vec::new();

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        let new_qx = 0.75 * p1.0 + 0.25 * p2.0;
        let new_qy = 0.75 * p1.1 + 0.25 * p2.1;

        let new_rx = 0.25 * p1.0 + 0.75 * p2.0;
        let new_ry = 0.25 * p1.1 + 0.75 * p2.1;

        // let new_r = Point {
        //     x: 0.25 * p1.x + 0.75 * p2.y,
        //     y: 0.25 * p1.x + 0.75 * p2.y,
        // };

        res.push((new_qx, new_qy));
        res.push((new_rx, new_ry));
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
    let mut points = vec![];
    let mut original = vec![];

    let mut is_start = false;
    let mut c = 0;
    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) && !is_start {
            // let (m_x, m_y) = mouse_position();
            points.push(mouse_position());
            original = points.clone();
        }
        for point in &original {
            draw_circle(point.0, point.1, 3.0, WHITE);
        }
        // for point in &points {
        //     draw_circle(point.0, point.1, 3.0, WHITE);
        // }

        if is_key_pressed(KeyCode::Enter) && points.len() >= 2 && !is_start {
            // Start the Chaikin algorithm
            is_start = true;
            original = points.clone();
        }

        if is_start {
            for i in 0..points.len() - 1 {
                // c += 1;
                let p1 = points[i];
                let p2 = points[i + 1];

                draw_line(p1.0, p1.1, p2.0, p2.1, 2.0, GREEN);
            }

            if c <= 7 {
                points = chaikin(&points);
                thread::sleep(Duration::from_secs(1));
                c += 1;
            } else {
                points = original.clone();
                c = 0;
            }
        }

        next_frame().await;
    }
}
