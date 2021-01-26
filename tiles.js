import init, { run_tiles_app } from './pkg/tiles.js';
async function main() {
   await init('/pkg/tiles_bg.wasm');
   run_tiles_app();
}
main()
