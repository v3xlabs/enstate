/**
 * Validates if a string could be an ENS name
 * Basic check: must contain a dot and have characters on both sides
 * More detailed spec: https://docs.ens.domains/contract-api-reference/name-processing
 */
export const isValidENSNameFormat = (name: string): boolean => {
  // Basic check for dot and characters on both sides
  // This regex allows for UTF-8 characters, numbers, and hyphens
  // but requires at least one character on each side of a dot
  return /^[^\s.]+\.[^\s.]+$/.test(name);
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
  return isValidENSNameFormat(searchTerm) || isValidEthereumAddress(searchTerm);
}; 