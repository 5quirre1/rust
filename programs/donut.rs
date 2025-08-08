use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use std::f64::consts::PI;

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 40;
    const SIZE: usize = WIDTH * HEIGHT;

    let shades = ".,-~:;=!*#$@";

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    loop {
        let mut output = vec![' '; SIZE];
        let mut zbuffer = vec![0.0_f64; SIZE];

        let theta_step = 0.05;
        let phi_step = 0.015;

        let cos_a = a.cos();
        let sin_a = a.sin();
        let cos_b = b.cos();
        let sin_b = b.sin();

        let r1 = 1.0;
        let r2 = 2.0;
        let k2 = 6.0;
        
        let k1 = (WIDTH as f64 * k2 * 3.0) / (8.0 * (r1 + r2));

        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            let mut phi = 0.0;
            while phi < 2.0 * PI {
                let cos_phi = phi.cos();
                let sin_phi = phi.sin();

                let circle_x = r2 + r1 * cos_theta;
                let circle_y = r1 * sin_theta;

                let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
                    - circle_y * cos_a * sin_b;
                let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
                    + circle_y * cos_a * cos_b;
                let z = k2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
                let ooz = 1.0 / z;

                let xp = (WIDTH as f64 / 2.0 + k1 * ooz * x) as isize;
                let yp = (HEIGHT as f64 / 2.0 - k1 * ooz * y) as isize;

                let lx = -cos_a * sin_theta * cos_phi + cos_theta * cos_phi * sin_a;
                let ly = -cos_theta * sin_phi;
                let lz = -sin_a * sin_theta * sin_phi - cos_a * cos_theta * sin_phi;
                let luminance = (lx + ly + lz).max(0.0);

                if xp >= 0 && xp < WIDTH as isize && yp >= 0 && yp < HEIGHT as isize {
                    let idx = xp as usize + (yp as usize) * WIDTH;
                    if ooz > zbuffer[idx] {
                        zbuffer[idx] = ooz;
                        let shade_idx = (luminance * (shades.len() as f64 - 1.0)).round() as usize;
                        output[idx] = shades.chars().nth(shade_idx).unwrap_or('.');
                    }
                }

                phi += phi_step;
            }
            theta += theta_step;
        }

        let mut stdout = io::stdout();
        for y in 0..HEIGHT {
            let start = y * WIDTH;
            let line: String = output[start..start + WIDTH].iter().collect();
            writeln!(stdout, "{}", line).unwrap();
        }
        stdout.flush().unwrap();
        
        print!("\x1B[{}A", HEIGHT + 1);
        io::stdout().flush().unwrap();
        a += 0.07;
        b += 0.03;
        sleep(Duration::from_micros(15_000));
    }
}
