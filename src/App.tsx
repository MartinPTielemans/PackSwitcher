import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { UpdateChecker } from './UpdateChecker'
import type { PackageManager, AsyncFunction } from './types'

import './App.css'

function App(): React.JSX.Element {
  const [selectedPM, setSelectedPM] = useState<PackageManager>('npm')
  const [isMonitoring, setIsMonitoring] = useState<boolean>(false)

  useEffect((): void => {
    const initializeApp = async (): Promise<void> => {
      try {
        await invoke('init')

        // Load saved preferences
        const pm = await invoke<PackageManager>('get_preferred_package_manager')
        if (pm) {
          setSelectedPM(pm)
        }

        const monitoringState = await invoke<boolean>('get_monitoring_state')
        setIsMonitoring(monitoringState)
      } catch (error) {
        console.error('Failed to initialize app:', error)
      }
    }

    initializeApp()
  }, [])

  const handlePMChange: AsyncFunction<[PackageManager]> = async (
    pm: PackageManager,
  ): Promise<void> => {
    try {
      setSelectedPM(pm)
      await invoke('set_preferred_package_manager', { packageManager: pm })
    } catch (error) {
      console.error('Failed to set preferred package manager:', error)
      // Revert the UI state if the backend call failed
      setSelectedPM(selectedPM)
    }
  }

  const toggleMonitoring = async (): Promise<void> => {
    const newState = !isMonitoring
    
    try {
      setIsMonitoring(newState)
      await invoke('set_monitoring_state', { enabled: newState })
    } catch (error) {
      console.error('Failed to toggle monitoring:', error)
      // Revert to the original state on error
      setIsMonitoring(isMonitoring)
    }
  }

  const handleQuit: AsyncFunction = async (): Promise<void> => {
    try {
      await invoke('quit_app')
    } catch (error) {
      console.error('Failed to quit app:', error)
    }
  }

  return (
    <>
      <UpdateChecker />
      <div className="container">
        <div className="header">
          <h1>📦 Package Manager Switcher</h1>
          <div className="header-controls">
            <div className="status">
              <span
                className={`indicator ${isMonitoring ? 'active' : 'inactive'}`}
              >
                {isMonitoring ? '🟢' : '🔴'}
              </span>
              <span>{isMonitoring ? 'Monitoring' : 'Stopped'}</span>
            </div>
            <button
              className="quit-button"
              onClick={handleQuit}
              title="Quit Application"
            >
              ✕
            </button>
          </div>
        </div>

        <div className="controls">
          <div className="pm-selector">
            <label>Preferred Package Manager:</label>
            <div className="pm-buttons">
              {(['npm', 'pnpm', 'yarn', 'bun'] as const).map((pm: PackageManager) => (
                <button
                  key={pm}
                  className={`pm-button ${selectedPM === pm ? 'selected' : ''}`}
                  onClick={() => handlePMChange(pm)}
                >
                  {pm}
                </button>
              ))}
            </div>
          </div>

          <button
            className={`monitor-toggle ${isMonitoring ? 'stop' : 'start'}`}
            onClick={toggleMonitoring}
          >
            {isMonitoring ? 'Stop Monitoring' : 'Start Monitoring'}
          </button>
        </div>
      </div>
    </>
  )
}

export default App
