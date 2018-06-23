pub mod prelude;

mod webgl;

use stdweb::web;
use stdweb::web::event;
use stdweb::web::html_element;
use stdweb::unstable::TryInto;
use stdweb::traits::*;

pub struct Webpage {
    pub context: webgl::WebGLRenderingContext,
}

impl Webpage {
    pub fn new(title: &str) -> Self {
        let window = Self::fetch_window();
        let document = Self::fetch_document();
        let body = Self::fetch_body(&document);
        let canvas = Self::create_canvas(&document);
        let context = Self::fetch_context(&canvas);

        Self::set_title(&document, title);
        Self::reset_styles(&document, &body);
        Self::add_canvas_to_page(&body, &canvas);
        Self::resize_canvas(&body, &canvas);
        Self::bind_resize_event(&window, &body, &canvas);

        Webpage { context }
    }

    fn fetch_window() -> web::Window {
        web::window()
    }

    fn fetch_document() -> web::Document {
        web::document()
    }

    fn fetch_body(document: &web::Document) -> web::HtmlElement {
        document.body().expect("failed to fetch body")
    }

    fn create_canvas(document: &web::Document) -> html_element::CanvasElement {
        let canvas = document.create_element("canvas").expect("failed to create element");
        canvas.try_into().expect("failed to convert element to canvas")
    }

    fn fetch_context(canvas: &html_element::CanvasElement) -> webgl::WebGLRenderingContext {
        canvas.get_context().expect("failed to fetch render context")
    }

    fn set_title(document: &web::Document, title: &str) {
        document.set_title(title)
    }

    fn reset_styles(document: &web::Document, body: &web::HtmlElement) {
        let style = document.create_element("style").expect("failed to create element");
        let inner = document.create_text_node(Self::css_reset());

        style.append_child(&inner);
        body.append_child(&style);
    }

    fn css_reset() -> &'static str {
        "html, body, canvas { margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; }"
    }

    fn add_canvas_to_page(body: &web::HtmlElement, canvas: &html_element::CanvasElement) {
        body.append_child(canvas);
    }

    fn resize_canvas(body: &web::HtmlElement, canvas: &html_element::CanvasElement) {
        let rectangle = body.get_bounding_client_rect();
        let pixel_ratio = Self::get_device_pixel_ratio();

        canvas.set_width(rectangle.get_width() as u32 * pixel_ratio);
        canvas.set_height(rectangle.get_height() as u32 * pixel_ratio);
    }

    fn bind_resize_event(window: &web::Window, body: &web::HtmlElement, canvas: &html_element::CanvasElement) {
        let b = body.clone();
        let c = canvas.clone();

        window.add_event_listener(move |_: event::ResizeEvent| {
            Self::resize_canvas(&b, &c)
        });
    }

    fn get_device_pixel_ratio() -> u32 {
        let ratio = js! { return window.devicePixelRatio };
        ratio.try_into().unwrap_or(1)
    }
}
