import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface UpdateInfo {
  version: string;
}

export function UpdateChecker() {
  const [updateAvailable, setUpdateAvailable] = useState<UpdateInfo | null>(null);
  const [isUpdating, setIsUpdating] = useState(false);
  const [showDialog, setShowDialog] = useState(false);

  useEffect(() => {
    // Listen for update events from the backend
    const unlisten = listen<string>('update-available', (event) => {
      setUpdateAvailable({ version: event.payload });
      setShowDialog(true);
    });

    // Check for updates on component mount
    checkForUpdates();

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const checkForUpdates = async () => {
    try {
      await invoke('check_for_updates');
    } catch (error) {
      console.error('Failed to check for updates:', error);
    }
  };

  const handleUpdate = async () => {
    try {
      setIsUpdating(true);
      setShowDialog(false);
      await invoke('install_update');
      // The app will restart automatically after installation
    } catch (error) {
      console.error('Failed to update:', error);
      setIsUpdating(false);
    }
  };

  const handleDismiss = () => {
    setShowDialog(false);
    setUpdateAvailable(null);
  };

  if (isUpdating) {
    return (
      <div style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'rgba(0, 0, 0, 0.8)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1000
      }}>
        <div style={{
          backgroundColor: 'white',
          padding: '20px',
          borderRadius: '8px',
          textAlign: 'center',
          maxWidth: '300px'
        }}>
          <h3>Updating...</h3>
          <p>Please wait while the application updates. The app will restart automatically.</p>
          <div style={{ marginTop: '10px' }}>
            <div style={{
              width: '100%',
              height: '4px',
              backgroundColor: '#e0e0e0',
              borderRadius: '2px',
              overflow: 'hidden'
            }}>
              <div style={{
                width: '100%',
                height: '100%',
                backgroundColor: '#007acc',
                animation: 'pulse 1.5s ease-in-out infinite'
              }} />
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (showDialog && updateAvailable) {
    return (
      <div style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'rgba(0, 0, 0, 0.8)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1000
      }}>
        <div style={{
          backgroundColor: 'white',
          padding: '20px',
          borderRadius: '8px',
          textAlign: 'center',
          maxWidth: '400px'
        }}>
          <h3>Update Available</h3>
          <p>
            A new version <strong>{updateAvailable.version}</strong> is available. 
            Would you like to update now?
          </p>
          <div style={{ marginTop: '20px', display: 'flex', gap: '10px', justifyContent: 'center' }}>
            <button
              onClick={handleUpdate}
              style={{
                backgroundColor: '#007acc',
                color: 'white',
                border: 'none',
                padding: '10px 20px',
                borderRadius: '4px',
                cursor: 'pointer'
              }}
            >
              Update Now
            </button>
            <button
              onClick={handleDismiss}
              style={{
                backgroundColor: '#ccc',
                color: 'black',
                border: 'none',
                padding: '10px 20px',
                borderRadius: '4px',
                cursor: 'pointer'
              }}
            >
              Later
            </button>
          </div>
        </div>
      </div>
    );
  }

  return null; // Component doesn't render anything visible normally
} 