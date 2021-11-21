import init, { run_app }  from './pkg/package.js';
async function main() {
   await init('./pkg/package_bg.wasm');
   run_app();
}
main()