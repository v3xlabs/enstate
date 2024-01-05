import { describe, expect, test } from "bun:test";
import { Dataset } from "../data/basic";

export const test_implementation = <DataSet extends Dataset<DataType>, DataType extends {}>(function_name: string, fn: (input: string) => Promise<Partial<DataType>>, dataset: DataSet) => {
    describe("t/" + function_name, () => {
        for (const { label, arg, expected } of dataset) {
            test(label + ` (${arg})`, async () => {
                let output = await fn(arg);

                expect(output).toMatchObject(expected);
            });
        }
    });
};
