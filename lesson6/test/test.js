import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';

const provider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({provider});
await api.isReady;

const keyring = new Keyring({ type: 'sr25519' });
const Alice = keyring.addFromUri('//Alice');
const Bob = keyring.addFromUri('//Bob');

const getFreeBalance = async (api, address) => {
    const Account = await api.query.system.account(address);
    return Account['data']['free'].toHuman();
}

console.log(await getFreeBalance(api, Alice.address));
console.log(await getFreeBalance(api, Bob.address));

// // Transfer from Alice to Bol
// await api.tx.balances.transfer(Bob.address, 10000).signAndSend(Alice, res => {
//     console.log(`Tx status: ${res.status}`)
// });

// await api.query.system.account(Alice.address, aliceAcct => {
//     const aliceFreeSub = aliceAcct.data.free;
//     console.log(`Subscribtion events of Alice's account: ${aliceFreeSub}`);
// });

// const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
// await sleep(600000);

// console.log(await getFreeBalance(api, Alice.address));
// console.log(await getFreeBalance(api, Bob.address));

api.tx.balances
    .transfer(Bob.address, 22222)
    .signAndSend(Alice, ({ events = [], status }) => {
      console.log('Transaction status:', status.type);

      if (status.isInBlock) {
        console.log('Included at block hash', status.asInBlock.toHex());
        console.log('Events:');

        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
        });
      } else if (status.isFinalized) {
        console.log('Finalized block hash', status.asFinalized.toHex());

        process.exit(0);
      }
    });
