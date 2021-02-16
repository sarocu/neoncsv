const addon = require('../native');

const {
    CsvFile,
} = addon;

const path = "/home/sam/Projects/webcsv/tests/addresses.csv"
const csv = new CsvFile(path);
const index = csv.create_index_sync(path);
console.log(index)

const rows = csv.paginate(10000, 20);
console.log(rows)