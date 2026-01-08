import init from './pkg/jolt.js';

const load = async () => {
  await init();
  console.info("Wasm loaded");
}
load();
