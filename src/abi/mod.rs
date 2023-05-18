use ethers::prelude::abigen;

// Technically the PublicResolver but it implements the Resolver spec
abigen!(Resolver, "./src/abi/resolver.json");
