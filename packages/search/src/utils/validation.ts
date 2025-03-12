/**
 * Validates if a string could be an ENS name
 * Basic check: must contain a dot and have characters on both sides
 * More detailed spec: https://docs.ens.domains/contract-api-reference/name-processing
 * Supports multiple layers of subdomains (e.g., sub.domain.eth)
 */
export const isValidENSNameFormat = (name: string): boolean => {
  // Updated regex to support multiple subdomains
  // Requires at least one character in each segment separated by dots
  return /^[^\s.]+(\.[^\s.]+)+$/.test(name);
};

/**
 * Validates if a string matches Ethereum address format
 * Must be 0x followed by 40 hex characters
 */
export const isValidEthereumAddress = (address: string): boolean => {
  return /^0x[a-fA-F0-9]{40}$/.test(address);
};

/**
 * Determines if a search term should trigger a direct profile lookup
 */
export const shouldAttemptDirectLookup = (searchTerm: string): boolean => {
  const x = isValidENSNameFormat(searchTerm) || isValidEthereumAddress(searchTerm);
  if (!x) {
    console.log('notValid', searchTerm);
  }

  return x;
}; 