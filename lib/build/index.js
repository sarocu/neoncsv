"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CsvFile = void 0;
const neoncsv = require('../../native/index.node');
class CsvFile {
    constructor(filePath) {
        this.native = neoncsv.csvFileNew(filePath);
    }
    createIndex() {
        return this.native.create_index_sync();
    }
    paginate(start, limit) {
        return this.native.paginate(start, limit);
    }
}
exports.CsvFile = CsvFile;
