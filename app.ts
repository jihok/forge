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

(async () => {
  try {
    const storeCode = new MsgStoreCode(
      wallet.key.accAddress,
      fs.readFileSync("./artifacts/my_first_contract.wasm").toString("base64")
    );

    const storeCodeTx = await wallet.createAndSignTx({
      msgs: [storeCode],
    });
    const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

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
