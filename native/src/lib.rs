use neon::prelude::*;

extern crate csv;
// use csv::{Position, Reader};
use csv_index::RandomAccessSimple;

use std::fs::File;
use std::io;
use std::path::Path;

pub struct CsvFile {
    index_path: String,
    file_path: String,
}

declare_types! {
    pub class JsCsv for CsvFile {
        init(mut cx) {
            let file_path: Handle<JsString> = cx.argument::<JsString>(0)?;
            println!("path: {}", file_path.value());
            // let this = cx.this();
            // let index_path = create_index_sync(file_path);

            Ok(CsvFile {
                file_path: file_path.value(),
                index_path: cx.string("").value(),
            })
        }

        method create_index_sync(mut cx) {
            let this = cx.this();
            let file_path = cx.argument::<JsString>(0)?;
            
            let reader = csv::Reader::from_path(file_path.value());
            let mut index_file_name: String = file_path.value();
            index_file_name.push_str(&".idx");
            let index_path = Path::new(&index_file_name);
            println!("index path: {:?}", &index_path);
            let new_index_file = File::create(&index_path).unwrap();
            let mut index_file = io::BufWriter::new(new_index_file);
            csv_index::RandomAccessSimple::create(&mut reader.unwrap(), &mut index_file);

            csv_index::RandomAccessSimple::open(File::open(&index_file_name).unwrap());
            let path = cx.string(&index_file_name);
            this.set(&mut cx, "index_path", path);
            Ok(cx.string(index_file_name).upcast())
        }

        method slice(mut cx) {
            let this = cx.this();
            let start = cx.argument::<JsNumber>(0)?;
            let end = cx.argument::<JsNumber>(1)?;

            println!("slice at {:?}", start.value());

            let reader_path = this
                .get(&mut cx, "index_path")?
                .downcast::<JsString>().or_throw(&mut cx)?
                .value();

            println!("index: {:?}", reader_path);

            // TODO: set the seek and return a range of rows
            
            Ok(cx.string("cool").upcast())
        }
    }
}

// impl CsvFile {
//     pub fn create_index_file(file_path: String) -> Result<String, JsValue> {
//         let index = CsvFile::create_index_sync(file_path);

//         let mut index = match index {
//             Ok(i) => JsValue::from("success"),
//             Err(e) => JsValue::from("couldn't create index"),
//         };
//         Ok("success".to_string())
//     }
// }

// register_module!(mut cx, {
//     cx.export_function("hello", hello)
// });

register_module!(mut m, { m.export_class::<JsCsv>("CsvFile") });
