use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    rc::Rc,
    sync::mpsc::{channel, Receiver},
    thread::sleep,
    time::{Duration, Instant},
};

use png::{ColorType, Encoder};
use ultralight::{
    buffer::Buffer,
    config::{Config, ViewConfig},
    filesystem::{set_platform_file_system, FileSystem},
    platform::enable_platform_font_loader,
    renderer::Renderer,
    surface::{BitmapSurface, Surface},
    view::View,
};

const UL_HTML: &str = include_str!("./ultralight.html");
const EXAMPLE_HTML: &str = include_str!("./example.html");

struct MyFileSystem {
    base_path: PathBuf,
}

impl MyFileSystem {
    fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

impl FileSystem for MyFileSystem {
    fn file_exists(&self, path: &Path) -> bool {
        self.base_path.join(path).exists()
    }

    fn get_file_mime_type(&self, _path: &Path) -> String {
        "application/unknown".into()
    }

    fn get_file_charset(&self, _path: &Path) -> String {
        "utf-8".into()
    }

    fn open_file(&self, path: &Path) -> Buffer {
        let path = self.base_path.join(path);

        let mut data = Vec::with_capacity(path.metadata().unwrap().len() as usize);
        let mut file = File::open(&path).unwrap();
        file.read_to_end(&mut data).unwrap();

        Buffer::new_owned(data)
    }
}

fn create_config() -> Config {
    let mut config = Config::default();
    config.set_user_stylesheet("html, body {overflow: hidden}");
    config
}

fn create_view_config() -> ViewConfig {
    let mut view_config = ViewConfig::default();
    view_config.set_font_family_standard("Arial");
    view_config
}

fn create_renderer(config: &Config) -> Rc<Renderer> {
    Renderer::new(config)
}

fn create_view(renderer: Rc<Renderer>, width: u32, height: u32, view_config: &ViewConfig) -> View {
    let session = renderer.default_session();
    let view = session.create_view(width, height, view_config);
    view
}

fn _write_png_with_ultralight(surface: &BitmapSurface, path: &Path) {
    surface.write_png(path);
}

fn write_png_with_png_rs(surface: &mut BitmapSurface, path: &Path) {
    let (width, height) = (surface.width(), surface.height());

    let file = File::create(path).unwrap();

    let encoder = {
        let mut e = Encoder::new(&file, width, height);
        e.set_color(ColorType::Rgba);
        e.set_depth(png::BitDepth::Eight);
        e
    };

    let pixels = surface.pixels();

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels.pixels()).unwrap();
    writer.finish().unwrap();
}

fn do_screenshot(
    renderer: Rc<Renderer>,
    view: &mut View,
    rx: &Receiver<()>,
    html: &str,
    save_prefix: &Path,
) {
    let t = Instant::now();

    // Load page
    view.load_html(html);
    loop {
        renderer.update();
        renderer.render();

        if let Ok(_) = rx.try_recv() {
            break;
        }
    }

    println!("Render cost: {:?}", t.elapsed());

    // do screenshot
    let surface = view.surface();
    let mut bitmap_surface: BitmapSurface = surface.into();

    // save PNG with png-rs
    let t = Instant::now();
    // bitmap use BGRA, so do swap
    bitmap_surface.swap_red_blue();
    write_png_with_png_rs(
        &mut bitmap_surface,
        &save_prefix.with_extension("pngrs.png"),
    );
    println!("Build PNG with png-rs cost: {:?}", t.elapsed());
    sleep(Duration::from_secs(1));
    bitmap_surface.swap_red_blue();

    // save PNG with ultralight
    // let t = Instant::now();
    // write_png_with_ultralight(&bitmap_surface, &save_prefix.with_extension("ul.png"));
    // println!("Build PNG with ultralight cost: {:?}", t.elapsed());
}

fn main() {
    // Basic setup
    // set_platform_logger(|level, msg| println!("{level:?} {msg}"));
    // equals to ulPlatformSetFileSystem
    set_platform_file_system(Box::new(MyFileSystem::new("./".into())));
    enable_platform_font_loader();

    // Make configs
    let config = create_config();
    let renderer = create_renderer(&config);
    let view_config = create_view_config();

    // Loaded callback
    let (tx, rx) = channel::<()>();
    let view_complete_callback = |_caller, _frame_id, _is_main, _url: &str| {
        tx.send(()).unwrap();
    };

    let mut view = create_view(renderer.clone(), 1024, 768, &view_config);
    view.set_finish_loading_callback(&view_complete_callback);

    let path_prefix = Path::new("./screenshot");
    loop {
        do_screenshot(renderer.clone(), &mut view, &rx, EXAMPLE_HTML, path_prefix);
        do_screenshot(renderer.clone(), &mut view, &rx, UL_HTML, path_prefix);
    }
}
