<template>
<div>
  <p>Using Goerli Testnet Network</p>
  <p>Contract hash: <a href="https://goerli.etherscan.io/address/0x0a88E84aAD539d0Ea0c060342cd6894b52c8082f">0x0a88E84aAD539d0Ea0c060342cd6894b52c8082f</a></p>
  <p>IoT node hash: <a href="https://goerli.etherscan.io/address/0x728cAc3C36589Df8b794181A257C4477089Ece69">0x728cAc3C36589Df8b794181A257C4477089Ece69</a></p>

0x728cAc3C36589Df8b794181A257C4477089Ece69
  <p v-if="loading">Loading data...</p>
  <p v-if="error">Error reading smart contract: {{ error }}</p>
</div>

<div v-if="values">
  <h2>Sensor data</h2>
  <p>These data has been retrieved from Goerli testnet by calling smart contract
    from your browser, using web3 library. Reading only the 10 more recent
    values. This call requires no gas and no private key (not modifying state).</p>
  <p>These values were recorded into the blockchain from an IoT node using
     Rust client.</p>
  <table class="dataset">
    <thead>
      <th>Date</th>
      <th>Temperature</th>
    </thead>
    <tbody>
      <tr v-for="(row, idx) in values" :key="idx">
        <td>{{ row.date }}</td>
        <td>{{ row.temp }}</td>
      </tr>
    </tbody>
  </table>
</div>


<!-- gráfico con los datos -->
<Chart
  :size="{ width: 800, height: 400 }"
  :data="values"
  :margin="margin"
  :direction="direction">

  <template #layers>
    <Grid strokeDasharray="2,2" />
    <Line :dataKeys="['date', 'temp']" type="monotone" />
  </template>
</Chart>

<div v-if="raw">
  <p><b>Raw data</b> returned from smart contract call:</p>
  <p>{{ raw }}</p>
</div>

</template>

<script>
import bc from '/src/services/blockchain.js'
import { Chart, Grid, Line } from 'vue3-charts'


export default {
  data() {
    return {
      raw: null,
      values: [],
      error: null,
      loading: false,
      direction: 'horizontal',
      margin: { left:0, top: 20, right: 20, bottom: 0 },
    }
  },
  components: {
    Chart, Grid, Line,
  },
  methods: {
    async get_sensor_data() {
      try {
        // obtiene los datos de la blockchain
        this.loading = true;
        const data = await bc.read_sensor();
        this.loading = false;
        this.error = null;
        this.raw = data;
        // reorganiza y ordena los datos para el gráfico
        const vec = [];
        for (let i = 0; i < data[0].length; ++i) {
          let date = new Date(data[1][i] * 1000).toLocaleString("es");
          vec.push({ date: date, temp: data[0][i] / 100 });
        }
        this.values = vec.sort((a, b) => a.date > b.date);
      }
      catch(e) {
        console.log("Error reading smart contract", e);
        this.values = null;
        this.loading = false;
        this.error = e;
        this.raw = null;
      }
    }
  },
  mounted() {
    console.log('Component mounted', bc);
    this.get_sensor_data();
  },
}

</script>

<style>
.layer-axis-x .tick text {
  transform: rotate(-90deg);
}

.dataset td {
  padding: 0 2em;
}
.dataset th {
  font-weight: bold;
}

div {
  margin-top: 1em;
}
table {
  margin-top: 1em;
}
</style>
