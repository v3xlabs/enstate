import { Dataset } from ".";
import qs from "qs";

export const dataset_name_bulk: Dataset<{response: {address: string}[], response_length: number }> = [{
    label: "ETHRegistry", arg: qs.stringify({names: ["luc.eth", "nick.eth"] }, { encode: false}), expected: {response: [{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {address: "0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5"}], response_length: 2 }},{
    label: "ETHRegistry (extra)", arg: qs.stringify({names: ["luc.eth", "nick.eth", "nick.eth"] }, { encode: false}), expected: {response: [{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {address: "0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5"}], response_length: 2 }},{
    label: "DNSRegistry", arg: qs.stringify({names: ["luc.computer", "antony.sh"] }, { encode: false}), expected: {response: [{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {address: "0x2B5c7025998f88550Ef2fEce8bf87935f542C190"}], response_length: 2 }},{
    label: "CCIP", arg: qs.stringify({names: ["luc.willbreak.eth", "lucemans.cb.id"] }, { encode: false}), expected: {response: [{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {address: "0x4e7abb71BEe38011c54c30D0130c0c71Da09222b"}], response_length: 2 }
}]

export const dataset_address_bulk: Dataset<{response: {name: string}[], response_length: number }> = [{
    label: "ETHRegistry", arg: qs.stringify({addresses: ["0x225f137127d9067788314bc7fcc1f36746a3c3B5", "0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5"] }, { encode: false}), expected: {response: [{name: "luc.eth"}, {name: "nick.eth"}], response_length: 2 }},
    {label: "ETHRegistry (extra)", arg: qs.stringify({addresses: ["0x2B5c7025998f88550Ef2fEce8bf87935f542C190", "0x2B5c7025998f88550Ef2fEce8bf87935F542c190"] }, { encode: false}), expected: {response: [{name: "antony.sh"}], response_length: 1 }},
    {label: "DNSRegistry", arg: qs.stringify({addresses: ["0x2B5c7025998f88550Ef2fEce8bf87935f542C190"] }, { encode: false}), expected: {response: [{name: "antony.sh"}], response_length: 1 }},
    // {label: "CCIP", arg: qs.stringify({names: ["luc.willbreak.eth", "lucemans.cb.id"] }, { encode: false}), expected: {response: [{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}], response_length: 2 }}
]

export const dataset_universal_bulk: Dataset<{response: ({address: string} | {name: string})[], response_length: number }> = [{
    label: "ETHRegistry", arg: qs.stringify({queries: ["luc.eth", "0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5"] }, { encode: false}), expected: {response: [{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {name: "nick.eth"}], response_length: 2 }},{
    label: "DNSRegistry", arg: qs.stringify({queries: ["0x2B5c7025998f88550Ef2fEce8bf87935f542C190", "antony.sh"] }, { encode: false}), expected: {response: [{name: "antony.sh"}, {address: "0x2B5c7025998f88550Ef2fEce8bf87935f542C190"}], response_length: 2 }},{
    label: "Mixed", arg: qs.stringify({queries: ["0x2B5c7025998f88550Ef2fEce8bf87935f542C190", "luc.eth", "luc.willbreak.eth"] }, { encode: false}), expected: {response: [{name: "antony.sh"},{address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}, {address: "0x225f137127d9067788314bc7fcc1f36746a3c3B5"}], response_length: 3 }
}]