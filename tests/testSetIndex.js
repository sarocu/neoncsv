const neoncsv = require('../native');

const {
    CsvFile,
} = neoncsv;

const path = "/home/sam/Projects/webcsv/tests/robots.csv"
const csv = new CsvFile(path);
csv.set_index("/home/sam/Projects/webcsv/tests/robots.csv.idx")

const rows = csv.paginate(0,100);
console.log(rows)

