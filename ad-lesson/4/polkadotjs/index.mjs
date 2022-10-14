import {ApiPromise, WsProvider } from '@polkadot/api';

const args = process.argv;

async function queryLocalStorage(key) {
    console.log(key);
    // network
    const provider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({provider});
    console.log(api.rpc.offchain);

    let bnU32Array = new Uint32Array(1);
    bnU32Array[0] = key;

    let inputKey = Buffer.from(bnU32Array.buffer).toString('hex');
    console.log('key', inputKey);
    let value = await api.rpc.offchain.localStorageGet('PERSISTENT', '0x'+inputKey);
    console.log('value', value.toHuman());
}

await queryLocalStorage(args[2]);