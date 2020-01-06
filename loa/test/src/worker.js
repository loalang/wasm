
import("../../gen").then(async ({ Server }) => {
    const server = await Server.load();

    server.set("some-uri", "12 + 32");

    const result = server.evaluate("some-uri");

    console.log(result);
});
