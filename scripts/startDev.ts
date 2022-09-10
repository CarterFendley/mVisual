import { spawn } from 'child_process'

const spawnProc = (command: string, args: string[]) => new Promise((resolve) => {
  const child = spawn(command, args)
  child.stdout.pipe(process.stdout)
  child.stderr.pipe(process.stderr)
  child.on('close', resolve)
});

const start = async () => {
  // Do initial build
  const buildProcess = spawnProc('yarn', ['build'])
  await buildProcess

  // Run electron and watch for updates with webpack
  const watchProcess = spawnProc('yarn', ['watch'])
  const electronProcess = spawnProc('electronmon', [`${__dirname}/../dist/electron.js`])

  await Promise.all([watchProcess, electronProcess])
}

start()
