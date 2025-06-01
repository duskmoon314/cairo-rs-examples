use cairo::{Context, SvgSurface};

fn main() -> anyhow::Result<()> {
    let surface = SvgSurface::new(128.0, 128.0, Some("imgs/diamond.svg"))?;

    let context = Context::new(&surface)?;

    let cx = 64.0;
    let cy = 64.0;
    let r = 50.0;

    let x1 = cx;
    let y1 = cy - r;

    let x2 = cx + r;
    let y2 = cy;

    let x3 = cx;
    let y3 = cy + r;

    let x4 = cx - r;
    let y4 = cy;

    context.move_to(x1, y1);
    context.line_to(x2, y2);
    context.line_to(x3, y3);
    context.line_to(x4, y4);
    context.close_path();

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.fill_preserve()?;

    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_line_width(2.0);
    context.stroke()?;

    Ok(())
}
