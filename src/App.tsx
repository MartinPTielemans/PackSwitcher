import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { UpdateChecker } from "./UpdateChecker";

import "./App.css";

type PackageManager = 'npm' | 'pnpm' | 'yarn' | 'bun';

function App() {
  const [selectedPM, setSelectedPM] = useState<PackageManager>('npm');
  const [isMonitoring, setIsMonitoring] = useState(false);

  useEffect(() => {
    invoke("init");
    
    // Load saved preferences
    invoke("get_preferred_package_manager").then((pm: any) => {
      if (pm) setSelectedPM(pm);
    });

    invoke("get_monitoring_state").then((state: any) => {
      setIsMonitoring(state);
    });
  }, []);

  const handlePMChange = async (pm: PackageManager) => {
    setSelectedPM(pm);
    await invoke("set_preferred_package_manager", { packageManager: pm });
  };

  const toggleMonitoring = async () => {
    const newState = !isMonitoring;
    setIsMonitoring(newState);
    await invoke("toggle_monitoring", { enabled: newState });
  };

  const handleQuit = async () => {
    await invoke("quit_app");
  };

  return (
    <>
      <UpdateChecker />
      <div className="container">
        <div className="header">
          <h1>📦 Package Manager Switcher</h1>
          <div className="header-controls">
            <div className="status">
              <span className={`indicator ${isMonitoring ? 'active' : 'inactive'}`}>
                {isMonitoring ? '🟢' : '🔴'}
              </span>
              <span>{isMonitoring ? 'Monitoring' : 'Stopped'}</span>
            </div>
            <button className="quit-button" onClick={handleQuit} title="Quit Application">
              ✕
            </button>
          </div>
        </div>

        <div className="controls">
          <div className="pm-selector">
            <label>Preferred Package Manager:</label>
            <div className="pm-buttons">
              {(['npm', 'pnpm', 'yarn', 'bun'] as PackageManager[]).map(pm => (
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
  );
}

export default App;
