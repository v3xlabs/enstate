/// <reference types="vite/client" />

interface ImportMeta {
  readonly env: {
    readonly VITE_API_URL: string;
    // Add other environment variables here
    [key: string]: string | undefined;
  };
} 