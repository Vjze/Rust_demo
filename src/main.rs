#![windows_subsystem = "windows"]

use tool_slint::{*, get_result,datetime};

fn main() {
    let app = App::new().unwrap();
    let cargo_worker = get_result::CargoWorker::new(&app);
    // #[cfg(feature = "chrono")]
    let _timer = datetime::setup(&app);

    app.global::<InfosData>().on_query({
        let cargo_channel = cargo_worker.channel.clone();
        move |action| {
            cargo_channel
                .send(get_result::QueryMessage::Action {
                    action,
                })
                .unwrap()
        }
    });
    
    app.run().unwrap();
    cargo_worker.join().unwrap();
}