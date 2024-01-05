export * from "./single"
export * from "./bulk"

export type DatasetEntry<T> = {
    label: string,
    arg: string,
    expected: T
}

export type Dataset<T> = DatasetEntry<T>[];
