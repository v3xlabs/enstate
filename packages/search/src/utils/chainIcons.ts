export const getChainIconUrl = (chain: string): string => {
  // Convert chain name to lowercase for consistency
  const chainName = chain.toLowerCase();
  
  // Map eth to ethereum for icon URL
  const normalizedChain = chainName === 'eth' ? 'ethereum' : chainName;
  
  return `https://frame.nyc3.cdn.digitaloceanspaces.com/icons/${normalizedChain}.svg`;
}; 