use crate::generated_code::{App,Action,Infos_Box,Infos_Sn,InfosData,Example};
use crate::logic::sql::box_work;
use futures::future::{Fuse, FutureExt};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::sql::sn_work;

#[derive(Debug)]
pub enum QueryMessage {
    Action { action: Action },
    Quit,
    ShowDialog(String)
}

pub struct CargoWorker {
    pub channel: UnboundedSender<QueryMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl CargoWorker {
    pub fn new(query: &App) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({
            let handle_weak = query.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(query_worker_loop(r, handle_weak))
                    .unwrap()
            }
        });
        Self {
            channel,
            worker_thread,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        let _ = self.channel.send(QueryMessage::Quit);
        self.worker_thread.join()
    }
}

async fn query_worker_loop(
    mut r: UnboundedReceiver<QueryMessage>,
    handle: slint::Weak<App>,
) -> tokio::io::Result<()> {
    let run_cargo_future = Fuse::terminated();
    futures::pin_mut!(run_cargo_future,);
    loop {
        let m = futures::select! {

                res = run_cargo_future => {
                    res?;
                    continue;
                }

            m = r.recv().fuse() => {
                match m {
                    None => return Ok(()),
                    Some(m) => m,
                }
            }
        };
        match m {
            QueryMessage::Action { action } => {
                run_cargo_future.set(run_query(action,handle.clone()).fuse())
            }
            QueryMessage::Quit => return Ok(()),
            QueryMessage::ShowDialog(_) => todo!(),
        }
    }
}

async fn run_query(action: Action,handle: slint::Weak<App>) -> tokio::io::Result<()> {
    if action.r#type == "SN" {
        let v = sn_work(action.text.to_string()).await;
        let table = v.0.clone();
        let nub = v.1.clone().to_string();
        handle
            .upgrade_in_event_loop(move |ui| {
                ui.global::<InfosData>().set_datas_sn(ModelRc::from(
                    Rc::new(VecModel::from(table)) as Rc<dyn Model<Data = Infos_Sn>>
                ));
                ui.global::<InfosData>().set_text(nub.into());
            })
            .unwrap();
    }else if action.r#type == "Box" {
        let v = box_work(action.text.to_string()).await;
        let table = v.0.clone();
        let nub = v.1.clone().to_string();
        handle
            .upgrade_in_event_loop(move |ui| {
                ui.global::<InfosData>().set_datas_box(ModelRc::from(
                    Rc::new(VecModel::from(table)) as Rc<dyn Model<Data = Infos_Box>>
                ));
                ui.global::<InfosData>().set_text(nub.into());
            })
            .unwrap();
        }
    
    Ok(())
}
