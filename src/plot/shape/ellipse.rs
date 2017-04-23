//! Ellipse drawing algorithm

use ::utils::min_max;

/// Draw's a smooth ellipse using the midpoint circle algorithm
pub fn draw_ellipse<P>(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut a = (x1 - x0).abs();
    let b = (y1 - x0).abs();

    if a == 0 || b == 0 {
        ::plot::line::draw_line_bresenham(x0, y0, x1, y1, plot);
    } else {
        // 1 if odd, 0 if even
        let mut b1 = b & 0x1;

        let mut dx = 4 * (1 - a) * b * b;
        let mut dy = 4 * (b1 + 1) * a * a;

        let mut err = dx + dy + b1 * a * a;

        if x0 > x1 {
            x0 = x1;
            x1 += a;
        }

        if y0 > y1 {
            y0 = y1;
        }

        y0 += (b + 1) / 2;
        y1 = y0 - b1;

        a *= 8 * a;
        b1 = 8 * b * b;

        loop {
            plot(x1, y0, 1.0); /*   I. Quadrant */
            plot(x0, y0, 1.0); /*  II. Quadrant */
            plot(x0, y1, 1.0); /* III. Quadrant */
            plot(x1, y1, 1.0); /*  IV. Quadrant */

            let e2 = 2 * err;

            if e2 <= dy {
                y0 += 1;
                y1 -= 1;
                dy += a;
                err += dy;
            }

            if e2 >= dx || 2 * err > dy {
                x0 += 1;
                x1 -= 1;
                dx += b1;
                err += dx;
            }

            if x0 > x1 { break; }
        }

        while (y0 - y1) < b {
            plot(x0 - 1, y0, 1.0);
            plot(x1 + 1, y0, 1.0);
            plot(x0 - 1, y1, 1.0);
            plot(x1 + 1, y1, 1.0);

            y0 += 1;
            y1 -= 1;
        }
    }
}

pub fn draw_ellipse_aa<P>(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut a = (x1 - x0).abs();
    let b = (y1 - y0).abs();

    if a == 0 || b == 0 {
        ::plot::line::draw_line_bresenham(x0, y0, x1, y1, plot);
    } else {
        let aa = (x1 + x0) / 2;
        let bb = (y1 + y0) / 2;

        plot(aa, y0, 1.0);
        plot(x0, bb, 1.0);
        plot(aa, y1, 1.0);
        plot(x1, bb, 1.0);

        let mut plot = |x, y, alpha| {
            if !(x == aa || y == bb) {
                plot(x, y, alpha);
            }
        };

        // 1 if odd, 0 if even
        let mut b1 = b & 0x1;

        let mut dx = 4 * (a - 1) * b * b;
        let mut dy = 4 * (b1 + 1) * a * a;

        let mut err = b1 * a * a - dx + dy;

        if x0 > x1 {
            x0 = x1;
            x1 += a;
        }

        if y0 > y1 {
            y0 = y1;
        }

        y0 += (b + 1) >> 1;

        y1 = y0 - b1;

        a = 8 * a * a;
        b1 = 8 * b * b;

        loop {
            let (i, ed) = min_max(dx, dy);

            let ed: f64 = if y0 == (y1 + 1) && err > dy && a > b1 {
                4.0 / a as f64
            } else {
                let (ed, i) = (ed as f64, i as f64);

                1.0 / (ed + 2.0 * ed * i * i / (4.0 * ed * ed + i * i))
            };

            let alpha: f64 = 1.0 - ed * (err + dx - dy).abs() as f64;

            plot(x0, y0, alpha);
            plot(x0, y1, alpha);
            plot(x1, y0, alpha);
            plot(x1, y1, alpha);

            let f = 2 * err + dy;

            if f >= 0 {
                if x0 == x1 { break; }

                let alpha = 1.0 - ed * (err + dx) as f64;

                if alpha > 0.0 {
                    plot(x0, y0 + 1, alpha);
                    plot(x0, y1 - 1, alpha);
                    plot(x1, y0 + 1, alpha);
                    plot(x1, y1 - 1, alpha);
                }
            }

            if 2 * err <= dx {
                let alpha = 1.0 - ed * (dy - err) as f64;

                if alpha > 0.0 {
                    plot(x0 + 1, y0, alpha);
                    plot(x1 - 1, y0, alpha);
                    plot(x0 + 1, y1, alpha);
                    plot(x1 - 1, y1, alpha);
                }

                y0 += 1;
                y1 -= 1;
                dy += a;
                err += dy;
            }

            if f > 0 {
                x0 += 1;
                x1 -= 1;
                dx -= b1;
                err -= dx;
            }
        }

        x0 -= 1;

        if x0 == {
            let tmp = x1;
            x1 += 1;
            tmp
        } {
            while (y0 - y1) < b {
                let alpha = 1.0 - 4.0 * (err + dx).abs() as f64 / b1 as f64;

                y0 += 1;

                plot(x0, y0, alpha);
                plot(x1, y0, alpha);

                y1 -= 1;

                plot(x0, y1, alpha);
                plot(x1, y1, alpha);

                dy += a;
                err += dy;
            }
        }
    }
}