const neoncsv = require('../native/index.node')

export class CsvFile {
    native: any

    constructor(filePath: string) {
        this.native = neoncsv.csvFileNew(filePath);
    }

    public createIndex(): string {
        return this.native.create_index_sync()
    }

    public paginate(start: number, limit: number): string[] {
        return this.native.paginate(start, limit)
    }
}