import { describe, expect, test } from 'bun:test';

import { Dataset } from '../data';

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
