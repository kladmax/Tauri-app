import { invoke } from '@tauri-apps/api/tauri'

document.getElementById('hibernate-btn').addEventListener('click', async () => {
  try {
      const message = await window.__TAURI__.invoke('run_hibernate');
      alert(message);
  } catch (error) {
      alert(error);
  }
});

