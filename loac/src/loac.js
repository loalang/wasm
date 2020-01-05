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

import("../gen").then(async loac => {
  await waitForDOM();

  const scripts = document.querySelectorAll(
    "script[type='application/loabin'], script[type='application/loa']"
  );

  await Promise.all(
    Array.from(scripts, async script => {
      if (script.src) {
        const response = await fetch(script.src);

        switch (script.type) {
          case "application/loabin":
            loac.run(new Uint8Array(await response.arrayBuffer()));
            break;
          case "application/loa":
            loac.register(script.src, await response.text());
            break;
        }
      } else {
        loac.register("<inline>", script.innerHTML);
      }
    })
  );

  loac.compile();
});
