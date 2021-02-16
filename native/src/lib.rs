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
            let path = cx.string(index_file_name.to_owned());
            this.set(&mut cx, "index_path", path);
            Ok(cx.string(index_file_name).upcast())
        }

        method paginate(mut cx) {
            let this = cx.this();
            let start = cx.argument::<JsNumber>(0)?;
            let limit = cx.argument::<JsNumber>(1)?;

            println!("slice at {:?}", start.value() as u32);

            // let getPaths = |cx: CallContext<JsCsv> | {
            //     let guard = cx.lock();
            //     let csv = this.borrow(&guard);
            //     return (csv.file_path.clone(), csv.index_path.clone())
            // };

            // let (reader_path, index_path) = getPaths(cx);


            // let reader_path = this
            //     .get(&mut cx, "file_path")?
            //     .downcast::<JsString>().or_throw(&mut cx)?
            //     .value();

            let reader_path = {
                let guard = cx.lock();
                let path = this.borrow(&guard).file_path.to_owned(); path
            };

            println!("file: {:?}", reader_path);

            let index_path = this
                .get(&mut cx, "index_path")?
                .downcast::<JsString>().or_throw(&mut cx)?
                .value();

            println!("file: {:?}", index_path);

            let mut reader = csv::Reader::from_path(reader_path).unwrap();
            let mut index = RandomAccessSimple::open(File::open(&index_path).unwrap()).unwrap();

            let pos = index.get(start.value() as u64).unwrap();
            reader.seek(pos);
            let rows: Handle<JsArray> = JsArray::new(&mut cx, limit.value() as u32);

            for i in 0..limit.value() as i64 {
                let row = reader.records().next().unwrap().unwrap();
                let row_values: Handle<JsArray> = JsArray::new(&mut cx, row.len() as u32);
                let mut p = 0;
                for field in row.iter() {
                    let field_value = cx.string(field.to_string());
                    row_values.set(&mut cx, p, field_value);
                    p = p + 1;
                }
                rows.set(&mut cx, i as u32, row_values).unwrap();
            }

            Ok(rows.upcast())
        }
    }
}

register_module!(mut m, { m.export_class::<JsCsv>("CsvFile") });
