use stdweb::web;
use stdweb::web::event;
use stdweb::web::html_element;
use stdweb::unstable::TryInto;
use stdweb::traits::*;

pub struct Webpage {

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

        Webpage { }
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

    fn fetch_context(canvas: &html_element::CanvasElement) -> web::CanvasRenderingContext2d {
        canvas.get_context().expect("failed to fetch render context")
    }

    fn set_title(document: &web::Document, title: &str) {
        document.set_title(title)
    }

    fn reset_styles(document: &web::Document, body: &web::HtmlElement) {
        let style = document.create_element("style").expect("failed to create element");
        let css = document.create_text_node("
            html, body { margin: 0; height: 100%; overflow: hidden; }
        ");

        style.append_child(&css);
        body.append_child(&style);
    }

    fn add_canvas_to_page(body: &web::HtmlElement, canvas: &html_element::CanvasElement) {
        body.append_child(canvas);
    }

    fn resize_canvas(body: &web::HtmlElement, canvas: &html_element::CanvasElement) {
        let rectangle = body.get_bounding_client_rect();

        canvas.set_width(rectangle.get_width() as u32);
        canvas.set_height(rectangle.get_height() as u32);
    }

    fn bind_resize_event(window: &web::Window, body: &web::HtmlElement, canvas: &html_element::CanvasElement) {
        let b = body.clone();
        let c = canvas.clone();

        window.add_event_listener(move |_: event::ResizeEvent| {
            Self::resize_canvas(&b, &c)
        });
    }
}
