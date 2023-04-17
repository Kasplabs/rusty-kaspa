globalThis.WebSocket = require('websocket').w3cwebsocket;

let kaspa = require('./kaspa/kaspa_wasm');
let { RpcClient, Encoding,
    XPublicKey,
} = kaspa;
kaspa.init_console_panic_hook();

(async ()=>{
    
    let xpub = await XPublicKey.fromXPrv(
        "kprv5y2qurMHCsXYrNfU3GCihuwG3vMqFji7PZXajMEqyBkNh9UZUJgoHYBLTKu1eM4MvUtomcXPQ3Sw9HZ5ebbM4byoUciHo1zrPJBQfqpLorQ",
        false,
        0n
    );

    console.log("xpub", xpub)

    let addresses = await xpub.receiveAddresses(0, 10);
    console.log("receive addresses", addresses)
    addresses = await xpub.changeAddresses(0, 10);
    console.log("change addresses", addresses)

})();