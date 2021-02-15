const addon = require('../native');

const {
    CsvFile,
} = addon;

const path = "/home/sam/Projects/webcsv/tests/addresses.csv"
const csv = new CsvFile(path);
const index = csv.create_index_sync(path);
console.log(index)

const isCool = csv.slice(10, 20);
console.log(isCool)