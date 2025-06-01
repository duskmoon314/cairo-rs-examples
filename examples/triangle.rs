use cairo::{Context, SvgSurface};

fn main() -> anyhow::Result<()> {
    let surface = SvgSurface::new(128.0, 128.0, Some("imgs/triangle.svg"))?;

    let context = Context::new(&surface)?;

    let cx = 64.0;
    let cy = 64.0;
    let r = 50.0;

    let x1 = cx + r * (0.0f64).sin();
    let y1 = cy - r * (0.0f64).cos();

    let x2 = cx + r * (2.0 * std::f64::consts::PI / 3.0).sin();
    let y2 = cy - r * (2.0 * std::f64::consts::PI / 3.0).cos();

    let x3 = cx + r * (4.0 * std::f64::consts::PI / 3.0).sin();
    let y3 = cy - r * (4.0 * std::f64::consts::PI / 3.0).cos();

    context.move_to(x1, y1);
    context.line_to(x2, y2);
    context.line_to(x3, y3);
    context.close_path();

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.fill_preserve()?;

    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_line_width(2.0);
    context.stroke()?;

    Ok(())
}
