use enigo::*;
use std::f64::consts::PI;
use winit::{event_loop::EventLoop, monitor::MonitorHandle};

const _SPEED: u64 = 1;
const WAIT_TIME: u8 = 4;

fn main() {
    let event_loop = EventLoop::new();
    let primary_monitor: Option<MonitorHandle> =
        event_loop.expect("error at event loop").primary_monitor();

    let (screen_width, screen_height) = if let Some(monitor) = primary_monitor {
        let size = monitor.size();
        (size.width as i32, size.height as i32)
    } else {
        panic!("No primary monitor found");
    };

    let mut enigo = Enigo::new();

    let center_x = screen_width / 4;
    let center_y = screen_height / 4;

    let radius = screen_height / 6;

    let points = 720000; // More points for a smoother circle

    // wait 4 seconds before the action happens
    for i in 0..WAIT_TIME {
        println!("{} seconds to go...", WAIT_TIME - i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let start_angle = 3.0 * PI / 2.0; // Angle for the bottom
    let start_x = (center_x as f64 + radius as f64 * start_angle.cos()).round() as i32;
    let start_y = (center_y as f64 + radius as f64 * start_angle.sin()).round() as i32;

    // Move the mouse to the starting point
    enigo.mouse_move_to(start_x, start_y);
    std::thread::sleep(std::time::Duration::from_millis(100)); // Small delay to ensure the cursor has moved

    // Press down the left mouse button to start drawing
    enigo.mouse_down(MouseButton::Left);
    // Loop through each point to draw the circle, starting from the bottom point
    for i in 0..points {
        let angle = start_angle + 2.0 * PI * i as f64 / points as f64;

        // Ensure the angle stays within the range of 0 to 2PI
        let angle = if angle > 2.0 * PI {
            angle - 2.0 * PI
        } else {
            angle
        };

        // Calculate x and y coordinates
        let x = (center_x as f64 + radius as f64 * angle.cos()).round() as i32;
        let y = (center_y as f64 + radius as f64 * angle.sin()).round() as i32;

        // Move the mouse to the calculated position
        enigo.mouse_move_to(x, y);

        // std::thread::sleep(std::time::Duration::from_millis(SPEED));
    }
    enigo.mouse_up(MouseButton::Left);

    println!("Circle drawing complete!");
}
