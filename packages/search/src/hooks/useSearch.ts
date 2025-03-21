import { useQuery } from '@tanstack/react-query';
import axios from 'axios';

export interface ProfileSearchResult {
  name: string;
}

export const useSearch = (searchTerm: string) => {
  return useQuery({
    queryKey: ['search', searchTerm],
    queryFn: async () => {
      if (!searchTerm.trim()) {
        return [] as ProfileSearchResult[];
      }

      const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:3000';
      const response = await axios.get<ProfileSearchResult[]>(
        `${apiUrl}/v2/discover/search?s=${encodeURIComponent(searchTerm)}`
      );
      
      return response.data;
    },
    enabled: Boolean(searchTerm.trim()),
    staleTime: 1000 * 10, // 10 seconds
  });
};
