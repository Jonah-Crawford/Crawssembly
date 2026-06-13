import init, { run_craw } from "./pkg/crawssembly.js";

const editor = document.getElementById("editor");
const output = document.getElementById("output");
const run = document.getElementById("run");
const clear = document.getElementById("clear");
const status = document.getElementById("status");
const examples = document.getElementById("examples");
const reset = document.getElementById("reset");
const copyOutput = document.getElementById("copy-output");

const programs = {
  hello: `sav 72 ref
sav 101 ref
sav 108 ref
sav 108 ref
sav 111 ref
sav 32 ref
sav 87 ref
sav 111 ref
sav 114 ref
sav 108 ref
sav 100 ref
sav 33 ref
stp`,

  countdown: `sav 5 r02

1
io text int r02
io text newline r01
cal add -1 r02
sav r01 r02
jmg 1

stp`,

  memory: `sav 100 r01
io mem addr r01

sav 123 r01
io mem write r01

sav 100 r01
io mem addr r01
io mem read r01

io text int r01
stp`,

  hex: `sav 123 r01
io text hex r01
stp`,

  loop: `1
jmp 1`
};

const defaultProgram = programs.hello;

try {
  await init();
  output.textContent = "Crawssembly Online loaded";
} catch (err) {
  output.textContent = "Crawssembly Online failed to load:\n" + err;
}

examples.onchange = () => {
  const selected = examples.value;

  if (!selected) return;

  editor.value = programs[selected];
  output.textContent = "";
  status.textContent = "Example loaded";
};

run.onclick = () => {
  output.textContent = "";
  status.textContent = "Running...";

  try {
    const start = performance.now();
    const result = run_craw(editor.value);
    const end = performance.now();

    output.textContent = result;
    status.textContent = `Done in ${(end - start).toFixed(2)} ms`;
  } catch (err) {
    output.textContent = "Runtime JS error:\n" + err;
    status.textContent = "Error";
  }
};

clear.onclick = () => {
  output.textContent = "";
  status.textContent = "Cleared";
};

reset.onclick = () => {
  editor.value = defaultProgram;
  examples.value = "";
  output.textContent = "";
  status.textContent = "Ready";
};

copyOutput.onclick = async () => {
  await navigator.clipboard.writeText(output.textContent);
  status.textContent = "Output copied";
};

editor.addEventListener("keydown", event => {
  if ((event.ctrlKey || event.metaKey) && event.key === "Enter") {
    event.preventDefault();
    run.click();
  }
});
