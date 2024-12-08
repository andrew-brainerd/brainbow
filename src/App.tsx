import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

import './App.css';

function App() {
  async function monitor() {
    await invoke('monitor_hue');
  }

  useEffect(() => {
    monitor();
  }, []);

  return (
    <main className="container">
      <h1>Monitoring Hue activity</h1>
    </main>
  );
}

export default App;
