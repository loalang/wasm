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

import("../gen").then(async loavm => {
  await waitForDOM();

  const scripts = document.querySelectorAll(
    "script[type='application/loabin']"
  );

  await Promise.all(
    Array.from(scripts, async script => {
      const response = await fetch(script.src);

      loavm.run(new Uint8Array(await response.arrayBuffer()));
    })
  );
});
