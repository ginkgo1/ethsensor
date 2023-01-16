import Web3 from 'web3'
import abi from '../assets/erc20_sensor_abi.json';

// https://blog.logrocket.com/integrate-web3-into-vue/

const INFURA_API_KEY = 'b155f7a79da2448cbf87dc3731b846e5'
const CONTRACT_ADDRESS = '0x0a88E84aAD539d0Ea0c060342cd6894b52c8082f';
const RASPBERRY_ADDRESS = '0x728cAc3C36589Df8b794181A257C4477089Ece69';

export default {

  async read_sensor() {

    const provider = new Web3.providers.HttpProvider(
      `https://goerli.infura.io/v3/${INFURA_API_KEY}`);
    const web3 = new Web3(provider);

    // contrato al que se va a llamar
    const contract_addr = CONTRACT_ADDRESS;
    // Loading the contract ABI
    const myContract = new web3.eth.Contract(abi, contract_addr);
    // dirección desde la que se hace la llamada (opcional, transacción local)
    const fromaddr = RASPBERRY_ADDRESS;

    // Some example calls how to read data from the smart contract

    const data = await myContract.methods.read(10).call({from: fromaddr});
    console.log('data read', data);
    return data;

  },

  async test() {
    return 42;
  },
}
