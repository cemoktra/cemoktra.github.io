use leptos::prelude::*;
use thaw::*;

mod cv;

use cv::Cv;

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());

    view! {
        <ConfigProvider theme attr:style="position: absolute; left: 0; top: 0; right: 0; bottom: 0">
            <Flex justify=FlexJustify::Center>
                <Cv/>
            </Flex>
        </ConfigProvider>
    }
}
