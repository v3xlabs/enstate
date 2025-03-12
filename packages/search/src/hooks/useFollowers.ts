import { useQuery } from '@tanstack/react-query';
import axios from 'axios';

export interface FollowerData {
  followers_count: string;
  following_count: string;
}

export const useFollowers = (searchTerm: string) => {
  return useQuery({
    queryKey: ['followers', searchTerm],
    queryFn: async () => {
      if (!searchTerm.trim()) {
        return undefined;
      }
      
      const apiUrl = `https://api.ethfollow.xyz/api/v1/users/${searchTerm}/stats`;
      const response = await axios.get<FollowerData>(apiUrl
      );
      
      return response.data;
    },
    enabled: Boolean(searchTerm.trim()),
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
};
