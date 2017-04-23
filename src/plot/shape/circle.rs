//! Circle drawing algorithms

/// Draw's a smooth circle using the midpoint circle algorithm
pub fn draw_circle<P>(xm: i64, ym: i64, mut radius: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut x = -radius;
    let mut y = 0;
    let mut err = 2 - 2 * radius;

    loop {
        plot(xm - x, ym + y, 1.0); /*   I. Quadrant */
        plot(xm - y, ym - x, 1.0); /*  II. Quadrant */
        plot(xm + x, ym - y, 1.0); /* III. Quadrant */
        plot(xm + y, ym + x, 1.0); /*  IV. Quadrant */

        radius = err;

        if radius <= y {
            y += 1;
            err += y * 2 + 1;
        }

        if radius > x || err > y {
            x += 1;
            err += x * 2 + 1;
        }

        if x >= 0 { break; }
    }
}

pub fn draw_circle_aa<P>(xm: i64, ym: i64, mut radius: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 2 - 2 * radius;
    radius = 1 - err;

    plot(xm, ym + (radius / 2 + 1), 1.0);
    plot(xm, ym - (radius / 2 + 1), 1.0);
    plot(xm + (radius / 2 + 1), ym, 1.0);
    plot(xm - (radius / 2 + 1), ym, 1.0);

    let mut plot = |x, y, alpha| {
        if !(x == xm || y == ym) {
            plot(x, y, alpha);
        }
    };

    loop {
        let alpha = 1.0 - (err + 2 * (x + y) - 2).abs() as f64 / radius as f64;

        plot(xm + x, ym - y, alpha); /*   I. Quadrant */
        plot(xm + y, ym + x, alpha); /*  II. Quadrant */
        plot(xm - x, ym + y, alpha); /* III. Quadrant */
        plot(xm - y, ym - x, alpha); /*  IV. Quadrant */

        if x == 0 { break; }

        let e2 = err;
        let mut x2 = x;

        if e2 > y {
            let alpha = 1.0 - (err + 2 * x - 1) as f64 / radius as f64;

            if alpha > 0.0 {
                plot(xm + x, ym - y + 1, alpha);
                plot(xm + y - 1, ym + x, alpha);
                plot(xm - x, ym + y - 1, alpha);
                plot(xm - y + 1, ym - x, alpha);
            }

            x -= 1;
            err -= x * 2 - 1;
        }

        if e2 <= {
            let tmp = x2;
            x2 -= 1;
            tmp
        } {
            let alpha = 1.0 - (1 - 2 * y - e2) as f64 / radius as f64;

            if alpha > 0.0 {
                plot(xm + x2, ym - y, alpha);
                plot(xm + y, ym + x2, alpha);
                plot(xm - x2, ym + y, alpha);
                plot(xm - y, ym - x2, alpha);
            }

            y -= 1;
            err -= y * 2 - 1;
        }
    }
}