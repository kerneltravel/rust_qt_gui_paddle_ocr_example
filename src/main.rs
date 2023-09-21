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

//================================================================
///#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'。
///要求 #[macro_use] 必须在 root crate 中定义。 也就是在 main.rs中写下面两句，启用。
#[macro_use]
extern crate rbatis;
/// macro_use是 Rust 中的一个编译时宏，用于在代码中使用其他宏。
/// 例如，假设有一个名为macro_x的宏，它定义了一段代码：
/// fn macro_x() {
/// println!("This is macro_x");
///}
/// 如果要在代码中使用macro_x宏，可以这样写：
/// macro_use! { macro_x };
//================================================================
mod filemd5;
mod ocr;
mod pdf;
mod sqlops;
mod uic;

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
