use gloo_timers::future::TimeoutFuture;
use yew::{html, AttrValue, Component, Html, Properties};

#[derive(Debug)]
pub enum DisplayMessage {
    Loaded,
    NewSrc(Option<String>),
}

#[derive(Debug, PartialEq, Properties)]
pub struct DisplayImageProps {
    pub src: Option<AttrValue>,
}

pub struct DisplayImage {
    loaded: bool,
    src: Option<String>,
}

impl Component for DisplayImage {
    type Message = DisplayMessage;

    type Properties = DisplayImageProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            loaded: false,
            src: ctx.props().src.as_ref().map(|attr| attr.to_string()),
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        self.loaded = false;

        let new_src = ctx.props().src.as_ref().map(|attr| attr.to_string());
        let is_transition = self.src.is_some();

        ctx.link().send_future(async move {
            if is_transition {
                // This must match the value in styles/display/display-image.scss
                TimeoutFuture::new(500).await;
            }
            DisplayMessage::NewSrc(new_src)
        });

        true
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DisplayMessage::Loaded => self.loaded = true,
            DisplayMessage::NewSrc(src) => self.src = src,
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let class = if self.loaded {
            "display-image"
        } else {
            "display-image-invis"
        };
        let onload = ctx.link().callback(|_| DisplayMessage::Loaded);

        if let Some(src) = &self.src {
            html! {
                <img src={format!("images/{}", src)} {class} {onload}/>
            }
        } else {
            html! {
                <img class="display-image-invis" />
            }
        }
    }
}
