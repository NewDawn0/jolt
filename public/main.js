import init, { GlRender } from './pkg/jolt.js';

const load = async () => {
  await init().catch(console.error);
  console.info("Wasm loaded");
}
load();
