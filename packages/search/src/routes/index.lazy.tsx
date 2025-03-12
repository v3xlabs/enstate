import { createLazyFileRoute, useNavigate, useSearch as useSearchParams, Link } from '@tanstack/react-router';
import { useState, useEffect } from 'react';
import { useSearch } from '../hooks/useSearch';
import { useProfile } from '../hooks/useProfile';
import { LuSearch, LuMapPin, LuMail, LuGlobe, LuTwitter, LuGithub, LuMessageSquare, LuSend } from "react-icons/lu";
import { useDebounce } from 'use-debounce';
import { getChainIconUrl } from '../utils/chainIcons';
import { shouldAttemptDirectLookup } from '../utils/validation';
import { ChainIcon } from '../components/ChainIcon';

interface SearchParams {
  q?: string
}

export const Route = createLazyFileRoute('/')({
  component: Home,
});

// ProfileFallback component that attempts direct profile lookup
function ProfileFallback({ searchTerm }: { searchTerm: string }) {
  const { data: profile, isLoading, error } = useProfile(searchTerm);

  if (isLoading) {
    return (
      <div className="text-center p-6">
        <div className="inline-flex items-center">
          <div className="animate-spin rounded-full h-5 w-5 border-2 border-blue-500 border-t-transparent mr-2"></div>
          <span className="text-gray-500">Looking up profile directly...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-6 text-center">
        <p className="text-gray-500">No profile found for "{searchTerm}"</p>
      </div>
    );
  }

  if (!profile) {
    return (
      <div className="p-6 text-center">
        <p className="text-gray-500">No profile found for "{searchTerm}"</p>
      </div>
    );
  }

  // If we found a profile, render it
  return (
    <div className="p-6">
      <div className="text-center mb-4">
        <p className="text-gray-500 mb-2">No search results found, but we found this profile:</p>
      </div>
      <div className="max-w-2xl mx-auto bg-white rounded-lg overflow-hidden shadow-sm">
        <Link
          to="/$profileId"
          // @ts-ignore
          params={{ profileId: profile.name }}
          className="block"
        >
          <div className="relative">
            {/* Header/Banner image */}
            {profile.header || profile.records?.header ? (
              <div className="w-full aspect-[3/1] overflow-hidden">
                <img 
                  src={profile.header || profile.records?.header} 
                  alt={`${profile.display} banner`} 
                  className="w-full h-full object-cover"
                />
              </div>
            ) : (
              <div className="w-full aspect-[3/1] bg-gradient-to-r from-blue-500 to-purple-600"></div>
            )}
            
            {/* Profile information with avatar */}
            <div className="p-4">
              <div className="flex items-start space-x-4">
                {/* Avatar */}
                <div className={`${profile.header || profile.records?.header ? '-mt-10' : ''} flex-shrink-0`}>
                  {(profile.avatar || profile.records?.avatar) ? (
                    <img 
                      src={profile.avatar || profile.records?.avatar} 
                      alt={profile.display}
                      className="h-20 w-20 rounded-full border-2 border-white shadow-md object-cover"
                    />
                  ) : (
                    <div className="h-20 w-20 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 text-2xl font-bold">
                      {profile.display.charAt(0).toUpperCase()}
                    </div>
                  )}
                </div>
                
                {/* Profile details */}
                <div className="flex-1 min-w-0">
                  <h3 className="text-xl font-semibold text-blue-600 truncate">
                    {profile.display}
                  </h3>
                  <p className="text-sm text-gray-500 truncate">
                    {profile.address}
                  </p>
                  <p className="mt-2 text-sm text-gray-600 whitespace-pre-line line-clamp-3">
                    {profile.records?.description || ''}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </Link>
      </div>
    </div>
  );
}

function Home() {
  const navigate = useNavigate();
  const params = new URLSearchParams(typeof window !== 'undefined' ? window.location.search : '');
  const [searchInput, setSearchInput] = useState(params.get('q') || '');
  const [debouncedSearchTerm] = useDebounce(searchInput, 300);
  const { data, isLoading, error } = useSearch(debouncedSearchTerm);
  
  // Keep track of previous results to show during new searches
  const [isInitialLoading, setIsInitialLoading] = useState(false);
  
  // When loading state changes, determine if it's an initial load or a subsequent load
  useEffect(() => {
    if (isLoading && !data) {
      setIsInitialLoading(true);
    } else {
      setIsInitialLoading(false);
    }
  }, [isLoading, data]);

  // Update URL when search term changes
  useEffect(() => {
    if (typeof window === 'undefined') return;
    
    const currentParams = new URLSearchParams(window.location.search);
    if (debouncedSearchTerm) {
      currentParams.set('q', debouncedSearchTerm);
    } else {
      currentParams.delete('q');
    }
    
    const newSearch = currentParams.toString();
    const newPath = window.location.pathname + (newSearch ? `?${newSearch}` : '');
    
    navigate({
      to: newPath,
      replace: true,
    });
  }, [debouncedSearchTerm, navigate]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // The URL will be updated by the effect above
  };

  // Render search results or appropriate message
  const renderResults = () => {
    // Case 1: We have data to show
    if (data && data.length > 0) {
      // Check if any of the results have an exact name match with the search term
      const hasExactMatch = data.some(profile => profile.name?.toLowerCase() === debouncedSearchTerm.toLowerCase());
      
      // If no exact match and it's a valid ENS name/address, try direct lookup alongside results
      if (!hasExactMatch && shouldAttemptDirectLookup(debouncedSearchTerm)) {
        return (
          <>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-1 relative">
              {data.map((profile, index) => (
                <div key={profile.name + index} className="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-all duration-200">
                  <Link
                    to="/$profileId"
                    // params={{ profileId: profile.name || profile.address }}
                    // @ts-ignore
                    params={{ profileId: profile.name }}
                    className="block h-full"
                  >
                    <div className="relative">
                      {/* Header/Banner image */}
                      {profile.header || profile.records?.header ? (
                        <div className="w-full aspect-[3/1] overflow-hidden">
                          <img 
                            src={profile.header || profile.records?.header} 
                            alt={`${profile.display} banner`} 
                            className="w-full h-full object-cover"
                          />
                        </div>
                      ) : (
                        <div className="w-full aspect-[3/1] bg-gradient-to-r from-blue-500 to-purple-600"></div>
                      )}
                      
                      {/* Profile information with avatar */}
                      <div className="p-2">
                        <div className="flex items-start space-x-2 pb-3">
                          {/* Avatar */}
                          <div className={`${profile.header || profile.records?.header ? '-mt-7' : ''} flex-shrink-0`}>
                            {profile.avatar ? (
                              <img 
                                src={profile.avatar} 
                                alt={profile.display}
                                className="h-14 w-14 rounded-full border-2 border-white shadow-md object-cover"
                              />
                            ) : (
                              <div className="h-14 w-14 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 text-xl font-bold">
                                {profile.display.charAt(0).toUpperCase()}
                              </div>
                            )}
                          </div>
                          
                          {/* Profile details */}
                          <div className="flex-1 min-w-0">
                            <h3 className="text-base font-semibold text-blue-600 truncate">
                              {profile.display}
                            </h3>
                            <p className="text-xs text-gray-500 truncate">
                              {profile.address}
                            </p>
                            <p className="mt-1 text-xs text-gray-600 whitespace-pre-line line-clamp-2">
                              {profile.records?.description || ''}
                            </p>
                            
                            {/* Chain addresses */}
                            {profile.chains && Object.keys(profile.chains).length > 0 && (
                              <div className="mt-1.5 flex flex-wrap gap-x-2 gap-y-1">
                                {Object.entries(profile.chains).map(([chain, address]) => (
                                  <div key={chain} className="flex items-center text-xs text-gray-500" title={`${chain.toUpperCase()}: ${address}`}>
                                    <ChainIcon 
                                      chain={chain}
                                      iconUrl={getChainIconUrl(chain)}
                                      className="mr-1"
                                    />
                                    <span className="truncate max-w-[100px]">{address}</span>
                                  </div>
                                ))}
                              </div>
                            )}
                            
                            {/* Profile metadata - making it more compact */}
                            <div className="mt-1.5 flex flex-wrap gap-x-3 gap-y-1 text-xs text-gray-500">
                              {profile.records?.location && (
                                <div className="flex items-center">
                                  <LuMapPin className="mr-1 h-4 w-4" />
                                  <span>{profile.records.location}</span>
                                </div>
                              )}
                              {profile.records?.email && (
                                <div className="flex items-center">
                                  <LuMail className="mr-1 h-4 w-4" />
                                  <span>{profile.records.email}</span>
                                </div>
                              )}
                              {profile.records?.url && (
                                <div className="flex items-center">
                                  <LuGlobe className="mr-1 h-4 w-4" />
                                  <span>{profile.records.url}</span>
                                </div>
                              )}
                            </div>
                            
                            {/* Social links */}
                            <div className="mt-1.5 flex space-x-2">
                              {profile.records?.['com.twitter'] && (
                                <div className="text-blue-400 hover:text-blue-600">
                                  <LuTwitter className="h-4 w-4" title={`Twitter: ${profile.records['com.twitter']}`} />
                                </div>
                              )}
                              {profile.records?.['com.github'] && (
                                <div className="text-gray-700 hover:text-gray-900">
                                  <LuGithub className="h-4 w-4" title={`GitHub: ${profile.records['com.github']}`} />
                                </div>
                              )}
                              {profile.records?.['com.discord'] && (
                                <div className="text-indigo-500 hover:text-indigo-700">
                                  <LuMessageSquare className="h-4 w-4" title={`Discord: ${profile.records['com.discord']}`} />
                                </div>
                              )}
                              {profile.records?.['org.telegram'] && (
                                <div className="text-blue-500 hover:text-blue-700">
                                  <LuSend className="h-4 w-4" title={`Telegram: ${profile.records['org.telegram']}`} />
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </Link>
                </div>
              ))}
            </div>
            <div className="mt-8 border-t pt-6">
              <ProfileFallback searchTerm={debouncedSearchTerm} />
            </div>
          </>
        );
      }
      
      return (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-1 relative">
          {data.map((profile, index) => (
            <div key={profile.name + index} className="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-all duration-200">
              <Link
                to="/$profileId"
                // @ts-ignore
                params={{ profileId: profile.name }}
                className="block h-full"
              >
                <div className="relative">
                  {/* Header/Banner image */}
                  {profile.header || profile.records?.header ? (
                    <div className="w-full aspect-[3/1] overflow-hidden">
                      <img 
                        src={profile.header || profile.records?.header} 
                        alt={`${profile.display} banner`} 
                        className="w-full h-full object-cover"
                      />
                    </div>
                  ) : (
                    <div className="w-full aspect-[3/1] bg-gradient-to-r from-blue-500 to-purple-600"></div>
                  )}
                  
                  {/* Profile information with avatar */}
                  <div className="p-2">
                    <div className="flex items-start space-x-2 pb-3">
                      {/* Avatar */}
                      <div className={`${profile.header || profile.records?.header ? '-mt-7' : ''} flex-shrink-0`}>
                        {profile.avatar ? (
                          <img 
                            src={profile.avatar} 
                            alt={profile.display}
                            className="h-14 w-14 rounded-full border-2 border-white shadow-md object-cover"
                          />
                        ) : (
                          <div className="h-14 w-14 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 text-xl font-bold">
                            {profile.display.charAt(0).toUpperCase()}
                          </div>
                        )}
                      </div>
                      
                      {/* Profile details */}
                      <div className="flex-1 min-w-0">
                        <h3 className="text-base font-semibold text-blue-600 truncate">
                          {profile.display}
                        </h3>
                        <p className="text-xs text-gray-500 truncate">
                          {profile.address}
                        </p>
                        <p className="mt-1 text-xs text-gray-600 whitespace-pre-line line-clamp-2">
                          {profile.records?.description || ''}
                        </p>
                        
                        {/* Chain addresses */}
                        {profile.chains && Object.keys(profile.chains).length > 0 && (
                          <div className="mt-1.5 flex flex-wrap gap-x-2 gap-y-1">
                            {Object.entries(profile.chains).map(([chain, address]) => (
                              <div key={chain} className="flex items-center text-xs text-gray-500" title={`${chain.toUpperCase()}: ${address}`}>
                                <ChainIcon 
                                  chain={chain}
                                  iconUrl={getChainIconUrl(chain)}
                                  className="mr-1"
                                />
                                <span className="truncate max-w-[100px]">{address}</span>
                              </div>
                            ))}
                          </div>
                        )}
                        
                        {/* Profile metadata - making it more compact */}
                        <div className="mt-1.5 flex flex-wrap gap-x-3 gap-y-1 text-xs text-gray-500">
                          {profile.records?.location && (
                            <div className="flex items-center">
                              <LuMapPin className="mr-1 h-4 w-4" />
                              <span>{profile.records.location}</span>
                            </div>
                          )}
                          {profile.records?.email && (
                            <div className="flex items-center">
                              <LuMail className="mr-1 h-4 w-4" />
                              <span>{profile.records.email}</span>
                            </div>
                          )}
                          {profile.records?.url && (
                            <div className="flex items-center">
                              <LuGlobe className="mr-1 h-4 w-4" />
                              <span>{profile.records.url}</span>
                            </div>
                          )}
                        </div>
                        
                        {/* Social links */}
                        <div className="mt-1.5 flex space-x-2">
                          {profile.records?.['com.twitter'] && (
                            <div className="text-blue-400 hover:text-blue-600">
                              <LuTwitter className="h-4 w-4" title={`Twitter: ${profile.records['com.twitter']}`} />
                            </div>
                          )}
                          {profile.records?.['com.github'] && (
                            <div className="text-gray-700 hover:text-gray-900">
                              <LuGithub className="h-4 w-4" title={`GitHub: ${profile.records['com.github']}`} />
                            </div>
                          )}
                          {profile.records?.['com.discord'] && (
                            <div className="text-indigo-500 hover:text-indigo-700">
                              <LuMessageSquare className="h-4 w-4" title={`Discord: ${profile.records['com.discord']}`} />
                            </div>
                          )}
                          {profile.records?.['org.telegram'] && (
                            <div className="text-blue-500 hover:text-blue-700">
                              <LuSend className="h-4 w-4" title={`Telegram: ${profile.records['org.telegram']}`} />
                            </div>
                          )}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </Link>
            </div>
          ))}
        </div>
      );
    }
    
    // Case 2: No results found for an active search - try direct profile lookup only if it looks like an ENS name or address
    if (debouncedSearchTerm && !isInitialLoading && !isLoading && shouldAttemptDirectLookup(debouncedSearchTerm)) {
      return <ProfileFallback searchTerm={debouncedSearchTerm} />;
    }
    
    // Case 3: No results found and not a valid ENS name/address format
    if (debouncedSearchTerm && !isInitialLoading && !isLoading) {
      return (
        <div className="p-6 text-center">
          <p className="text-gray-500">No results found for "{debouncedSearchTerm}"</p>
        </div>
      );
    }
    
    // Case 4: Initial loading with no previous data
    if (isInitialLoading) {
      return (
        <div className="text-center p-6">
          <div className="inline-flex items-center">
            <div className="animate-spin rounded-full h-5 w-5 border-2 border-blue-500 border-t-transparent mr-2"></div>
            <span className="text-gray-500">Searching...</span>
          </div>
        </div>
      );
    }
    
    // Case 5: Default state - no search input yet
    return (
      <div className="p-6 text-center">
        <p className="text-gray-500">Enter a search term to find profiles</p>
        <p className="text-sm text-gray-400 mt-2">
          Try searching for an ENS name (e.g., "vitalik.eth") or an Ethereum address
        </p>
      </div>
    );
  };

  return (
    <div className="flex flex-col">
      <h2 className="text-2xl font-semibold mb-4">Welcome to the Search App</h2>
      <p className="text-gray-600 mb-4">
        Search for ENS names, addresses, or profiles using the search bar below.
      </p>
      
      <div className="mb-8">
        <form onSubmit={handleSearch} className="flex gap-2">
          <div className="relative flex-1">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <LuSearch className="h-5 w-5 text-gray-400" />
            </div>
            <input
              type="text"
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              placeholder="Search ENS names, addresses..."
              className="pl-10 w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
            {isLoading && (
              <div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                <div className="animate-spin h-4 w-4 border-2 border-blue-500 rounded-full border-t-transparent"></div>
              </div>
            )}
          </div>
          <button
            type="submit"
            className="px-4 py-2 border border-transparent rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            Search
          </button>
        </form>
      </div>

      {/* Results section */}
      <div className="rounded-lg overflow-hidden relative">
        {renderResults()}
        
        {/* Always show a subtle indicator when search is happening */}
        {isLoading && Boolean(data) && (
          <div className="fixed bottom-4 right-4 bg-white shadow-md rounded-full p-1 z-50">
            <div className="animate-spin rounded-full h-5 w-5 border-2 border-blue-500 border-t-transparent"></div>
          </div>
        )}
        
        {/* Show error message if needed */}
        {error && (
          <div className="p-6 text-center">
            <p className="text-red-500">Error: {error.message}</p>
          </div>
        )}
      </div>
    </div>
  );
} 