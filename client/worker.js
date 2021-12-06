import init, { initThreadPool } from 'wasm-rand';

await init();
await initThreadPool(navigator.hardwareConcurrency);

import('wasm-rand')
  // Now we have the WASM methods available
  .then((wasm) => {
    const value = wasm.thread_rng_random();
    postMessage({value});
  })
  .catch(e => console.error('Error importing wasm module `wasm-rand`:', e));
