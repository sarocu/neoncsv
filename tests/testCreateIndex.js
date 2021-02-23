const neoncsv = require('../native');

const {
    CsvFile,
} = neoncsv;

const path = "/home/sam/Projects/webcsv/tests/robots.csv"
const csv = new CsvFile(path);
const index = csv.create_index_sync(path);

const rows = csv.paginate(0,100);
console.log(rows)