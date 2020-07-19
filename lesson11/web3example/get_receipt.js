const Web3 = require("web3");
const web3 = new Web3('http://127.0.0.1:8545');

const lookup = async() => {
  let b = await web3.eth.getBalance("0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b");
  console.log(b);

  let hash = "0xa70629cf9c9b8767f483b5d81f55d71d0d41a2ff267a7cbc82267ea11f4a9596";

  let rp = await web3.eth.getTransactionReceipt(hash);
  // let tx = await web3.eth.getTransaction(hash);
  console.log(rp);
  // console.log(tx);
};

lookup().then();
