use cairo::{Context, SvgSurface};

fn main() -> anyhow::Result<()> {
    let surface = SvgSurface::new(128.0, 128.0, Some("imgs/square.svg"))?;

    let context = Context::new(&surface)?;

    context.rectangle(14.0, 14.0, 100.0, 100.0);

    // Fill the square with white
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.fill_preserve()?;

    // Draw the black border
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_line_width(2.0);
    context.stroke()?;

    Ok(())
}
