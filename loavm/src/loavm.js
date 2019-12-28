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

const loadingLoa = import("../gen").then(exports => {
  exports.init();
  run = exports.run;
});

Promise.all([waitForDOM(), loadingLoa]).then(async () => {
  const scripts = document.querySelectorAll(
    "script[type='application/loabin']"
  );

  for (const script of scripts) {
    const response = await fetch(script.src);

    run(new Uint8Array(await response.arrayBuffer()));
  }
});
