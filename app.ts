import {
  LCDClient,
  MsgSend,
  MnemonicKey,
  MsgStoreCode,
  isTxError,
} from "@terra-money/terra.js";
import * as fs from "fs";

// connect to testnet
const terra = new LCDClient({
  URL: "https://tequila-lcd.terra.dev",
  chainID: "tequila-0004",
});

// To use LocalTerra
// const terra = new LCDClient({
//   URL: 'http://localhost:1317',
//   chainID: 'localterra'
// });

// create a key out of a mnemonic
const mk = new MnemonicKey({
  mnemonic:
    "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius",
});

// a wallet can be created out of any key
// wallets abstract transaction building
const wallet = terra.wallet(mk);

// create a simple message that moves coin balances
// const send = new MsgSend(
//   'terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v',
//   'terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp',
//   { uluna: 1000000, ukrw: 1230201, uusd: 1312029 }
// );

// wallet
//   .createAndSignTx({
//     msgs: [send],
//     memo: 'test from terra.js!',
//   })
//   .then(tx => terra.tx.broadcast(tx))
//   .then(result => {
//     console.log(`TX hash: ${result.txhash}`);
//   });

(async () => {
  try {
    const storeCode = new MsgStoreCode(
      wallet.key.accAddress,
      fs.readFileSync("./artifacts/my_first_contract.wasm").toString("base64")
    );

    const storeCodeTx = await wallet.createAndSignTx({
      msgs: [storeCode],
    });
    console.log(terra);
    const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

    console.log(storeCodeTxResult);

    if (isTxError(storeCodeTxResult)) {
      throw new Error(
        `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
      );
    }

    const {
      store_code: { code_id },
    } = storeCodeTxResult.logs[0].eventsByType;
    console.log(code_id);
  } catch (e) {
    console.error(e);
  }
})();
