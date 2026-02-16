const binding = require("../index.node");
const assert = require("assert");

async function test() {
  console.log("Loading zcp-node binding...");
  console.log("Exports:", binding);

  const { Client } = binding;
  console.log("Client class:", Client);
  console.log("Client keys:", Object.keys(Client));
  console.log(
    "Client prototype:",
    Object.getOwnPropertyNames(Client.prototype),
  );
  console.log("Client static props:", Object.getOwnPropertyNames(Client));

  assert(Client, "Client class should be exported");
  assert(Client.connectTcp, "Client.connectTcp should be exported");

  try {
    console.log("Attempting connection to 127.0.0.1:9999 (should fail)...");
    await Client.connectTcp("127.0.0.1:9999");
  } catch (e) {
    console.log("caught error as expected:", e.message);
    assert(
      e.message.includes("Failed to connect"),
      "Error should be connection related",
    );
    console.log("✅ PASS: Binding works!");
    return;
  }

  throw new Error("Should have failed connection");
}

test().catch((err) => {
  console.error("❌ FAIL:", err);
  process.exit(1);
});
