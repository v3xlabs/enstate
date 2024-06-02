import { describe_for } from '../src/test_implementation';

const url = process.env.REMOTE_URL ?? 'https://enstate.rs';

describe_for(`remote (${url})`, url);
