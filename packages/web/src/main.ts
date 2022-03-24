import { setContext } from 'svelte';
import App from './App.svelte';
import init from 'vite-wasm-functions';
import { default as initU } from 'wasm-game-of-life';

const load = async () => {
  const startTime = performance.now()
  await init()
  await initU()
  const endTime = performance.now()
  console.log(`Call to wasm init took ${endTime - startTime} milliseconds`)

  const app = new App({
    target: document.getElementById('app')
  })
}

load()
