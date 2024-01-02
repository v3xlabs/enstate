import { describe, expect, test } from "bun:test";
import { dataset_names_basic } from "../data/basic";

export const test_implementation = <DataSet extends [string, string, Partial<DataType>][], DataType extends {}>(function_name: string, x: (input: string) => Promise<Partial<DataType>>, dataset: DataSet) => {
    describe("t/" + function_name, () => {
        for (const [label, input, partial] of dataset) {
            test(label + ` (${input})`, async () => {
                let output = await x(input);

                expect(output).toMatchObject(partial);
            });
        }
    });
};
