// This file is required by the index.html file and will
// be executed in the renderer process for that window.
// No Node.js APIs are available in this process unless
// nodeIntegration is set to true in webPreferences.
// Use preload.js to selectively enable features
// needed in the renderer process.

import rust from "../pkg/m_visual"

rust.then(m => {
  const canvas = document.getElementById('rustCanvas') as HTMLCanvasElement;
  const gl = canvas.getContext('webgl', { antialias: true });
  if (!canvas || !gl) {
    alert('Failed to init WebGL');
    return;
  }

  const FPS_THROTTLE = 1000.0 / 30.0; // Milliseconds / frames
  const visual = new m.MVisual();
  const initialTime = Date.now();
  var lastDrawTime = -1; // In ms

  function render() {
    window.requestAnimationFrame(render);
    const currTime = Date.now();

    if (currTime > lastDrawTime + FPS_THROTTLE && canvas && gl){
      lastDrawTime = currTime;

      // Check for window resize to update the canvas size
      if (window.innerHeight != canvas.height || window.innerWidth != canvas.width) {
        canvas.height = window.innerHeight;
        // canvas.clientHeight = window.innerHeight;
        canvas.style.height = `${window.innerHeight}`;

        canvas.width = window.innerWidth;
        // canvas.clientWidth = window.innerWidth;
        canvas.style.width = `${window.innerHeight}`;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }

      let elapsedTime = currTime - initialTime;
      visual.update(elapsedTime, window.innerHeight, window.innerWidth);
      visual.render()
    }
  }

  // Initialize the loop
  render();
})