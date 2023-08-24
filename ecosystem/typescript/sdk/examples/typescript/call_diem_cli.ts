const ffi = require("ffi-napi");

const lib = ffi.Library("../../../../../target/release/libdiem", {
  run_diem_sync: ["char *", ["string"]], // run the diem CLI synchronously
  run_diem_async: ["char *", ["string"]], // run the diem CLI asynchronously
  free_cstring: ["void", ["char *"]], // free the return pointer memory allocated by the diem CLI
});

const args_run_local_testnet = ["diem", "node", "run-local-testnet", "--with-faucet"];
const args_diem_info = ["diem", "info"];

(async () => {
  console.log("Running diem CLI from Typescript");
  const diem_info = lib.run_diem_sync(args_diem_info.join(" "));
  const run_local_testnet = lib.run_diem_async(args_run_local_testnet.join(" "));
  try {
    console.log(`Diem Info: ${diem_info.readCString()}`);
    console.log(`Run Local Testnet: ${run_local_testnet.readCString()}`);
  } catch (error) {
    console.error(error);
  } finally {
    // free the string pointer memory allocated by the diem CLI
    lib.free_cstring(diem_info);
    lib.free_cstring(run_local_testnet);
  }

  console.log("Finish diem CLI");
})();
