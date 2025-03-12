import { useQuery } from '@tanstack/react-query';
import axios from 'axios';

export interface SearchResult {
  name: string;
  address: string;
  avatar?: string;
  header?: string;
  display: string;
  contenthash?: string;
  records?: {
    avatar?: string;
    description?: string;
    email?: string;
    name?: string;
    url?: string;
    location?: string;
    'com.discord'?: string;
    'com.github'?: string;
    'com.twitter'?: string;
    'org.telegram'?: string;
    header?: string;
    timezone?: string;
    pronouns?: string;
    [key: string]: string | undefined;
  };
  chains?: Record<string, string>;
  fresh?: number;
  resolver?: string;
  ccip_urls?: string[];
  errors?: Record<string, any>;
}

export const useSearch = (searchTerm: string) => {
  return useQuery({
    queryKey: ['search', searchTerm],
    queryFn: async () => {
      if (!searchTerm.trim()) {
        return [] as SearchResult[];
      }
      
      const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:3000';
      const response = await axios.get<SearchResult[]>(
        `${apiUrl}/v2/discover/search?s=${encodeURIComponent(searchTerm)}`
      );
      
      return response.data;
    },
    enabled: Boolean(searchTerm.trim()),
    staleTime: 1000 * 10, // 10 seconds
  });
};
