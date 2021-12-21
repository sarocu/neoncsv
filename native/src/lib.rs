extern crate csv;
// use csv::{Position, Reader};
use csv_index::RandomAccessSimple;

use neon::prelude::*;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Clone)]

pub struct CsvFile {
    index_path: String,
    file_path: String,
}

impl CsvFile {
    fn new(index_path: String, file_path: String) -> Self {
        Self {
            index_path,
            file_path,
        }
    }
}

impl Finalize for CsvFile {}

fn csvfile_new(mut cx: FunctionContext) -> JsResult<JsBox<CsvFile>> {
    let file_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let index_path = file_path.to_string() + &".idx";

    let csv = CsvFile::new(file_path, index_path);
    Ok(cx.boxed(csv))
}

fn create_index_sync(mut cx: FunctionContext) -> JsResult<JsString> {
    let this = cx.argument::<JsBox<CsvFile>>(0)?;

    let reader = csv::Reader::from_path(&this.file_path);

    println!("index path: {:?}", &this.index_path);
    let new_index_file = File::create(&this.index_path).unwrap();
    let mut index_file = io::BufWriter::new(new_index_file);
    csv_index::RandomAccessSimple::create(&mut reader.unwrap(), &mut index_file);

    RandomAccessSimple::open(File::open(&this.index_path).unwrap());
    Ok(cx.string("success"))
}

fn paginate(mut cx: FunctionContext) -> JsResult<JsArray> {
    let this = cx.argument::<JsBox<CsvFile>>(0)?;
    let start = cx.argument::<JsNumber>(0)?;
    let limit = cx.argument::<JsNumber>(1)?.value(&mut cx).to_owned() as u32;

    let reader_path = {
        let guard = cx.lock();
        let path = this.file_path.to_owned();
        path
    };

    let mut reader = csv::Reader::from_path(reader_path).unwrap();
    let mut index = RandomAccessSimple::open(File::open(&this.index_path).unwrap()).unwrap();

    let pos = index.get(start.value(&mut cx) as u64).unwrap();
    reader.seek(pos);
    let rows: Handle<JsArray> = JsArray::new(&mut cx, limit);

    for i in 0..limit as i64 {
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

    Ok(rows)
}

// declare_types! {
//     pub class JsCsv for CsvFile {
//         init(mut cx) {
//             let file_path: Handle<JsString> = cx.argument::<JsString>(0)?;

//             Ok(CsvFile {
//                 file_path: file_path.value(&mut cx),
//                 index_path: cx.string("").value(&mut cx),
//             })
//         }

//         method create_index_sync(mut cx) {
//             let this = cx.this();
//             let file_path = cx.argument::<JsString>(0)?;

//             let reader = csv::Reader::from_path(file_path.value(&mut cx));
//             let mut index_file_name: String = file_path.value(&mut cx);
//             index_file_name.push_str(&".idx");
//             let index_path = Path::new(&index_file_name);
//             println!("index path: {:?}", &index_path);
//             let new_index_file = File::create(&index_path).unwrap();
//             let mut index_file = io::BufWriter::new(new_index_file);
//             csv_index::RandomAccessSimple::create(&mut reader.unwrap(), &mut index_file);

//             csv_index::RandomAccessSimple::open(File::open(&index_file_name).unwrap());
//             let path = cx.string(index_file_name.to_owned());
//             this.set(&mut cx, "index_path", path);
//             Ok(cx.string(index_file_name).upcast())
//         }

//         method set_index(mut cx) {
//             let this = cx.this();
//             let index_path = cx.argument::<JsString>(0)?;
//             let path = cx.string(index_path.value(&mut cx));
//             this.set(&mut cx, "index_path", path);
//             Ok(cx.string(index_path.value(&mut cx)).upcast())
//         }

//         method paginate(mut cx) {
//             let this = cx.this();
//             let start = cx.argument::<JsNumber>(0)?;
//             let limit = cx.argument::<JsNumber>(1)?;

//             let reader_path = {
//                 let guard = cx.lock();
//                 let path = this.borrow(&guard).file_path.to_owned(); path
//             };

//             let index_path = this
//                 .get(&mut cx, "index_path")?
//                 .downcast::<JsString>().or_throw(&mut cx)?
//                 .value(&mut cx);

//             let mut reader = csv::Reader::from_path(reader_path).unwrap();
//             let mut index = RandomAccessSimple::open(File::open(&index_path).unwrap()).unwrap();

//             let pos = index.get(start.value(&mut cx) as u64).unwrap();
//             reader.seek(pos);
//             let rows: Handle<JsArray> = JsArray::new(&mut cx, limit.value(&mut cx) as u32);

//             for i in 0..limit.value(&mut cx) as i64 {
//                 let row = reader.records().next().unwrap().unwrap();
//                 let row_values: Handle<JsArray> = JsArray::new(&mut cx, row.len() as u32);
//                 let mut p = 0;
//                 for field in row.iter() {
//                     let field_value = cx.string(field.to_string());
//                     row_values.set(&mut cx, p, field_value);
//                     p = p + 1;
//                 }
//                 rows.set(&mut cx, i as u32, row_values).unwrap();
//             }

//             Ok(rows.upcast())
//         }

//         method mapLambda(mut cx){
//             let this = cx.this();

//             let reader_path = {
//                 let guard = cx.lock();
//                 let path = this.borrow(&guard).file_path.to_owned(); path
//             };

//             let index_path = this
//                 .get(&mut cx, "index_path")?
//                 .downcast::<JsString>().or_throw(&mut cx)?
//                 .value(&mut cx);

//             let mut reader = csv::Reader::from_path(reader_path).unwrap();
//             let mut index = RandomAccessSimple::open(File::open(&index_path).unwrap()).unwrap();

//             // call helper function with JS func + a set of rows
//             let mapFunction = cx.argument::<JsFunction>(0)?;

//             // file name to save the transformed data to
//             let filePath = cx.argument::<JsString>(1)?;
//             let mut writer = csv::Writer::from_path(filePath.value(&mut cx)).unwrap();
//             let last = index.len() - 1;
//             let mut rowIds: Vec<u64> = (0..last).collect();

//             let null = cx.null();

//             let call_js_function = move |row: csv::StringRecord| {
//                 let mut jsRow: Vec<Handle<JsString>> = Vec::with_capacity(row.len());
//                 for field in row.iter() {
//                     jsRow.push(cx.string(field.to_string()));
//                 }
//                 let newJsRow = mapFunction.call(&mut cx, null, jsRow)?.downcast::<JsArray>().or_throw(&mut cx)?;
//                 let newRow = newJsRow.to_vec(&mut cx)?;
//                 let newRustRow: Vec<String> = newRow.iter().map(|&item| {
//                     item
//                         .downcast::<JsString>().or_throw(&mut cx).unwrap()
//                         .value(&mut cx)
//                 }).collect();
//                 Ok(vec!["things"])
//             };

//             rowIds.par_iter_mut()
//             .map(|rowId: &mut u64| {
//                 // this might be dumb to constantly be seeking in diff threads
//                 let ownedId: u64 = rowId.to_owned();
//                 let pos = index.get(ownedId).unwrap();
//                 reader.seek(pos).unwrap();
//                 let row = reader.records().next().unwrap().unwrap();
//                 let newRow = call_js_function(row).unwrap();

//                 writer.write_record(newRow).unwrap();
//                 return Ok(String::from("hi"))
//             });

//             writer.flush().unwrap();
//             Ok(filePath.upcast())
//         }

//     }
// }

// register_module!(mut m, { m.export_class::<JsCsv>("CsvFile") });
