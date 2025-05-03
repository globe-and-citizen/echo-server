<script setup>
import * as interceptor_wasm from "interceptor-wasm";

defineProps({
  msg: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <h3>
      Your data is approved with my signature.
    </h3>
  </div>

  <div>
    <div class="action-row">
      <button @click="interceptor_wasm.test_wasm">
        test_wasm
      </button>
    </div>

    <div class="action-row">
      <button @click="interceptor_wasm.persistence_check">
        persistence_check
      </button>
    </div>

    <div class="action-row">
      <button
          @click="() => interceptor_wasm.check_encrypted_tunnel().
        then(val => console.log('CheckEncryptedTunnel Result:', val)).
        catch(err => console.error('CheckEncryptedTunnel Error:', err))"
      >
        check_encrypted_tunnel
      </button>
    </div>

    <div class="action-row">
      <button
          @click="() => interceptor_wasm.init_encrypted_tunnel({'hello': 'world'}).
        then(val => console.log('InitEncryptedTunnel Result:', val)).
        catch(err => console.error('InitEncryptedTunnel Error:', err))"
      >
        init_encrypted_tunnel
      </button>
    </div>

    <div class="action-row">
      <button
          @click="interceptor_wasm.fetch('hello').
        then(val => console.log('Fetch Result:', val)).
        catch(err => console.error('Fetch Error:', err))"
      >
        fetch
      </button>
    </div>

    <div class="action-row">
      <button
          @click="interceptor_wasm.get_static('hello').
        then(val => console.log('GetStatic Result:', val)).
        catch(err => console.error('GetStatic Error:', err))"
      >
        get_static
      </button>
    </div>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}

</style>
