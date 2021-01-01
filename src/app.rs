#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys;
use yew::services::ConsoleService;


// Our root component
pub struct App {
    link: ComponentLink<Self>,
    image_index: usize,
    images: Vec<String>,
}

pub enum Msg {
    Next,
    Prev,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            image_index: 0,
            images: vec![
                "IMG_20201223_143025.jpg".to_string(), // Fitst pic not in date order.
                "IMG_20200723_112309.jpg".to_string(),
                "IMG_20200723_112418.jpg".to_string(),
                "IMG_20200723_112527.jpg".to_string(),
                "IMG_20200723_112609.jpg".to_string(),
                "IMG_20200723_112715.jpg".to_string(),
                "IMG_20200723_112740.jpg".to_string(),
                "IMG_20200723_112844.jpg".to_string(),
                "IMG_20200723_112848.jpg".to_string(),
                "IMG_20200723_113132.jpg".to_string(),
                "IMG_20200723_113213.jpg".to_string(),
                "IMG_20200723_113423.jpg".to_string(),
                "IMG_20200723_113431.jpg".to_string(),
                "IMG_20200723_113437.jpg".to_string(),
                "IMG_20200723_113630.jpg".to_string(),
                "IMG_20200723_113700.jpg".to_string(),
                "IMG_20200723_113708.jpg".to_string(),
                "IMG_20200723_113717.jpg".to_string(),
                "IMG_20200723_113955.jpg".to_string(),
                "IMG_20200723_114045.jpg".to_string(),
                "IMG_20200723_114049.jpg".to_string(),
                "IMG_20200723_114546.jpg".to_string(),
                "IMG_20200723_114736.jpg".to_string(),
                "IMG_20200723_114846.jpg".to_string(),
                "IMG_20200723_115430.jpg".to_string(),
                "IMG_20201109_143320.jpg".to_string(),
                "IMG_20201109_143328.jpg".to_string(),
                "IMG_20201109_143342.jpg".to_string(),
                "IMG_20201109_143655.jpg".to_string(),
                "IMG_20201109_143659.jpg".to_string(),
                "IMG_20201109_143713.jpg".to_string(),
                "IMG_20201109_144254.jpg".to_string(),
                "IMG_20201109_144258.jpg".to_string(),
                "IMG_20201109_144302.jpg".to_string(),
                "IMG_20201110_125817.jpg".to_string(),
                "IMG_20201112_143420.jpg".to_string(),
                "IMG_20201112_143559.jpg".to_string(),
                "IMG_20201118_142457.jpg".to_string(),
                "IMG_20201119_130040.jpg".to_string(),
                "IMG_20201119_130047.jpg".to_string(),
                "IMG_20201119_130159.jpg".to_string(),
                "IMG_20201119_130208.jpg".to_string(),
                "IMG_20201119_130243.jpg".to_string(),
                "IMG_20201119_130300.jpg".to_string(),
                "IMG_20201119_130649.jpg".to_string(),
                "IMG_20201127_132820.jpg".to_string(),
                "IMG_20201127_132849.jpg".to_string(),
                "IMG_20201127_133009.jpg".to_string(),
                "IMG_20201127_133045.jpg".to_string(),
                "IMG_20201127_133104.jpg".to_string(),
                "IMG_20201127_133349.jpg".to_string(),
                "IMG_20201127_133601.jpg".to_string(),
                "IMG_20201201_142138.jpg".to_string(),
                "IMG_20201201_142411.jpg".to_string(),
                "IMG_20201201_143420.jpg".to_string(),
                "IMG_20201202_135310.jpg".to_string(),
                "IMG_20201202_135340.jpg".to_string(),
                "IMG_20201205_105936.jpg".to_string(),
                "IMG_20201205_110403.jpg".to_string(),
                "IMG_20201205_110429.jpg".to_string(),
                "IMG_20201205_130158.jpg".to_string(),
                "IMG_20201205_130259.jpg".to_string(),
                "IMG_20201205_130457.jpg".to_string(),
                "IMG_20201208_114621.jpg".to_string(),
                "IMG_20201208_114753.jpg".to_string(),
                "IMG_20201208_114826.jpg".to_string(),
                "IMG_20201208_114951.jpg".to_string(),
                "IMG_20201208_132016.jpg".to_string(),
                "IMG_20201208_132312.jpg".to_string(),
                "IMG_20201217_110408.jpg".to_string(),
                "IMG_20201217_110420.jpg".to_string(),
                "IMG_20201217_110445.jpg".to_string(),
                "IMG_20201217_110526.jpg".to_string(),
                "IMG_20201217_112310.jpg".to_string(),
                "IMG_20201218_135015.jpg".to_string(),
                "IMG_20201218_135038.jpg".to_string(),
                "IMG_20201218_135236.jpg".to_string(),
                "IMG_20201221_154234.jpg".to_string(),
                "IMG_20201223_123622.jpg".to_string(),
                "IMG_20201223_141633.jpg".to_string(),
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Next => {
                let last = self.images.len() - 1;
                    self.image_index = (self.image_index + 1) % last;
                    ConsoleService::error(&format!("Image index: {}", self.image_index));
            }
            Msg::Prev => {
                if self.image_index == 0 {
                    let window: web_sys::Window = web_sys::window().expect("window not available");
                    window
                        .alert_with_message("Sorry, 0 is the minimum allowed!")
                        .expect("alert failed");
                } else {
                    self.image_index -= 1;
                    ConsoleService::info(&format!("Image index: {}", self.image_index));
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let window: web_sys::Window = web_sys::window().expect("window not available");
        window
            .alert_with_message("App change was called for.")
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
                    {" "}
                    <button onclick=self.link.callback(|_| Msg::Next)>
                        {"Next"}
                    </button>
                </div>
            </div>
        }
    }
}


