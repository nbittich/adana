import { EXAMPLES } from "./examples.js";

async function loadWasmContext() {
  const module = await import("./pkg/adana_script_wasm.js");
  await module.default();
  // return module.compute_as_string; use that one instead to create the heap from javascript
  return module.make_ctx_and_compute_as_string;
}

function toggleForm(form, toggle) {
  const elements = form.elements;
  for (let i = 0, len = elements.length; i < len; ++i) {
    elements[i].readOnly = toggle;
  }
  const submitButton = form.querySelector('button[type="submit"]');
  submitButton.disabled = toggle;
}
async function run() {
  const form = document.querySelector("form");

  // uncomment this to create the heap from javascript
  // const memory = new WebAssembly.Memory({
  //   initial: 32, // 2mb
  //   maximum: 512, // 32mb
  //   shared: true,
  // });
  //  const ctx = new Uint8Array(memory.buffer);

  toggleForm(form, true);
  let compute = await loadWasmContext();

  toggleForm(form, false);
  const text_area = document.querySelector("#code");
  text_area.value = "";
  text_area.focus();

  const out = document.querySelector("#out");
  out.value = "";

  let logs = [];

  console.log = function () {
    for (const a of arguments) {
      logs.push(a);
    }
  };

  const select = document.querySelector("#examples");
  for (const example of EXAMPLES) {
    const option = document.createElement("option");
    option.innerText = example.label;
    option.value = example.key;
    select.appendChild(option);
  }

  select.addEventListener("change", function (e) {
    const key = e.target.value;
    const example = EXAMPLES.find((e) => e.key === key);
    if (example) {
      text_area.value = example.script;
    }
  });

  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    toggleForm(form, true);
    logs = [];
    out.classList.remove("text-danger");

    const data = new FormData(e.target);
    // uncomment this if you create memory from javascript
    // for (let i = 0; i < ctx.length; i++) {
    //   ctx[i] = undefined;
    // }
    try {
      // let res = compute(data.get("code") || "", ctx); // use this instead if you create memory from javascript
      let res = compute(data.get("code") || "");
      // console.log(res);
      out.value = logs.join("");
      toggleForm(form, false);
    } catch (e) {
      out.classList.add("text-danger");
      out.value = e.toString();
      compute = await loadWasmContext();
      toggleForm(form, false);
    }
  });
  const issueLink = document.querySelector("#issueLink");
  issueLink.onclick = (e) => {
    e.preventDefault();
    const a = document.createElement("a");
    let params = new URLSearchParams();
    const data = new FormData(form);
    params.append("title", "Adana playground bug");

    a.href = `https://github.com/nbittich/adana/issues/new?${params.toString()}`;
    a.target = "_blank";
    a.click();
  };

  const copyToClipboardBtn = document.querySelector("#copyToClipboard");
  copyToClipboardBtn.onclick = function (e) {
    e.preventDefault();
    if (out.value) {
      navigator.clipboard.writeText(out.value);

      alert("Copied to clipboard");
    } else {
      alert("Nothing to copy");
    }
  };
}

run();
