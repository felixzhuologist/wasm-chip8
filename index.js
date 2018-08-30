const rust = import('./chip8');
const wasm = import('./chip8_bg')

rust.then(m => {
wasm.then(w => {

const { CPU } = m;
const { memory } = w;

const cpu = CPU.new();
cpu.reset();
const canvas = document.getElementById("chip8-canvas");
console.log(cpu);
console.log("yay!");

});
});
