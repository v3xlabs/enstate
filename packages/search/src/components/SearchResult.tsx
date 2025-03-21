import { Link } from '@tanstack/react-router';
import { LuMapPin, LuMail, LuGlobe, LuTwitter, LuGithub, LuMessageSquare, LuSend } from "react-icons/lu";
import { ChainIcon } from './ChainIcon';
import { getChainIconUrl } from '../utils/chainIcons';
import { useFollowers } from '../hooks/useFollowers';
import { ProfileSearchResult } from '@/hooks/useSearch';
import { useProfile } from '@/hooks/useProfile';

interface Profile {
  name: string;
  display: string;
  address?: string;
  avatar?: string;
  header?: string;
  records?: {
    header?: string;
    avatar?: string;
    description?: string;
    location?: string;
    email?: string;
    url?: string;
    'com.twitter'?: string;
    'com.github'?: string;
    'com.discord'?: string;
    'org.telegram'?: string;
  };
  chains?: Record<string, string>;
}

interface SearchResultProps {
  profile: ProfileSearchResult;
}

export function SearchResult({ profile }: SearchResultProps) {
  const { data: followersData } = useFollowers(profile.name);
  const { data: profileData } = useProfile(profile.name);

  return (
    <div className="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-all duration-200">
      <Link
        to="/$profileId"
        params={{ profileId: profile.name }}
        className="block h-full"
      >
        <div className="relative h-full flex flex-col">
          {/* Header/Banner image */}
          {profileData?.header || profileData?.records?.header ? (
            <div className="w-full aspect-[3/1] overflow-hidden">
              <img
                src={profileData.header || profileData.records?.header}
                alt={`${profileData.display} banner`}
                className="w-full h-full object-cover"
              />
            </div>
          ) : (
            <div className="w-full aspect-[3/1] bg-gradient-to-r from-blue-500 to-purple-600"></div>
          )}

          {/* Profile information with avatar */}
          <div className="p-2 flex-1">
            <div className="flex items-start space-x-2 pr-2 pb-1 h-full">
              {/* Avatar */}
              <div className={`${profileData?.header || profileData?.records?.header ? '-mt-7' : ''} flex-shrink-0`}>
                {profileData?.avatar ? (
                  <img
                    src={profileData.avatar}
                    alt={profileData.display}
                    className="h-14 w-14 rounded-full border-2 border-white shadow-md object-cover"
                  />
                ) : (
                  <div className="h-14 w-14 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 text-xl font-bold">
                    {profileData?.display?.charAt(0).toUpperCase()}
                  </div>
                )}
              </div>

              {/* Profile details */}
              <div className="flex-1 min-w-0 h-full flex flex-col">
                <h3 className="text-base font-semibold text-blue-600 truncate">
                  {profileData?.display}
                </h3>
                {profileData?.address && (
                  <p className="text-xs text-gray-500 truncate">
                    {profileData.address}
                  </p>
                )}
                <p className="mt-1 text-xs text-gray-600 whitespace-pre-line line-clamp-2">
                  {profileData?.records?.description || ''}
                </p>

                {/* Chain addresses */}
                {profileData?.chains && Object.keys(profileData.chains).length > 0 && (
                  <div className="mt-1.5 flex flex-wrap gap-x-2 gap-y-1">
                    {Object.entries(profileData.chains).map(([chain, address]) => (
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

                {/* Profile metadata */}
                <div className="mt-1.5 flex flex-wrap gap-x-3 gap-y-1 text-xs text-gray-500">
                  {profileData?.records?.location && (
                    <div className="flex items-center">
                      <LuMapPin className="mr-1 h-4 w-4" />
                      <span>{profileData.records.location}</span>
                    </div>
                  )}
                  {profileData?.records?.email && (
                    <div className="flex items-center">
                      <LuMail className="mr-1 h-4 w-4" />
                      <span>{profileData.records.email}</span>
                    </div>
                  )}
                  {profileData?.records?.url && (
                    <div className="flex items-center">
                      <LuGlobe className="mr-1 h-4 w-4" />
                      <span>{profileData.records.url}</span>
                    </div>
                  )}
                </div>

                {/* Social links and followers */}
                <div className="mt-1.5 flex justify-between items-center flex-1">
                  <div className="flex space-x-2">
                    {profileData?.records?.['com.twitter'] && (
                      <div className="text-blue-400 hover:text-blue-600">
                        <LuTwitter className="h-4 w-4" title={`Twitter: ${profileData.records['com.twitter']}`} />
                      </div>
                    )}
                    {profileData?.records?.['com.github'] && (
                      <div className="text-gray-700 hover:text-gray-900">
                        <LuGithub className="h-4 w-4" title={`GitHub: ${profileData.records['com.github']}`} />
                      </div>
                    )}
                    {profileData?.records?.['com.discord'] && (
                      <div className="text-indigo-500 hover:text-indigo-700">
                        <LuMessageSquare className="h-4 w-4" title={`Discord: ${profileData.records['com.discord']}`} />
                      </div>
                    )}
                    {profileData?.records?.['org.telegram'] && (
                      <div className="text-blue-500 hover:text-blue-700">
                        <LuSend className="h-4 w-4" title={`Telegram: ${profileData.records['org.telegram']}`} />
                      </div>
                    )}
                  </div>

                  {/* Followers data */}
                  {followersData && (
                    <div className="flex gap-3 items-center self-end" onClick={(e) => e.stopPropagation()}>
                      <a href={`https://ethfollow.xyz/${profile.name}?tab=following`} target="_blank" rel="noopener noreferrer" className="text-xs text-gray-500 flex gap-1 items-center hover:underline cursor-pointer">
                        <span className="font-bold">{followersData.following_count}</span>
                        <span className="text-gray-500">Following</span>
                      </a>
                      <a href={`https://ethfollow.xyz/${profile.name}?tab=followers`} target="_blank" rel="noopener noreferrer" className="text-xs text-gray-500 flex gap-1 items-center hover:underline cursor-pointer">
                        <span className="font-bold">{followersData.followers_count}</span>
                        <span className="text-gray-500">Followers</span>
                      </a>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        </div>
      </Link>
    </div>
  );
} 