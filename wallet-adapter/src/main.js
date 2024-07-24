import React, { useMemo, useCallback } from 'react';
import ReactDOM from 'react-dom/client';
import { ConnectionProvider, WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import {
  BaseWalletMultiButton,
  WalletModalProvider,
} from '@solana/wallet-adapter-react-ui';
import { Transaction } from '@solana/web3.js';
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// Default styles that can be overridden by your app
require('../src/styles.css');

const LABELS = {
    'change-wallet': 'Change wallet',
    connecting: 'Connecting ...',
    'copy-address': 'Copy address',
    copied: 'Copied',
    disconnect: 'Disconnect',
    'has-wallet': 'Connect',
    'no-wallet': 'Connect',
};

export const Wallet = () => {
  const endpoint = "http://localhost:8899";
  const wallets = useMemo(
    () => [
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <BaseWalletMultiButton labels={LABELS} />
          { /* Your app's components go here, nested within the context providers. */}
          <Dispatcher />
          <SignTransaction />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

function MountWalletAdapter() {
  const container = document.getElementById("ore-wallet-adapter");
  const root = ReactDOM.createRoot(container);
  root.render(<Wallet />);
}
window.MountWalletAdapter = MountWalletAdapter;

function Dispatcher() {
  const { publicKey } = useWallet();
  useMemo(() => {
    let msg;
    if (publicKey) {
      msg = publicKey.toBuffer().toJSON().data;
    } else {
      msg = null
    }
    try {
      const event = new CustomEvent(
        "ore-pubkey",
        {
          detail: {
            pubkey: msg
          }
        }
      );
      window.dispatchEvent(
        event
      );
    } catch (err) {
      console.log(err);
    }
    return
  }, [publicKey]);
}

function SignTransaction() {
  const { publicKey, signTransaction } = useWallet();
  const callback = useCallback(async (msg) => {
    try {
      const tx = Transaction.from(
        Buffer.from(
          msg.b64,
          "base64"
        )
      );
      const signed = await signTransaction(
        tx
      );
      return signed
        .serialize()
        .toString("base64");
    } catch (err) {
      console.log(err);
    }
  }, [publicKey]);
  window.OreTxSigner = callback;
  return
}
