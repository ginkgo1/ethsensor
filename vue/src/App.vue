<script setup>
import SensorEth from './components/SensorEth.vue'
</script>

<template>
  <header>
    <img alt="Ethereum logo" class="logo" src="./assets/ethereum.png" width="125" height="125" />

    <div class="wrapper">
      <p>Ethereum sensor data storage</p>
    </div>
  </header>

  <main>
    <SensorEth />
    <!-- <TheWelcome /> -->
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    /* display: flex; 
    place-items: center;
    padding-right: calc(var(--section-gap) / 2); */
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    /* display: flex;
    place-items: flex-start;
    flex-wrap: wrap; */
  }
}
</style>
