const rust = import('./chip8');
const wasm = import('./chip8_bg')

const canvas = document.getElementById("chip8-canvas");

const loadRom = async () =>
  new Uint8Array(await fetch('roms/WIPEOFF').then(resp => resp.arrayBuffer()));

const run = async () => {
  const { CPUWrapper } = await rust;
  const { memory } = await wasm;

  const cpu = CPUWrapper.new();
  cpu.reset();
  cpu.load_rom(await loadRom());
  
  const renderLoop = () => {
    cpu.cycle();
    requestAnimationFrame(renderLoop);
  }

  requestAnimationFrame(renderLoop);
}

run();
