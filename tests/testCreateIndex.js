const neoncsv = require('../lib/build/index')

const path = "/home/sam/Projects/webcsv/tests/robots.csv"
const csv = new neoncsv.CsvFile(path);
csv.createIndex()

const rows = csv.paginate(0,100);
console.log(rows)