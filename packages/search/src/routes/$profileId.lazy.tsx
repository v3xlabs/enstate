import { createLazyFileRoute } from '@tanstack/react-router';
import { useProfile } from '../hooks/useProfile';
import {
  LuMapPin,
  LuMail,
  LuGlobe,
  LuTwitter,
  LuGithub,
  LuMessageSquare,
  LuSend,
  LuCopy,
  LuCalendar,
  LuUser,
} from "react-icons/lu";
import { useState } from 'react';
import { getChainIconUrl } from '../utils/chainIcons';
import { ChainIcon } from '../components/ChainIcon';
import { useFollowers } from '@/hooks/useFollowers';

export const Route = createLazyFileRoute('/$profileId')({
  component: Profile,
});

function Profile() {
  const { profileId } = Route.useParams();
  const { data: profile, isLoading, error } = useProfile(profileId);
  const { data: followersData } = useFollowers(profileId);
  const [copySuccess, setCopySuccess] = useState('');

  // Helper function to copy text to clipboard
  const copyToClipboard = async (text: string, label: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopySuccess(`Copied ${label}!`);
      setTimeout(() => setCopySuccess(''), 2000);
    } catch (err) {
      setCopySuccess('Failed to copy');
      setTimeout(() => setCopySuccess(''), 2000);
    }
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center min-h-[50vh]">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-50 border border-red-400 text-red-700 p-4 rounded">
        <p>Error loading profile: {error.message}</p>
      </div>
    );
  }

  if (!profile) {
    return (
      <div className="bg-yellow-50 border border-yellow-400 text-yellow-700 p-4 rounded">
        <p>Profile not found</p>
      </div>
    );
  }

  // Format date for profile freshness
  const formatDate = (timestamp?: number) => {
    if (!timestamp) return 'Unknown';
    return new Date(timestamp).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  };

  return (
    <div className="bg-white shadow overflow-hidden rounded-lg">
      {/* Banner image */}
      {profile.header ? (
        <div className="w-full aspect-[3/1] overflow-hidden">
          <img
            src={profile.header}
            alt={`${profile.display} banner`}
            className="w-full h-full object-cover"
          />
        </div>
      ) : (
        <div className="w-full h-32 bg-gradient-to-r from-blue-500 to-purple-600"></div>
      )}

      {/* Profile header with avatar */}
      <div className="px-4 py-5 sm:px-6 relative">
        <div className="flex items-end">
          <div className="-mt-16 flex-shrink-0">
            {profile.avatar ? (
              <img
                src={profile.avatar}
                alt={profile.display}
                className="h-24 w-24 rounded-full border-4 border-white shadow-md object-cover"
              />
            ) : (
              <div className="h-24 w-24 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 text-3xl font-bold border-4 border-white shadow-md">
                {profile.display.charAt(0).toUpperCase()}
              </div>
            )}
          </div>
          <div className="ml-4 flex-1">
            <h1 className="text-2xl font-bold text-gray-900">{profile.display}</h1>
            <p className="text-sm text-gray-500">{profile.display || profile.name}</p>
          </div>
          {followersData && (
            <div className="ml-4 flex-1 flex justify-end gap-3 items-start self-start">
              <a href={`https://ethfollow.xyz/${profileId}?tab=following`} target="_blank" rel="noopener noreferrer" className="text-sm text-gray-500 flex gap-1 items-center hover:underline cursor-pointer">
                <span className="font-bold">{followersData.following_count}</span>
                <span className="text-gray-500">Following</span>
              </a>
              <a href={`https://ethfollow.xyz/${profileId}?tab=followers`} target="_blank" rel="noopener noreferrer" className="text-sm text-gray-500 flex gap-1 items-center hover:underline cursor-pointer">
                <span className="font-bold">{followersData.followers_count}</span>
                <span className="text-gray-500">Followers</span>
              </a>
            </div>
          )}
        </div>

        {/* Copy success message */}
        {copySuccess && (
          <div className="absolute top-2 right-2 bg-black bg-opacity-70 text-white px-3 py-1 rounded-md text-sm">
            {copySuccess}
          </div>
        )}
      </div>

      {/* Profile description */}
      {profile.records?.description && (
        <div className="px-4 py-3 sm:px-6 border-t border-gray-200">
          <p className="whitespace-pre-line text-gray-700">
            {profile.records.description}
          </p>
        </div>
      )}

      {/* Profile metadata */}
      <div className="border-t border-gray-200">
        <dl>
          {/* Basic Info Section */}
          <div className="px-4 py-4 sm:px-6">
            <h3 className="text-lg font-medium text-gray-900 mb-3">Basic Info</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* ENS Name */}
              <div className="flex items-start">
                <div className="mt-1 flex-shrink-0 text-gray-400">
                  <LuUser className="h-5 w-5" />
                </div>
                <div className="ml-3">
                  <dt className="text-sm font-medium text-gray-500">ENS Name</dt>
                  <dd className="mt-1 text-sm text-gray-900 flex items-center">
                    {profile.name}
                    <button
                      onClick={() => copyToClipboard(profile.name, 'ENS name')}
                      className="ml-2 text-gray-400 hover:text-gray-600"
                      title="Copy ENS name"
                    >
                      <LuCopy className="h-4 w-4" />
                    </button>
                  </dd>
                </div>
              </div>

              {/* Ethereum Address */}
              <div className="flex items-start">
                <div className="mt-1 flex-shrink-0 text-gray-400">
                  <svg className="h-5 w-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M12 2L3 12H5V20H19V12H21L12 2Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
                  </svg>
                </div>
                <div className="ml-3">
                  <dt className="text-sm font-medium text-gray-500">Ethereum Address</dt>
                  <dd className="mt-1 text-sm text-gray-900 flex items-center">
                    <span className="truncate max-w-xs">{profile.address}</span>
                    {profile.address && (
                      <button
                        onClick={() => copyToClipboard(profile.address!, 'address')}
                        className="ml-2 text-gray-400 hover:text-gray-600"
                        title="Copy address"
                      >
                        <LuCopy className="h-4 w-4" />
                      </button>
                    )}
                  </dd>
                </div>
              </div>

              {/* Email (if available) */}
              {profile.records?.email && (
                <div className="flex items-start">
                  <div className="mt-1 flex-shrink-0 text-gray-400">
                    <LuMail className="h-5 w-5" />
                  </div>
                  <div className="ml-3">
                    <dt className="text-sm font-medium text-gray-500">Email</dt>
                    <dd className="mt-1 text-sm text-gray-900 flex items-center">
                      {profile.records.email}
                      <button
                        onClick={() => copyToClipboard(profile.records.email, 'email')}
                        className="ml-2 text-gray-400 hover:text-gray-600"
                        title="Copy email"
                      >
                        <LuCopy className="h-4 w-4" />
                      </button>
                    </dd>
                  </div>
                </div>
              )}

              {/* Website (if available) */}
              {profile.records?.url && (
                <div className="flex items-start">
                  <div className="mt-1 flex-shrink-0 text-gray-400">
                    <LuGlobe className="h-5 w-5" />
                  </div>
                  <div className="ml-3">
                    <dt className="text-sm font-medium text-gray-500">Website</dt>
                    <dd className="mt-1 text-sm text-gray-900">
                      <a
                        href={profile.records.url.startsWith('http') ? profile.records.url : `https://${profile.records.url}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-blue-600 hover:text-blue-800"
                      >
                        {profile.records.url}
                      </a>
                    </dd>
                  </div>
                </div>
              )}

              {/* Location (if available) */}
              {profile.records?.location && (
                <div className="flex items-start">
                  <div className="mt-1 flex-shrink-0 text-gray-400">
                    <LuMapPin className="h-5 w-5" />
                  </div>
                  <div className="ml-3">
                    <dt className="text-sm font-medium text-gray-500">Location</dt>
                    <dd className="mt-1 text-sm text-gray-900">{profile.records.location}</dd>
                  </div>
                </div>
              )}

              {/* Last Updated */}
              {profile.fresh && (
                <div className="flex items-start">
                  <div className="mt-1 flex-shrink-0 text-gray-400">
                    <LuCalendar className="h-5 w-5" />
                  </div>
                  <div className="ml-3">
                    <dt className="text-sm font-medium text-gray-500">Last Updated</dt>
                    <dd className="mt-1 text-sm text-gray-900">{formatDate(profile.fresh)}</dd>
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Social Media Section */}
          {(profile.records?.['com.twitter'] ||
            profile.records?.['com.github'] ||
            profile.records?.['com.discord'] ||
            profile.records?.['org.telegram']) && (
              <div className="px-4 py-4 sm:px-6 border-t border-gray-200">
                <h3 className="text-lg font-medium text-gray-900 mb-3">Social Media</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* Twitter */}
                  {profile.records?.['com.twitter'] && (
                    <div className="flex items-start">
                      <div className="mt-1 flex-shrink-0 text-blue-400">
                        <LuTwitter className="h-5 w-5" />
                      </div>
                      <div className="ml-3">
                        <dt className="text-sm font-medium text-gray-500">Twitter</dt>
                        <dd className="mt-1 text-sm text-gray-900">
                          <a
                            href={`https://twitter.com/${profile.records['com.twitter']}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:text-blue-800"
                          >
                            @{profile.records['com.twitter']}
                          </a>
                        </dd>
                      </div>
                    </div>
                  )}

                  {/* GitHub */}
                  {profile.records?.['com.github'] && (
                    <div className="flex items-start">
                      <div className="mt-1 flex-shrink-0 text-gray-700">
                        <LuGithub className="h-5 w-5" />
                      </div>
                      <div className="ml-3">
                        <dt className="text-sm font-medium text-gray-500">GitHub</dt>
                        <dd className="mt-1 text-sm text-gray-900">
                          <a
                            href={`https://github.com/${profile.records['com.github']}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:text-blue-800"
                          >
                            {profile.records['com.github']}
                          </a>
                        </dd>
                      </div>
                    </div>
                  )}

                  {/* Discord */}
                  {profile.records?.['com.discord'] && (
                    <div className="flex items-start">
                      <div className="mt-1 flex-shrink-0 text-indigo-500">
                        <LuMessageSquare className="h-5 w-5" />
                      </div>
                      <div className="ml-3">
                        <dt className="text-sm font-medium text-gray-500">Discord</dt>
                        <dd className="mt-1 text-sm text-gray-900 flex items-center">
                          {profile.records['com.discord']}
                          <button
                            onClick={() => copyToClipboard(profile.records['com.discord'], 'Discord username')}
                            className="ml-2 text-gray-400 hover:text-gray-600"
                            title="Copy Discord username"
                          >
                            <LuCopy className="h-4 w-4" />
                          </button>
                        </dd>
                      </div>
                    </div>
                  )}

                  {/* Telegram */}
                  {profile.records?.['org.telegram'] && (
                    <div className="flex items-start">
                      <div className="mt-1 flex-shrink-0 text-blue-500">
                        <LuSend className="h-5 w-5" />
                      </div>
                      <div className="ml-3">
                        <dt className="text-sm font-medium text-gray-500">Telegram</dt>
                        <dd className="mt-1 text-sm text-gray-900">
                          <a
                            href={`https://t.me/${profile.records['org.telegram']}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:text-blue-800"
                          >
                            @{profile.records['org.telegram']}
                          </a>
                        </dd>
                      </div>
                    </div>
                  )}
                </div>
              </div>
            )}

          {/* Blockchain Addresses Section */}
          {profile.chains && Object.keys(profile.chains).length > 0 && (
            <div className="px-4 py-4 sm:px-6 border-t border-gray-200">
              <h3 className="text-lg font-medium text-gray-900 mb-3">Blockchain Addresses</h3>
              <div className="grid grid-cols-1 gap-3">
                {Object.entries(profile.chains).map(([chain, address]) => (
                  <div key={chain} className="flex items-start">
                    <div className="mt-1 flex-shrink-0 text-gray-400">
                      <ChainIcon
                        chain={chain}
                        iconUrl={getChainIconUrl(chain)}
                        size="md"
                      />
                    </div>
                    <div className="ml-3">
                      <dt className="text-sm font-medium text-gray-500">{chain.toUpperCase()}</dt>
                      <dd className="mt-1 text-sm text-gray-900 flex items-center">
                        <span className="font-mono">{address}</span>
                        <button
                          onClick={() => copyToClipboard(address as string, `${chain} address`)}
                          className="ml-2 text-gray-400 hover:text-gray-600"
                          title={`Copy ${chain} address`}
                        >
                          <LuCopy className="h-4 w-4" />
                        </button>
                      </dd>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </dl>
      </div>
    </div>
  );
} 