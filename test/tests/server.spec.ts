import { Subprocess } from 'bun';
import { afterAll, beforeAll } from 'bun:test';

import { describe_for } from '../src/test_implementation';

const TEST_RELEASE = true;

let server: Subprocess | undefined;

beforeAll(async () => {
    console.log('Building server...');

    await new Promise<void>((resolve) => {
        Bun.spawn(['cargo', 'build', TEST_RELEASE ? '--release' : ''], {
            cwd: '../server',
            onExit() {
                resolve();
            },
        });
    });

    console.log('Build finished!');

    server = Bun.spawn([`../server/target/${TEST_RELEASE ? 'release' : 'debug'}/enstate`], {
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

describe_for('server', 'http://127.0.0.1:3000');
