import { randomBytes } from 'node:crypto';

import { Dataset } from '.';

export const dataset_name_single: Dataset<{ address: string } | { status: number }> = [
    {
        label: 'ETHRegistry',
        arg: 'luc.eth',
        expected: { address: '0x225f137127d9067788314bc7fcc1f36746a3c3B5' },
    },
    {
        label: 'ETHRegistry',
        arg: 'nick.eth',
        expected: { address: '0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5' },
    },
    {
        label: 'ETHRegistry (not found)',
        arg: randomBytes(8).toString('hex') + '.eth',
        expected: { status: 404 },
    },
    {
        label: 'DNSRegistry',
        arg: 'luc.computer',
        expected: { address: '0x225f137127d9067788314bc7fcc1f36746a3c3B5' },
    },
    {
        label: 'DNSRegistry',
        arg: 'antony.sh',
        expected: { address: '0x2B5c7025998f88550Ef2fEce8bf87935f542C190' },
    },
    {
        label: 'DNSRegistry (offchain DNSSEC)',
        arg: 'antony.cash',
        expected: { address: '0x2B5c7025998f88550Ef2fEce8bf87935f542C190' },
    },
    {
        label: 'DNSRegistry (not found)',
        arg: randomBytes(8).toString('hex') + '.com',
        expected: { status: 404 },
    },
    {
        label: 'CCIP Offchain RS',
        arg: 'luc.willbreak.eth',
        expected: { address: '0x225f137127d9067788314bc7fcc1f36746a3c3B5' },
    },
    {
        label: 'CCIP Coinbase',
        arg: 'lucemans.cb.id',
        expected: { address: '0x4e7abb71BEe38011c54c30D0130c0c71Da09222b' },
    },
];

export const dataset_address_single: Dataset<{ name: string } | { status: number }> = [
    {
        label: 'Error (bad address)',
        arg: 'hi',
        expected: { status: 400 },
    },
    {
        label: 'Error (address not found)',
        arg: '0x000000000000000000000000000000000000ffff',
        expected: { status: 404 },
    },
    {
        label: 'ETHRegistry',
        arg: '0x225f137127d9067788314bc7fcc1f36746a3c3B5',
        expected: { name: 'luc.eth' },
    },
    {
        label: 'ETHRegistry',
        arg: '0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5',
        expected: { name: 'nick.eth' },
    },
    // TODO: find another dns primary name address
    // {
    //     label: 'DNSRegistry',
    //     arg: '0x225f137127d9067788314bc7fcc1f36746a3c3B5',
    //     expected: { name: 'luc.computer' },
    // },
    {
        label: 'DNSRegistry',
        arg: '0x2B5c7025998f88550Ef2fEce8bf87935f542C190',
        expected: { name: 'antony.sh' },
    },
    // TODO: find 2 ccip primary name addresses
    // {
    //     label: 'CCIP Offchain RS',
    //     arg: '0x225f137127d9067788314bc7fcc1f36746a3c3B5',
    //     expected: { name: 'luc.willbreak.eth' },
    // },
    // {
    //     label: 'CCIP Coinbase',
    //     arg: '0x4e7abb71BEe38011c54c30D0130c0c71Da09222b',
    //     expected: { name: 'lucemans.cb.id' },
    // },
];

export const dataset_universal_single: Dataset<
    { address: string } | { name: string } | { status: number }
> = [
    {
        label: 'Error (not found)',
        arg: '0x000000000000000000000000000000000000ffff',
        expected: { status: 404 },
    },
    {
        label: 'ETHRegistry',
        arg: 'luc.eth',
        expected: { address: '0x225f137127d9067788314bc7fcc1f36746a3c3B5' },
    },
    {
        label: 'ETHRegistry',
        arg: '0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5',
        expected: { name: 'nick.eth' },
    },
    {
        label: 'ETHRegistry (not found)',
        arg: randomBytes(8).toString('hex') + '.eth',
        expected: { status: 404 },
    },
    {
        label: 'DNSRegistry',
        arg: 'luc.computer',
        expected: { address: '0x225f137127d9067788314bc7fcc1f36746a3c3B5' },
    },
    {
        label: 'DNSRegistry (offchain DNSSEC)',
        arg: 'antony.cash',
        expected: { address: '0x2B5c7025998f88550Ef2fEce8bf87935f542C190' },
    },
    {
        label: 'DNSRegistry',
        arg: '0x2B5c7025998f88550Ef2fEce8bf87935f542C190',
        expected: { name: 'antony.sh' },
    },
    {
        label: 'DNSRegistry (not found)',
        arg: randomBytes(8).toString('hex') + '.com',
        expected: { status: 404 },
    },
    {
        label: 'CCIP Offchain RS',
        arg: 'luc.willbreak.eth',
        expected: { address: '0x225f137127d9067788314bc7fcc1f36746a3c3B5' },
    },
    // TODO: refer to above todo
    // {
    //     label: 'CCIP Coinbase',
    //     arg: '0x4e7abb71BEe38011c54c30D0130c0c71Da09222b',
    //     expected: { name: 'lucemans.cb.id' },
    // },
];
