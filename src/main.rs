#![windows_subsystem = "console"] //要显示控制台时，仅启用这句
                                  //#![windows_subsystem = "windows"]  // 要隐藏时，仅启用这句

use cpp_core::{CastInto, NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, slot, QDateTime, QObject, QString, SlotNoArgs};
use qt_widgets::{QApplication, QFileDialog, QMessageBox, QWidget};
use serde::{Deserialize, Serialize};
use serde_json;
use std::future::Future;
use std::thread;
use std::{rc::Rc, str::FromStr};
mod winhelloworld;
use winhelloworld::HelloWorld;
//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;

mod filemd5;
mod ocr;
mod pdf;
mod sqlops;
mod uic;

/// may also write `CRUDTable` as `impl CRUDTable for BizActivity{}`
/// #[crud_table]
/// #[crud_table(table_name:biz_activity)]
/// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag")]
/// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]

#[tokio::main]
pub async fn main() {
    let out = sqlops::mytest_db_crud().await;
    println!("来自结果异步函数的返回结果 {:?}", out);
    QApplication::init(|app| unsafe {
        //根据 https://github.com/jnbooth/ruic 的说法，
        //in Rust+Qt, the way to create a parentless widget is to pass NullPtr as the parent.
        let _form = HelloWorld::new(NullPtr);
        _form.ui.btn_quit.clicked().connect(app.slot_quit());
        _form.show();
        QApplication::exec()
    })
}
