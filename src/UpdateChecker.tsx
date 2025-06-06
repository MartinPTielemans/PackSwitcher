import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type {
  UpdateInfo,
  DownloadProgress,
  UpdateAvailableEvent,
  UpdateProgressEvent,
  AsyncFunction,
} from './types'

export function UpdateChecker(): React.JSX.Element | null {
  const [updateAvailable, setUpdateAvailable] = useState<UpdateInfo | null>(
    null
  )
  const [isUpdating, setIsUpdating] = useState<boolean>(false)
  const [showDialog, setShowDialog] = useState<boolean>(false)
  const [downloadProgress, setDownloadProgress] =
    useState<DownloadProgress | null>(null)

  useEffect((): (() => void) => {
    let cleanupFunctions: UnlistenFn[] = []

    // Listen for update events from the backend
    const setupListeners = async (): Promise<void> => {
      try {
        const unlistenUpdate = await listen<UpdateAvailableEvent>(
          'update-available',
          (event): void => {
            // event.payload is a string (version), not an object
            setUpdateAvailable({ version: event.payload })
            setShowDialog(true)
          }
        )

        const unlistenProgress = await listen<UpdateProgressEvent>(
          'update-progress',
          (event): void => {
            const { downloaded, contentLength } = event.payload
            const percentage =
              contentLength > 0
                ? Math.round((downloaded / contentLength) * 100)
                : 0
            setDownloadProgress({ downloaded, contentLength, percentage })
          }
        )

        const unlistenFinished = await listen('update-finished', (): void => {
          console.log('Update installation finished')
        })

        cleanupFunctions = [unlistenUpdate, unlistenProgress, unlistenFinished]
      } catch (error) {
        console.error('Failed to setup update listeners:', error)
      }
    }

    setupListeners()

    // Check for updates on component mount
    checkForUpdates()

    // Cleanup listeners on component unmount
    return (): void => {
      // Handle cleanup functions properly - UnlistenFn returns void, not Promise
      cleanupFunctions.forEach((cleanup): void => {
        try {
          cleanup()
        } catch (error) {
          console.error('Error during cleanup:', error)
        }
      })
    }
  }, [])

  const checkForUpdates: AsyncFunction = async (): Promise<void> => {
    try {
      await invoke('check_for_updates')
    } catch (error) {
      console.error('Failed to check for updates:', error)
    }
  }

  const handleUpdate: AsyncFunction = async (): Promise<void> => {
    try {
      setIsUpdating(true)
      setShowDialog(false)
      setDownloadProgress({ downloaded: 0, contentLength: 0, percentage: 0 })
      await invoke('install_update')
    } catch (error) {
      console.error('Failed to update:', error)
      setIsUpdating(false)
      setDownloadProgress(null)
    }
  }

  const handleDismiss = (): void => {
    setShowDialog(false)
    setUpdateAvailable(null)
  }

  if (isUpdating) {
    return (
      <div
        style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          backgroundColor: 'rgba(0, 0, 0, 0.8)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000,
        }}
      >
        <div
          style={{
            backgroundColor: '#1e1e1e',
            padding: '15px',
            borderRadius: '8px',
            textAlign: 'center',
            width: '280px',
            maxHeight: '200px',
            border: '1px solid #2d2d2d',
            color: '#ffffff',
          }}
        >
          <h3
            style={{ margin: '0 0 8px 0', color: '#ffffff', fontSize: '16px' }}
          >
            Updating...
          </h3>
          <p
            style={{
              color: '#e0e0e0',
              margin: '0 0 12px 0',
              fontSize: '12px',
              lineHeight: '1.4',
            }}
          >
            {downloadProgress?.percentage === 100
              ? 'Installing update...'
              : 'Downloading update...'}
          </p>
          <div style={{ marginTop: '8px' }}>
            <div
              style={{
                width: '100%',
                height: '4px',
                backgroundColor: '#2d2d2d',
                borderRadius: '2px',
                overflow: 'hidden',
              }}
            >
              <div
                style={{
                  width: downloadProgress
                    ? `${downloadProgress.percentage}%`
                    : '100%',
                  height: '100%',
                  backgroundColor: '#007aff',
                  transition: 'width 0.3s ease',
                  animation: downloadProgress
                    ? 'none'
                    : 'pulse 1.5s ease-in-out infinite',
                }}
              />
            </div>
            {downloadProgress && downloadProgress.contentLength > 0 && (
              <div
                style={{
                  marginTop: '4px',
                  fontSize: '10px',
                  color: '#a0a0a0',
                  display: 'flex',
                  justifyContent: 'space-between',
                }}
              >
                <span>{downloadProgress.percentage}%</span>
                <span>
                  {(downloadProgress.downloaded / 1024 / 1024).toFixed(1)}MB /{' '}
                  {(downloadProgress.contentLength / 1024 / 1024).toFixed(1)}MB
                </span>
              </div>
            )}
          </div>
        </div>
      </div>
    )
  }

  if (showDialog && updateAvailable) {
    return (
      <div
        style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          backgroundColor: 'rgba(0, 0, 0, 0.8)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000,
        }}
      >
        <div
          style={{
            backgroundColor: '#1e1e1e',
            padding: '15px',
            borderRadius: '8px',
            textAlign: 'center',
            width: '280px',
            maxHeight: '200px',
            border: '1px solid #2d2d2d',
            color: '#ffffff',
          }}
        >
          <h3
            style={{ margin: '0 0 8px 0', color: '#ffffff', fontSize: '16px' }}
          >
            Update Available
          </h3>
          <p
            style={{
              color: '#e0e0e0',
              margin: '0 0 15px 0',
              fontSize: '12px',
              lineHeight: '1.4',
            }}
          >
            A new version{' '}
            <strong style={{ color: '#ffffff' }}>
              {updateAvailable.version}
            </strong>{' '}
            is available. Would you like to update now?
          </p>
          <div
            style={{ display: 'flex', gap: '8px', justifyContent: 'center' }}
          >
            <button
              onClick={handleUpdate}
              style={{
                backgroundColor: '#007aff',
                color: 'white',
                border: 'none',
                padding: '8px 16px',
                borderRadius: '4px',
                cursor: 'pointer',
                fontWeight: '500',
                fontSize: '12px',
                flex: 1,
              }}
            >
              Update Now
            </button>
            <button
              onClick={handleDismiss}
              style={{
                backgroundColor: '#2d2d2d',
                color: '#e0e0e0',
                border: '1px solid #3a3a3a',
                padding: '8px 16px',
                borderRadius: '4px',
                cursor: 'pointer',
                fontWeight: '500',
                fontSize: '12px',
                flex: 1,
              }}
            >
              Later
            </button>
          </div>
        </div>
      </div>
    )
  }

  return null // Component doesn't render anything visible normally
}
