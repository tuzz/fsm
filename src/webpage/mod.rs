pub mod prelude;

mod webgl;

use stdweb::web;
use stdweb::web::html_element;
use stdweb::unstable::TryInto;
use stdweb::traits::*;

#[derive(New)]
#[Sig = "title: &str"]
#[allow(dead_code)]
pub struct Webpage {
    window: web::Window,
    document: web::Document,
    body: web::HtmlElement,
    canvas: html_element::CanvasElement,
    context: webgl::WebGLRenderingContext,
    style: web::Element,
}

impl Init {
    fn init(&mut self, title: &str) {
        self.fetch_window();
        self.fetch_document();
        self.fetch_body();
        self.create_canvas();
        self.fetch_context();
        self.create_style();

        self.set_page_title(title);
        self.reset_styles();
        self.add_canvas_to_page();
        self.add_style_to_page();
        self.resize_canvas();
    }

    fn fetch_window(&mut self) {
        self.set_window(web::window());
    }

    fn fetch_document(&mut self) {
        self.set_document(web::document());
    }

    fn fetch_body(&mut self) {
        let body = self.document().body().expect("failed to fetch body");
        self.set_body(body);
    }

    fn create_canvas(&mut self) {
        let canvas = self.document().create_element("canvas")
            .expect("failed to create element").try_into()
            .expect("failed to convert element to canvas");

        self.set_canvas(canvas);
    }

    fn fetch_context(&mut self) {
        let context = self.canvas().get_context()
            .expect("failed to fetch render context");

        self.set_context(context);
    }

    fn create_style(&mut self) {
        let style = self.document().create_element("style")
            .expect("failed to create element");

        self.set_style(style)
    }

    fn set_page_title(&self, title: &str) {
        self.document().set_title(title);
    }

    fn reset_styles(&self) {
        let inner = self.document().create_text_node(self.css_reset());
        self.style().append_child(&inner);
    }

    fn css_reset(&self) -> &'static str {
        "html, body, canvas { margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; }"
    }

    fn add_canvas_to_page(&self) {
        self.body().append_child(self.canvas());
    }

    fn add_style_to_page(&self) {
        self.body().append_child(self.style());
    }

    fn resize_canvas(&self) {
        let rectangle = self.body().get_bounding_client_rect();
        let pixel_ratio = self.get_device_pixel_ratio();

        let width = rectangle.get_width() as u32 * pixel_ratio;
        let height = rectangle.get_height() as u32 * pixel_ratio;

        self.canvas().set_width(width);
        self.canvas().set_height(height);
    }

    fn get_device_pixel_ratio(&self) -> u32 {
        let ratio = js! { return window.devicePixelRatio };
        ratio.try_into().unwrap_or(1)
    }
}

impl Webpage {
    pub fn context(&self) -> &webgl::WebGLRenderingContext {
        &self.context
    }
}
