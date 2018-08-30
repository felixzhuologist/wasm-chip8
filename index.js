const rust = import('./chip8');
const wasm = import('./chip8_bg')

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const PIXEL_OFF_COLOR = "#FFFFFF";
const PIXEL_ON_COLOR = "#000000";

// maps key code to hex keypad index
const KEYMAP = {
  49: 0x1,
  50: 0x2,
  51: 0x3,
  52: 0xc,
  81: 0x4,
  87: 0x5,
  69: 0x6,
  82: 0xd,
  65: 0x7,
  83: 0x8,
  68: 0x9,
  70: 0xe,
  90: 0xa,
  88: 0x0,
  67: 0xb,
  86: 0xf
};

const run = async () => {

const { CPUWrapper } = await rust;
const { memory } = await wasm;

const cpu = CPUWrapper.new();
cpu.reset();

const canvas = document.getElementById('chip8-canvas');
const playPauseButton = document.getElementById('play-pause');
const stepButton = document.getElementById('step');
const width = 64;
const height = 32;
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');

const loadRom = async () =>
  new Uint8Array(await fetch('roms/WIPEOFF').then(resp => resp.arrayBuffer()));

const addKeyListeners = () => {
  document.addEventListener('keydown', event => {
    if (KEYMAP.hasOwnProperty(event.keyCode)) {
      cpu.key_down(KEYMAP[event.keyCode])
    }
  })
  document.addEventListener('keyup', event => {
    if (KEYMAP.hasOwnProperty(event.keyCode)) {
      cpu.key_up(KEYMAP[event.keyCode])
    }
  })
}

const drawScreen = () => {
    ctx.beginPath();
    
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        ctx.fillStyle = cpu.get_pixel(col, row) ? PIXEL_ON_COLOR : PIXEL_OFF_COLOR;
        ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
        );
      }
    }

    ctx.stroke();
}

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
}

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
}

const step = () => {
  cpu.cycle(true);
  drawScreen();
}

let animationId = null;
const isPaused = () => animationId === null;
const renderLoop = () => {
  for (let i = 0; i < 10; i++) {
    cpu.cycle();
  }
  cpu.decrement_timers();
  drawScreen();
  animationId = requestAnimationFrame(renderLoop);
}

const addPlayPauseListener = () => {
  playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
  })
  stepButton.addEventListener("click", event => step());
}

cpu.load_rom(await loadRom());
drawScreen();
addKeyListeners();
addPlayPauseListener();
pause();

}

run();
