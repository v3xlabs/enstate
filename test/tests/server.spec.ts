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

const TEST_RELEASE = true;

let server: Subprocess | undefined;

beforeAll(async () => {
    server = Bun.spawn([`server/target/${TEST_RELEASE ? 'release' : 'debug'}/enstate`], {
        cwd: '../server',
    });

    console.log('Waiting for server to start...');

    let attempts = 0;

    while (attempts < 10) {
        try {
            console.log('Attempting heartbeat...');
            await fetch('http://0.0.0.0:3000/');
            console.log('Heartbeat succes!');
            break;
        } catch {
            console.log('Waiting another 1s for heartbeat...');
            attempts++;
            await new Promise<void>((resolve) => setTimeout(resolve, 1000));
            continue;
        }
    }

    console.log('Ready to start testing');
});

afterAll(async () => {
    server?.kill();
});

const PREFIX = 'server';

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
