import { Subprocess } from 'bun';
import { afterAll, beforeAll } from 'bun:test';

import { describe_for } from '../src/test_implementation';

let server: Subprocess<'ignore', 'pipe', 'inherit'> | undefined;

beforeAll(async () => {
    console.log('Building worker...');

    server = Bun.spawn(['pnpm', 'dev', '--port', '3000'], { cwd: '../worker' });

    console.log('Waiting for server to start...');

    // TODO: fix
    // eslint-disable-next-line no-constant-condition
    while (true) {
        try {
            console.log('Attempting heartbeat...');
            await fetch('http://0.0.0.0:3000/');
            console.log('Heartbeat succes!');
            await new Promise<void>((resolve) => setTimeout(resolve, 2000));
            break;
        } catch {
            console.log('Waiting another 5s for heartbeat...');
            await new Promise<void>((resolve) => setTimeout(resolve, 5000));
            continue;
        }
    }

    console.log('Ready to start testing');
});

afterAll(async () => {
    server?.kill();

    await server?.exited;
});

describe_for('worker', 'http://127.0.0.1:3000');
