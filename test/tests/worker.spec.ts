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

    // TODO: fix
    // eslint-disable-next-line no-constant-condition
    while (true) {
        try {
            console.log('Attempting heartbeat...');
            await fetch('http://0.0.0.0:3000/');
            console.log('Heartbeat succes!');
            break;
        } catch {
            console.log('Waiting another 1s for heartbeat...');
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

const PREFIX = 'worker';

test_implementation(`${PREFIX}/name`, http_fetch('http://0.0.0.0:3000/n/'), dataset_name_single);
test_implementation(
    `${PREFIX}/address`,
    http_fetch('http://0.0.0.0:3000/a/'),
    dataset_address_single
);
test_implementation(
    `${PREFIX}/universal`,
    http_fetch('http://0.0.0.0:3000/u/'),
    dataset_universal_single
);

test_implementation(
    `${PREFIX}/bulk/name`,
    http_fetch('http://0.0.0.0:3000/bulk/n?'),
    dataset_name_bulk
);
test_implementation(
    `${PREFIX}/bulk/address`,
    http_fetch('http://0.0.0.0:3000/bulk/a?'),
    dataset_address_bulk
);
test_implementation(
    `${PREFIX}/bulk/universal`,
    http_fetch('http://0.0.0.0:3000/bulk/u?'),
    dataset_universal_bulk
);
