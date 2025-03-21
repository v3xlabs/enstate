import { createLazyFileRoute, useNavigate } from '@tanstack/react-router';
import { useState, useEffect } from 'react';
import { useSearch } from '../hooks/useSearch';
import { useProfile } from '../hooks/useProfile';
import { LuSearch } from "react-icons/lu";
import { useDebounce } from 'use-debounce';
import { shouldAttemptDirectLookup, isValidENSNameFormat } from '../utils/validation';
import { SearchResult } from '../components/SearchResult';

export const Route = createLazyFileRoute('/')({
  component: Home,
});

// SuggestedSearch component that shows .eth auto-complete when user forgets to add .eth
function SuggestedSearch({ searchTerm }: { searchTerm: string }) {
  const suggestedEns = `${searchTerm}.eth`;
  const { data: profile, isLoading } = useProfile(suggestedEns);

  // Only show if we found a profile
  if (isLoading || !profile) {
    return null;
  }

  return (
    <div className="mb-4 p-3 bg-blue-50 rounded-md shadow-sm border border-blue-200">
      <div className="flex items-center">
        {profile.avatar && (
          <img 
            src={profile.avatar} 
            alt={suggestedEns} 
            className="w-8 h-8 rounded-full mr-3"
          />
        )}
        <div>
          <p className="text-gray-700">
            Did you mean <a href={`/${suggestedEns}`} className="font-semibold text-blue-600 hover:text-blue-800">{suggestedEns}</a>?
          </p>
          <a 
            href={`/${suggestedEns}`}
            className="text-sm text-blue-600 hover:text-blue-800 underline"
          >
            Go to {suggestedEns} profile
          </a>
        </div>
      </div>
    </div>
  );
}

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

  if (error || !profile) {
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
      <div className="max-w-2xl mx-auto">
        <SearchResult profile={profile} />
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

  // Check if we should show the .eth auto-complete
  const shouldShowEthAutoComplete = () => {
    // Only show auto-complete if there's a search term, no dots in it (not already ENS), and it's not an Ethereum address
    return (
      debouncedSearchTerm && 
      debouncedSearchTerm.length > 1 && 
      !debouncedSearchTerm.includes('.') && 
      !debouncedSearchTerm.startsWith('0x')
    );
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
            <div className="mt-8 border-b mb-6 pb-3 w-full">
              <ProfileFallback searchTerm={debouncedSearchTerm} />
            </div>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-1 relative">
              {data.map((profile, index) => (
                <SearchResult key={profile.name + index} profile={profile} />
              ))}
            </div>
          </>
        );
      }

      return (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-1 relative">
          {data.map((profile, index) => (
            <SearchResult key={profile.name + index} profile={profile} />
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

      {/* .eth Auto-Complete */}
      {shouldShowEthAutoComplete() && (
        <SuggestedSearch searchTerm={debouncedSearchTerm} />
      )}

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