import React, { useMemo, useCallback } from "react";
import ReactDOM from "react-dom/client";
import {
  ConnectionProvider,
  WalletProvider,
  useWallet,
} from "@solana/wallet-adapter-react";
import {
  BaseWalletMultiButton,
  WalletModalProvider,
} from "@solana/wallet-adapter-react-ui";
import { Transaction, Connection, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, AccountLayout } from "@solana/spl-token";
import ExcelJs from "exceljs";
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// Default styles that can be overridden by your app
require("../src/styles.css");

const LABELS = {
  "change-wallet": "Change wallet",
  connecting: "Connecting ...",
  "copy-address": "Copy address",
  copied: "Copied",
  disconnect: "Disconnect",
  "has-wallet": "Connect",
  "no-wallet": "Connect",
};

export const Wallet = () => {
  const endpoint = "http://localhost:8899";
  const wallets = useMemo(
    () => [],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <BaseWalletMultiButton labels={LABELS} />
          {/* Your app's components go here, nested within the context providers. */}
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

const connection = new Connection(
  "https://solemn-distinguished-grass.solana-mainnet.quiknode.pro/1f1a1a918a36aea5a0f6ee9366fe1c4e1e7b1e63/"
);

const tokenMintAddress = new PublicKey(
  "oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp"
);

const exportToExcel = (data) => {
  let sheetName = "Allen_test.xlsx";
  let headerName = "RequestsList";

  // 获取sheet对象，设置当前sheet的样式
  // showGridLines: false 表示不显示表格边框
  let workbook = new ExcelJs.Workbook();
  let sheet = workbook.addWorksheet(sheetName, {
    views: [{ showGridLines: false }],
  });

  // 获取每一列的header
  let columnArr = [];
  for (let i in data[0]) {
    let tempObj = { name: "" };
    tempObj.name = i;
    columnArr.push(tempObj);
  }

  // 设置表格的主要数据部分
  sheet.addTable({
    name: headerName,
    ref: "A1",
    headerRow: true,
    totalsRow: false,
    style: {
      theme: "TableStyleMedium2",
      showRowStripes: false,
      width: 200,
    },
    columns: columnArr ? columnArr : [{ name: "" }],
    rows: data.map((e) => {
      let arr = [];
      for (let i in e) {
        arr.push(e[i]);
      }
      return arr;
    }),
  });

  sheet.getCell("A1").font = { size: 20, bold: true };

  // 设置每一列的宽度
  sheet.columns = sheet.columns.map((e) => {
    return { width: 60 };
  });

  const table = sheet.getTable(headerName);
  for (let i = 0; i < table.table.columns.length; i++) {
    // 表格主体数据是从A5开始绘制的，一共有三列。这里是获取A5到，B5，C5单元格，定义表格的头部样式
    sheet.getCell(`${String.fromCharCode(65 + i)}1`).font = { size: 12 };
    sheet.getCell(`${String.fromCharCode(65 + i)}1`).fill = {
      type: "pattern",
      pattern: "solid",
      fgColor: { argb: "c5d9f1" },
    };

    // 获取表格数据部分，定义其样式
    for (let j = 0; j < table.table.rows.length; j++) {
      let rowCell = sheet.getCell(`${String.fromCharCode(65 + i)}${j + 6}`);
      rowCell.alignment = { wrapText: true };
      rowCell.border = {
        bottom: {
          style: "thin",
          color: { argb: "a6a6a6" },
        },
      };
    }
  }
  table.commit();

  const writeFile = (fileName, content) => {
    const link = document.createElement("a");
    const blob = new Blob([content], {
      type: "application/vnd.ms-excel;charset=utf-8;",
    });
    link.download = fileName;
    link.href = URL.createObjectURL(blob);
    link.click();
  };

  // 表格的数据绘制完成，定义下载方法，将数据导出到Excel文件
  workbook.xlsx.writeBuffer().then((buffer) => {
    writeFile(sheetName, buffer);
  });
};

// 获取与Token Mint地址关联的所有Token账户
async function getTokenHolders() {
  try {
    // 获取Token账户与Mint地址关联的过滤器
    const filters = [
      {
        dataSize: 165, // SPL Token账户的数据大小为165字节
      },
      {
        memcmp: {
          offset: 0, // 代币Mint地址在Token账户数据中的偏移
          bytes: tokenMintAddress.toBase58(),
        },
      },
    ];

    // 获取与指定Mint地址相关的所有Token账户
    const accounts = await connection.getProgramAccounts(TOKEN_PROGRAM_ID, {
      filters: filters,
    });

    let HolderCount = 0;
    let HolderUsers = [];
    const Decimals = 10n ** 11n;
    accounts.forEach((account, index) => {
      const tokenAccountInfo = AccountLayout.decode(account.account.data);
      const holderAddress = new PublicKey(tokenAccountInfo.owner);

      const tokenAmount = BigInt(tokenAccountInfo.amount);
      const adjustedAmount = tokenAmount / Decimals;

      if (adjustedAmount > 10) {
        // console.log(`Holder ${index + 1}:`);
        // console.log(`  Address: ${holderAddress.toBase58()}`);
        // console.log(`  Amount: ${adjustedAmount}`); // Assuming 9 decimals for SPL token standard
        // console.log(`持有总数：${HolderCount}`);
        HolderCount++;
        HolderUsers.push({
          Address: holderAddress.toBase58(),
          Amount: `${adjustedAmount}`,
        });
      }
    });
    const container = document.getElementById("ore-holders-list");
    const root = ReactDOM.createRoot(container);
    root.render(
      <div class="flex flex-col gap-16 overflow-visible">
        <div class="flex flex-col grow gap-2 sm:gap-4 max-w-full">
          <h2 class="text-lg sm:text-xl md:text-2xl font-bold">
            Token Holders Total
          </h2>
          <div class="flex flex-row grow justify-between max-w-full">
            <div class="flex flex-row my-auto gap-2.5 md:gap-4 truncate">
              <div class="flex rounded-full text-gray-300 bg-gray-200 dark:bg-gray-700 dark:text-gray-900 my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10">
                <svg
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  class="h-1/2 m-auto"
                >
                  <path
                    fillRule="evenodd"
                    clipRule="evenodd"
                    d="M7.5 6a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM3.751 20.105a8.25 8.25 0 0 1 16.498 0 .75.75 0 0 1-.437.695A18.683 18.683 0 0 1 12 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 0 1-.437-.695Z"
                  ></path>
                </svg>
              </div>
              <h2 class="text-3xl sm:text-4xl md:text-5xl">
                {HolderCount} > 10 ORE
              </h2>
            </div>
          </div>
        </div>
        <div class="flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start">
          <div class="flex items-center justify-between">
            <h2 class="text-lg md:text-2xl font-bold my-auto">Holders</h2>
            <button
              class="rounded-full transition-colors my-auto px-2 py-1 sm:py-2 text-white bg-green-500 hover:bg-green-600 active:bg-green-700"
              onClick={() => {
                exportToExcel(HolderUsers);
              }}
            >
              Export
            </button>
          </div>
          <ul>
            <li class="flex flex-row justify-between py-3 gap-3 w-full font-semibold">
              <div>Address</div>
              <div>Amount(ORE)</div>
            </li>
            {HolderUsers.map((holder, index) => (
              <li
                key={index}
                class="flex flex-row justify-between py-3 gap-3 w-full px-2 rounded hover-100 active-200 transition-colors"
              >
                <div class="break-words" style={{ width: "80%" }}>
                  {holder.Address}
                </div>
                <div class="flex-none">{holder.Amount}</div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    );
  } catch (error) {
    console.error("Error getting token holders:", error);
    const container = document.getElementById("ore-holders-list");
    const root = ReactDOM.createRoot(container);
    root.render(
      <div>
        <h2>Token Holders</h2>
        <p>Error getting token holders: {error}</p >
      </div>
    );
  }
}
window.getTokenHolders = getTokenHolders;

function Dispatcher() {
  const { publicKey } = useWallet();
  useMemo(() => {
    let msg;
    if (publicKey) {
      msg = publicKey.toBuffer().toJSON().data;
    } else {
      msg = null;
    }
    try {
      const event = new CustomEvent("ore-pubkey", {
        detail: {
          pubkey: msg,
        },
      });
      window.dispatchEvent(event);
    } catch (err) {
      console.log(err);
    }
    return;
  }, [publicKey]);
}

function SignTransaction() {
  const { publicKey, signTransaction } = useWallet();
  const callback = useCallback(
    async (msg) => {
      try {
        const tx = Transaction.from(Buffer.from(msg.b64, "base64"));
        const signed = await signTransaction(tx);
        return signed.serialize().toString("base64");
      } catch (err) {
        console.log(err);
      }
    },
    [publicKey]
  );
  window.OreTxSigner = callback;
  return;
}