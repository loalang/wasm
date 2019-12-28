function waitForDOM() {
  if (document.readyState !== "loading") {
    return Promise.resolve();
  }
  return new Promise(resolve => {
    window.addEventListener("DOMContentLoaded", () => {
      resolve();
    });
  });
}

let run;
let register;
let compile;

const loadingLoa = import("../gen").then(exports => {
  exports.init();
  run = exports.run;
  register = exports.register;
  compile = exports.compile;
});

Promise.all([waitForDOM(), loadingLoa]).then(async () => {
  const scripts = document.querySelectorAll(
    "script[type='application/loabin'], script[type='application/loa']"
  );

  for (const script of scripts) {
    if (script.src) {
      const response = await fetch(script.src);

      switch (script.type) {
        case "application/loabin":
          run(new Uint8Array(await response.arrayBuffer()));
          break;
        case "application/loa":
          register(script.src, await response.text());
          break;
      }
    } else {
      register("<inline>", script.innerHTML);
    }
  }

  compile();
});
