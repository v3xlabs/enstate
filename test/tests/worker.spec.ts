import { expect, test, describe, beforeAll, afterAll } from "bun:test";
import { test_implementation } from "../src/test_implementation";
import { Subprocess } from "bun";
import { dataset_address_single, dataset_name_single, dataset_universal_single } from "../data/single";
import { http_fetch } from "../src/http_fetch";

let server: Subprocess | undefined = undefined;

beforeAll(async () => {
    console.log("Building worker...");

    server = Bun.spawn(['pnpm', 'dev', '--port', '3000'], { cwd: "../worker" });

    console.log('Waiting for server to start...');

    let attempts = 0;
    while (attempts < 30) {
        try {
            console.log("Attempting heartbeat...");
            await fetch("http://0.0.0.0:3000/");
            console.log("Heartbeat succes!");
            break;
        } catch (e) {
            console.log("Waiting another 1s for heartbeat...");
            attempts++;
            await new Promise<void>((resolve) => setTimeout(resolve, 1000));
            continue;
        }
    }

    console.log("Ready to start testing");
});

afterAll(async () => {
    server?.kill();

    await server?.exited;
});

test_implementation("worker/name", http_fetch("http://0.0.0.0:3001/n/"), dataset_name_single);
test_implementation("worker/address", http_fetch("http://0.0.0.0:3001/n/"), dataset_address_single);
test_implementation("worker/universal", http_fetch("http://0.0.0.0:3001/n/"), dataset_universal_single);
