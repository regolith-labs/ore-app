import React, { useMemo, useCallback } from 'react';
import ReactDOM from 'react-dom/client';
import { ConnectionProvider, WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import {
  WalletModalProvider,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { Transaction } from '@solana/web3.js';
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// Default styles that can be overridden by your app
require('@solana/wallet-adapter-react-ui/styles.css');

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
          <WalletMultiButton />
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
    if (publicKey) {
      try {
        const event = new CustomEvent(
          "ore-pubkey",
          {
            detail: {
              pubkey: publicKey.toBuffer().toJSON().data
            }
          }
        );
        window.dispatchEvent(
          event
        );
      } catch (err) {
        console.log(err);
      }
    }
    return
  }, [publicKey]);
}

function SignTransaction() {
  const { publicKey, signTransaction } = useWallet();
  const callback = useCallback(async (msg) => {
    try {
      const b64 = msg.b64;
      console.log(b64);
      const buf = Buffer.from(b64, "base64");
      console.log(buf);
      const tx = Transaction.from(buf);
      console.log(tx);
      const signed = await signTransaction(tx);
      console.log(signed);
      const ser = signed.serialize();
      console.log(ser);
      const str = ser.toString("base64");
      console.log(str);
      return str;
    } catch (err) {
      console.log(err);
    }
  }, [publicKey]);
  window.OreTxSigner = callback;
  return
}
