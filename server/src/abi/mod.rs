// Technically the PublicResolver but it implements the Resolver spec
#[allow(non_snake_case)]
pub mod IResolver {
    use ethers::prelude::abigen;

    abigen!(Resolver, "./src/abi/resolver.json");
}

// Universal Resolver (0xc0497E381f536Be9ce14B0dD3817cBcAe57d2F62)
#[allow(non_snake_case)]
pub mod UResolver {
    use ethers::prelude::abigen;

    abigen!(UniversalResolver, "./src/abi/universal_resolver.json");
}
