use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;

const GLADE_UI_SOURCE: &'static str = include_str!("app.glade");

fn build_ui(app: &gtk::Application) {
    let builder = gtk::Builder::from_string(GLADE_UI_SOURCE);

    let window = builder.object::<gtk::Window>("MainWindow").unwrap();
    window.set_application(Some(app));

    let drawing_area: gtk::DrawingArea = builder.object("MainDrawingArea").unwrap();

    drawing_area.connect_draw(move |widget, cr| {
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        
        let root = backend.into_drawing_area();

        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root).caption("A test chart", ("sans-serif", 50).into_font())
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32).unwrap();

        chart.draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0),
            // The below would be the correct arg, yielding tuples instead of scalars    
            //(-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        )).unwrap();

        root.present().unwrap();

        Inhibit(false)
    });

    window.show_all();
}


fn main() {
    let app = gtk::Application::new(
        Some("acoustic.beamforming.display"),
        Default::default(),
    );
    app.connect_activate(|app| {
        build_ui(app);
    });
    app.run();
}
