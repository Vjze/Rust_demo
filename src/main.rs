mod get_result;
mod sql;
mod util;
use std::{
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
};

use slint::{SharedString, StandardListViewItem, VecModel, Weak};
use sql::{box_work, carton_work, sn_work};

slint::include_modules!();

// #[tokio::main]
fn main() {
    let app = App::new().unwrap();
    // let run = Run::new(&app);
    let app_weak = app.as_weak();
    app.global::<TableView>().on_query({
        
        
        move |action| {
            
             
                
                if action.r#type.as_str() == "SN" {
                    let app_weak = app_weak.clone();
                     get_sn_result(action.text.to_string(),app_weak);
                    
                    
                };
                if action.r#type.as_str() == "Box" {
                    let app_weak = app_weak.clone();
                    get_box_result(action.text.to_string(),app_weak);
                    // app_weak
                    //     .unwrap()
                    //     .global::<TableView>()
                    //     .set_row_data(res.into());
                };
                if action.r#type.as_str() == "Carton" {
                    let app_weak = app_weak.clone();
                    get_carton_result(action.text.to_string(),app_weak);
                    // app_weak
                    //     .unwrap()
                    //     .global::<TableView>()
                    //     .set_row_data(res.into());
                };
            }
        });
   

    app.run().unwrap();
}

fn get_sn_result(text: String,app_weak:Weak<App>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async {
        let row_data = sn_work(text).await;
        app_weak
                        .unwrap()
                        .global::<TableView>()
                        .set_row_data(row_data.into());
    });
    // res
}

fn get_box_result(text: String,app_weak:Weak<App>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async {
        let row_data = box_work(text).await;
        app_weak
        .unwrap()
        .global::<TableView>()
        .set_row_data(row_data.into());
    });
    // res
}

fn get_carton_result(text: String,app_weak:Weak<App>)  {
    let rt = tokio::runtime::Runtime::new().unwrap();
   let res =  rt.block_on(async {
        let row_data = carton_work(text).await;
        app_weak
        .unwrap()
        .global::<TableView>()
        .set_row_data(row_data.into());
        // app_weak
        //     .unwrap()
        //     .global::<TableView>()
        //     .set_row_data(row_data.into());
    });
//     res
}
