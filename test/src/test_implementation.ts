import { describe, expect, test } from 'bun:test';

import {
    Dataset,
    dataset_address_bulk,
    dataset_address_single,
    dataset_name_bulk,
    dataset_name_single,
    dataset_universal_bulk,
    dataset_universal_single,
} from '../data';
import { http_fetch } from './http_fetch';

export const test_implementation = <DataSet extends Dataset<DataType>, DataType extends {}>(
    function_name: string,
    resolve_function: (_input: string) => Promise<Partial<DataType>>,
    dataset: DataSet
) => {
    describe('t/' + function_name, () => {
        for (const { label, arg, expected } of dataset) {
            test(label + ` (${arg})`, async () => {
                const output = await resolve_function(arg);

                expect(output).toMatchObject(expected);
            });
        }
    });
};

export const describe_for = (prefix: string, base_url: string) => {
    test_implementation(`${prefix}/name`, http_fetch(`${base_url}/n/`), dataset_name_single);
    test_implementation(`${prefix}/address`, http_fetch(`${base_url}/a/`), dataset_address_single);
    test_implementation(
        `${prefix}/universal`,
        http_fetch(`${base_url}/u/`),
        dataset_universal_single
    );
    test_implementation(
        `${prefix}/bulk/name`,
        http_fetch(`${base_url}/bulk/n?`),
        dataset_name_bulk
    );
    test_implementation(
        `${prefix}/bulk/address`,
        http_fetch(`${base_url}/bulk/a?`),
        dataset_address_bulk
    );
    test_implementation(
        `${prefix}/bulk/universal`,
        http_fetch(`${base_url}/bulk/u?`),
        dataset_universal_bulk
    );
};
