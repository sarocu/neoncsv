const neoncsv = require('../native');

const {
    CsvFile,
} = neoncsv;

const path = "/home/sam/Projects/webcsv/tests/robots.csv"
const csv = new CsvFile(path);

csv.set_index("/home/sam/Projects/webcsv/tests/robots.csv.idx")

const f = (row) => {
    return row.map((item) => {
        item.toLowerCase();
    })
}

csv.mapLambda(f, "/home/sam/Projects/webcsv/tests/robotsLower.csv")
