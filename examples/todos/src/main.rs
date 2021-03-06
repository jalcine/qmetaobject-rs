extern crate qmetaobject;
use qmetaobject::*;

use std::ffi::CStr;

mod implementation;

qrc!(my_ressource,
    "todos/qml" {
      //  "../main.qml" as "main.qml",
        "main.qml",
    },
);

fn main() {
    my_ressource();
    qml_register_type::<implementation::Todos>(CStr::from_bytes_with_nul(b"RustCode\0").unwrap(), 1, 0,
        CStr::from_bytes_with_nul(b"Todos\0").unwrap());
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/todos/qml/main.qml".into());
    engine.exec();
}
