export * from './bulk';
export * from './single';

export type DatasetEntry<T> = {
    label: string;
    arg: string;
    expected: T;
};

export type Dataset<T> = DatasetEntry<T>[];
