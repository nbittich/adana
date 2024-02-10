import init, { compute_as_string as compute } from "./pkg/adana_script_wasm.js";
import { EXAMPLES } from "./examples.js";

async function run() {
  const form = document.querySelector("form");
  form.classList.add("d-none");

  await init();

  const memory = new WebAssembly.Memory({
    initial: 32, // 2mb
    maximum: 512, // 32mb
    shared: true,
  });

  const ctx = new Uint8Array(memory.buffer);

  form.classList.remove("d-none");

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

  form.addEventListener("submit", (e) => {
    e.preventDefault();
    logs = [];
    const data = new FormData(e.target);
    for (let i = 0; i < ctx.length; i++) {
      ctx[i] = undefined;
    }
    let res = compute(data.get("code") || "", ctx);
    console.log(res); // NORDINE
    out.value = logs.join("");
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
