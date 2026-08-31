use hann_rs::{HannMode, hann_f64};
use plotters::coord::Shift;
use plotters::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::ops::Range;
use std::path::{Path, PathBuf};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 680;
const SAMPLE_COUNT: usize = 32;
const DEFAULT_OUTPUT: &str = "plots/hann_window_modes.svg";
const BLUE: RGBColor = RGBColor(37, 99, 235);
const ORANGE: RGBColor = RGBColor(234, 88, 12);

fn main() -> Result<(), Box<dyn Error>> {
    let output = env_output_path();
    create_parent_directory(&output)?;

    let symmetric = hann_f64(SAMPLE_COUNT, HannMode::Symmetric);
    let periodic = hann_f64(SAMPLE_COUNT, HannMode::Periodic);

    let root = SVGBackend::new(&output, (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let (header, content) = root.split_vertically(100);
    let (plots, footer) = content.split_vertically(500);
    header.draw(&Text::new(
        "Hann window modes",
        (24, 38),
        ("sans-serif", 30).into_font(),
    ))?;
    header.draw(&Text::new(
        "N = 32 · f64 coefficients · normalized amplitude",
        (24, 72),
        ("sans-serif", 17).into_font(),
    ))?;

    let panels = plots.split_evenly((1, 2));
    draw_panel(
        &panels[0],
        "Full window",
        0.0..31.0,
        0..SAMPLE_COUNT,
        &symmetric,
        &periodic,
    )?;
    draw_panel(
        &panels[1],
        "Trailing samples (24..31)",
        24.0..31.0,
        24..SAMPLE_COUNT,
        &symmetric,
        &periodic,
    )?;

    footer.draw(&Text::new(
        "Periodic mode omits the duplicated zero endpoint; the trailing panel highlights this tail difference.",
        (24, 38),
        ("sans-serif", 14).into_font(),
    ))?;

    root.present().map_err(|error| {
        io::Error::other(format!(
            "failed to write SVG '{}': {error}",
            output.display()
        ))
    })?;
    println!("Generated {}", output.display());
    Ok(())
}

fn env_output_path() -> PathBuf {
    std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT))
}

fn create_parent_directory(output: &Path) -> io::Result<()> {
    let Some(parent) = output.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    fs::create_dir_all(parent).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!(
                "failed to create output directory '{}': {error}",
                parent.display()
            ),
        )
    })
}

fn draw_panel(
    area: &DrawingArea<SVGBackend<'_>, Shift>,
    title: &str,
    x_range: Range<f64>,
    indices: Range<usize>,
    symmetric: &[f64],
    periodic: &[f64],
) -> Result<(), Box<dyn Error>> {
    let mut chart = ChartBuilder::on(area)
        .margin(12)
        .caption(title, ("sans-serif", 21))
        .x_label_area_size(42)
        .y_label_area_size(48)
        .build_cartesian_2d(x_range, 0.0..1.05)?;

    chart
        .configure_mesh()
        .x_labels(8)
        .y_labels(5)
        .x_desc("Sample index")
        .y_desc("Amplitude")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            indices
                .clone()
                .map(|index| (index as f64, symmetric[index])),
            BLUE,
        ))?
        .label("Symmetric (D = N − 1)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], BLUE));
    chart.draw_series(PointSeries::of_element(
        indices
            .clone()
            .map(|index| (index as f64, symmetric[index])),
        4,
        ShapeStyle::from(BLUE).filled(),
        &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
    ))?;

    chart
        .draw_series(LineSeries::new(
            indices.clone().map(|index| (index as f64, periodic[index])),
            ORANGE,
        ))?
        .label("Periodic (D = N)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], ORANGE));
    chart.draw_series(PointSeries::of_element(
        indices.map(|index| (index as f64, periodic[index])),
        4,
        ShapeStyle::from(ORANGE).filled(),
        &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
    ))?;

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(WHITE.mix(0.9))
        .border_style(BLACK)
        .draw()?;
    Ok(())
}
