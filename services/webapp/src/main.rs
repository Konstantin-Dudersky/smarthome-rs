use leptos::*;
use leptos_router::*;

use messages::{self, types, Messages};
use webapp_lib::{
    components::{Button, State},
    define_window_url, handle_ws_connection,
};

use webapp::{process_ws_message, ApplicationShell, GlobalNavigation, MsgData};

#[component]
fn App() -> impl IntoView {
    let gs = use_context::<MsgData>().expect("no global state");

    let command_start = move || {
        // let msg = Messages::CommandStart(types::Command::default());
        // gs.send_msg.set(Some(msg));
    };

    let command_stop = move || {
        // let msg = Messages::CommandStop(types::Command::default());
        // gs.send_msg.set(Some(msg));
    };

    view! {
        <div class="container mx-auto">

            <div class="flex flex-row">
                <div class="basis-1/2">
                    <p class="m-4">
                        Температура
                    </p>
                </div>
                <div class="basis-1/2">
                    <p class="m-4">
                        { Signal::derive(move|| gs.room_temperature.get().value) }
                    </p>
                </div>
            </div>

            <div class="flex flex-row">
                <div class="basis-1/2">
                    <p class="m-4">
                        Влажность
                    </p>
                </div>
                <div class="basis-1/2">
                    <p class="m-4">
                        { Signal::derive(move|| gs.room_humidity.get().value) }
                    </p>
                </div>
            </div>

            <div class="flex flex-row">
                <div class="basis-1/2">
                    <p class="m-4">
                        Давление
                    </p>
                </div>
                <div class="basis-1/2">
                    <p class="m-4">
                        { Signal::derive(move|| gs.room_pressure.get().value) }
                    </p>
                </div>
            </div>

        </div>
    }
}

#[component]
fn App2() -> impl IntoView {
    view! {
        <i class="fa-brands fa-github-square"></i>
    }
}

pub fn main() {
    provide_context(MsgData::default());
    let global_state = use_context::<MsgData>().expect("no global state");

    let window_url = define_window_url().expect("Не удалось определить URL окна");
    global_state.window_url.set(Some(window_url.clone()));

    // create_resource(
    //     move || (global_state.send_msg.get(), global_state.api_url.get()),
    //     |(send_msg, api_url)| async move {
    //         // if let Some(send_msg) = send_msg {
    //         //     api::send_message_to_api(&api_url, send_msg).await;
    //         // }
    //     },
    // );

    // let ws_url = format!("ws://{}:8081", window_url.host().unwrap());
    let ws_url = "ws://target:8081";
    spawn_local(async move {
        handle_ws_connection(&ws_url, process_ws_message).await;
    });

    mount_to_body(|| {
        view! {
            <Router>
                <ApplicationShell
                    navigation=GlobalNavigation
                    content=|| view!(
                        <Routes>
                            <Route path="/" view=App/>
                            <Route path="/app2" view=App2/>
                        </Routes>
                    )
                />
            </Router>
        }
    })
}
