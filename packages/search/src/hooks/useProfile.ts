import { useQuery } from '@tanstack/react-query';
import { ProfileSearchResult } from './useSearch';

// We'll use the same interface as SearchResult for consistency
export type Profile = ProfileSearchResult;

import { profileFetcher } from 'use-enstate/helpers';

export const useProfile = (profileId: string) => {
  return useQuery({
    queryKey: ['profile', profileId],
    queryFn: async () => {
      const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:3000';

      // // Use the universal endpoint that can handle both ENS names and addresses
      // const response = await axios.get<Profile>(
      //   `${apiUrl}/u/${encodeURIComponent(profileId)}`
      // );

      const response = await profileFetcher(apiUrl, profileId);

      if (!response) {
        throw new Error('Profile not found');
      }

      return response;
    },
    enabled: Boolean(profileId),
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
};
