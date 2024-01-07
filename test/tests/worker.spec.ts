import { Subprocess } from 'bun';
import { afterAll, beforeAll } from 'bun:test';

import { dataset_address_bulk, dataset_name_bulk, dataset_universal_bulk } from '../data';
import {
    dataset_address_single,
    dataset_name_single,
    dataset_universal_single,
} from '../data/single';
import { http_fetch } from '../src/http_fetch';
import { test_implementation } from '../src/test_implementation';

let server: Subprocess<'ignore', 'pipe', 'inherit'> | undefined;

beforeAll(async () => {
    console.log('Building worker...');

    server = Bun.spawn(['wrangler', 'dev', '--port', '3000'], { cwd: '../worker' });

    console.log('Waiting for server to start...');

    let attempts = 0;

    // TODO: fix
    // eslint-disable-next-line no-constant-condition
    while (true) {
        try {
            console.log('Attempting heartbeat...');
            await fetch('http://0.0.0.0:3000/');
            console.log('Heartbeat succes!');
            break;
        } catch {
            console.log('Waiting another 4s for heartbeat...');
            attempts++;
            await new Promise<void>((resolve) => setTimeout(resolve, 1000));
            continue;
        }
    }

    console.log('Ready to start testing');
});

afterAll(async () => {
    server?.kill();

    await server?.exited;
});

test_implementation('worker/name', http_fetch('http://0.0.0.0:3000/n/'), dataset_name_single);
test_implementation('worker/address', http_fetch('http://0.0.0.0:3000/a/'), dataset_address_single);
test_implementation(
    'worker/universal',
    http_fetch('http://0.0.0.0:3000/u/'),
    dataset_universal_single
);

test_implementation(
    'worker/bulk/name',
    http_fetch('http://0.0.0.0:3000/bulk/n?'),
    dataset_name_bulk
);
test_implementation(
    'worker/bulk/address',
    http_fetch('http://0.0.0.0:3000/bulk/a?'),
    dataset_address_bulk
);
test_implementation(
    'worker/bulk/universal',
    http_fetch('http://0.0.0.0:3000/bulk/u?'),
    dataset_universal_bulk
);
