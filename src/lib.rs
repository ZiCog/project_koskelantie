#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys;

// Our root component
struct Model {
    link: ComponentLink<Self>,
    image_index: usize,
    images: Vec<String>,
}

enum Msg {
    Next,
    Prev,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            image_index: 0,
            images: vec![
                "IMG_20201223_143025.jpg".to_string(),
                "IMG_20201221_154234.jpg".to_string(),
                "IMG_20201223_123622.jpg".to_string(),
                "IMG_20201223_141633.jpg".to_string(),
                "IMG_20201223_143025.jpg".to_string(),
                "IMG_20201224_180229.jpg".to_string(),
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Next => {
                let last = self.images.len() - 1;
                    self.image_index = (self.image_index + 1) % last;
            }
            Msg::Prev => {
                if self.image_index == 0 {
                    let window: web_sys::Window = web_sys::window().expect("window not available");
                    window
                        .alert_with_message("Sorry, 0 is the minimum allowed!")
                        .expect("alert failed");
                } else {
                    self.image_index -= 1;
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let window: web_sys::Window = web_sys::window().expect("window not available");
        window
            .alert_with_message("Model change was called for.")
            .expect("alert failed");
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="text-align:center; height=4000;" >
                <h1>{"Project Koskelantie"}</h1>
                <img src=self.images[self.image_index] alt="Image of project progress." height="768" onclick=self.link.callback(|_| Msg::Next)/>
                <p>{ self.images[self.image_index].clone() }</p>
                <div>
                    <button onclick=self.link.callback(|_| Msg::Prev)>
                        {"Prev"}
                    </button>
                    <button onclick=self.link.callback(|_| Msg::Next)>
                        {"Next"}
                    </button>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    // Mount our web app to the pages <body> and start it.
    App::<Model>::new().mount_to_body();
}
