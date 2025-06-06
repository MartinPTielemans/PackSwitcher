// Package manager types
export type PackageManager = 'npm' | 'pnpm' | 'yarn' | 'bun'

// Update-related types
export interface UpdateInfo {
  version: string
}

export interface DownloadProgress {
  downloaded: number
  contentLength: number
  percentage: number
}

// Tauri command response types
export interface TauriResponse<T> {
  success: boolean
  data?: T
  error?: string
}

// Event payload types for Tauri listeners
// Note: update-available event sends version as string payload, not object
export type UpdateAvailableEvent = string

export interface UpdateProgressEvent {
  downloaded: number
  contentLength: number
}

// Error types for better error handling
export class TauriError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
  ) {
    super(message)
    this.name = 'TauriError'
  }
}

// Utility type for async functions
export type AsyncFunction<T extends unknown[] = [], R = void> = (
  ...args: T
) => Promise<R> 