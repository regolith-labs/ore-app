import React, { useMemo, useCallback } from 'react';
import ReactDOM from 'react-dom/client';
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from '@solana/wallet-adapter-react';
import {
  WalletModalProvider,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { Keypair, SystemProgram, Transaction } from '@solana/web3.js';
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// Default styles that can be overridden by your app
require('@solana/wallet-adapter-react-ui/styles.css');

export const Wallet = () => {
  // You can also provide a custom RPC endpoint.
  const endpoint = "https://devnet.helius-rpc.com/?api-key=1de92644-323b-4900-9041-13c02730955c"

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
          <SendSOLToRandomAddress />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

function oreWalletAdapter() {
  const container = document.getElementById('ore-wallet-adapter-id');
  const root = ReactDOM.createRoot(container);
  root.render(<Wallet />);
}

function SendSOLToRandomAddress() {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();
  console.log(publicKey);
  const onClickk = useCallback(async () => {
    // 890880 lamports as of 2022-09-01
    const lamports = await connection.getMinimumBalanceForRentExemption(0);
    console.log(lamports);

    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: publicKey,
        toPubkey: Keypair.generate().publicKey,
        lamports,
      })
    );

    const {
      context: { slot: minContextSlot },
      value: { blockhash, lastValidBlockHeight }
    } = await connection.getLatestBlockhashAndContext();

    const signature = await sendTransaction(transaction, connection, { minContextSlot });

    await connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature });
  }, [publicKey, sendTransaction, connection]);
  window.addEventListener("ore-go", onClickk);
  return
}

window.OreWalletAdapter = oreWalletAdapter;
