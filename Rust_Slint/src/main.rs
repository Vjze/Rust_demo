#![windows_subsystem = "windows"]

use tool_slint::{datetime, get_result, *};

fn main() {
    let app = App::new().unwrap();
    let cargo_worker = get_result::CargoWorker::new(&app);
    let _timer = datetime::setup(&app);

    app.global::<InfosData>().on_query({
        let window = app.as_weak();
        let cargo_channel = cargo_worker.channel.clone();
        move |action| {
            if action.text.len() == 0 {
                // window.unwrap().invoke_show_confirm_popup();
                window.unwrap().invoke_show_tip();
            } else {
                cargo_channel
                    .send(get_result::QueryMessage::Action { action })
                    .unwrap()
            }
        }
    });

    app.run().unwrap();
    cargo_worker.join().unwrap();
}