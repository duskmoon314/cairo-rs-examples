use cairo::{Context, SvgSurface};

fn main() -> anyhow::Result<()> {
    let surface = SvgSurface::new(128.0, 128.0, Some("imgs/circle.svg"))?;

    let context = Context::new(&surface)?;

    context.arc(64.0, 64.0, 50.0, 0.0, std::f64::consts::PI * 2.0);

    // Fill the circle with white
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.fill_preserve()?;

    // Draw the edge of the circle
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_line_width(2.0);
    context.stroke()?;

    Ok(())
}
