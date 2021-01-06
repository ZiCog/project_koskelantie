use anyhow::Error;
use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

type AsBinary = bool;
pub enum Format {
    Json,
    Toml,
}

pub enum Msg {
    Clicked,
    FetchData(Format, AsBinary),
    FetchReady(Result<DataFromFile, Error>),
}

/// This type is used to parse data from `./static/data.json` file and
/// have to correspond the data layout from that file.
#[derive(Clone, Deserialize, Debug)]
pub struct DataFromFile {
    value: u32,
    files: Vec<String>,
}
// Define a button component
pub struct Fetcher {
    link: ComponentLink<Self>,
    title: String,
    onsignal: Callback<()>,
    fetching: bool,
    data: Option<DataFromFile>,
    error: Option<String>,
    fetch_task: Option<FetchTask>,
}

// Properties passed in my parent component.
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub onsignal: Callback<()>,
}

// Implementation of some methods on our fetcher.
impl Fetcher {

    fn view_data(&self) -> Html {
        if let Some(data_from_file) = &self.data {
            let files: Vec<Html> = data_from_file
                .files
                .iter()
                .map(|file: &String| {
                    html!{
                        <div>
                            {file}
                        </div>
                    }
                })
                .collect ();
            html! {
                <>
                    <p>{ data_from_file.value }</p>
                    <p>{ format!("There are {} files", data_from_file.files.len()) }</p>
                    <p>{ files }</p>
                </>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet...." }</p>
            }
        }
    }
}

// Implementation of button component.
impl Component for Fetcher {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Fetcher {
            link,
            title: props.title,
            onsignal: props.onsignal,
            fetching: false,
            data: None,
            error: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(());
                false
            }
            Msg::FetchData(format, binary) => {
                // Numbered steps from: https://yew.rs/docs/en/next/concepts/services/fetch
                self.fetching = true;
                // 1. Build the request
                let request = Request::get("/data.json")
                    .body(Nothing)
                    .expect("Could not build request.");

                // 2. Construct a callback
                let callback = self.link.callback(
                    |response: Response<Json<Result<DataFromFile, anyhow::Error>>>| {
                        // Consume the response, returning just the body.
                        let body = response.into_body();

                        // Converts bodies JSON string to a data (lazy). 
                        let Json(data) = body;

                        // Return a fetch ready message with received data.
                        Msg::FetchReady(data)
                    },
                );

                // 3. Pass the request and callback to the fetch service.
                println!("Fetching...");
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                // 4. Store the task so it isn't canceled immediately
                self.fetch_task = Some(task);

                // We want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            Msg::FetchReady(response) => {
                // Indicate we are no longer fetching
                self.fetching = false;

                // There is no fetch_task running anymore.
                self.fetch_task = None;

                match response {
                    Ok(location) => {
                        self.data = Some(location);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }

                // We want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, false))>
                        { "Fetch Data" }
                    </button>
                    { self.view_data() }
            </div>
        }
    }
}
