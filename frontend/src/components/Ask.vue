<template>
  <div class="body">
    <h2>Input your data</h2>
    <div class="data">
      <div class="input-container">
        <label>
          <span>Data:</span>
          <textarea v-model="data"/>
        </label>
        <label>
          <span>Signature:</span>
          <textarea v-model="signature" class="signature-field" placeholder="Signature to verify"/>
        </label>
      </div>

      <label class="public-key">
        <span>Verify Key:</span>
        <textarea v-model="publicKey" readonly class="public-key-field"/>
      </label>

      <div class="action-row">
        <div class="button-group">
          <button @click="sign">Sign</button>
          <button @click="verifyBackend">Verify</button>
          <button class="clear-btn" @click="clearAll">Clear</button>
        </div>
        <div class="verify-result" v-if="verifyResult">
          <p>{{ verifyResult }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import axios from "axios"
import {Buffer} from 'buffer';

const data = ref("")
const signature = ref("")
const publicKey = "5d036a858ce89f844491762eb89e2bfbd50a4a0a0da658e4b2628b25b117ae09"
const verifyResult = ref("")
const PROXY_ADDR = "http://localhost:6191"

async function sign() {
  console.log("Signing data:", data.value)
  let res = await axios.post(`${PROXY_ADDR}/sign`,
      {data: data.value},
      {
        headers: {
          'Content-Type': "application/json",
          'Accept': "application/json",
        }
      }
  )
  signature.value = res.data.result
}

async function verifyBackend() {
  console.log("Verifying signature:", signature.value)
  let res = await axios.post(`${PROXY_ADDR}/verify`,
      {
        data: data.value,
        signature: signature.value
      },
      {
        headers: {
          'Content-Type': "application/json",
          'Accept': "application/json",
        }
      }
  )
  verifyResult.value = res.data.result

  if (res.data.result === "valid") {
    verifyResult.value = "✅ Valid signature"
  } else {
    verifyResult.value = "❌ Invalid signature"
  }
}

async function verifyFrontend() {
  try {
    var publicKeyBuffer = Buffer.from(publicKey, 'hex')
    var signatureBuffer = Buffer.from(signature.value, 'hex')
    var dataBuffer = Buffer.from(data.value, 'utf-8')

    const subtlePublicKey = await crypto.subtle.importKey(
        'raw', // Raw format for Ed25519
        publicKeyBuffer,
        {name: "Ed25519"}, // Algorithm name
        true, // Extractable
        ['verify'] // Key usage
    );

    const isValid = await crypto.subtle.verify(
        {
          name: 'Ed25519',
        },
        subtlePublicKey,
        signatureBuffer,
        dataBuffer
    );

    if (isValid) {
      verifyResult.value = "✅ Valid signature"
    } else {
      verifyResult.value = "❌ Invalid signature"
    }
  } catch (err) {
    console.error("Error verifying signature:", err)
    verifyResult.value = "❌ Invalid signature"
  }
}

function clearAll() {
  data.value = ""
  signature.value = ""
  verifyResult.value = ""
}
</script>


<style scoped>
div.body {
  margin: 5%;
}

h2 {
  font-weight: 300;
  font-size: 2.6rem;
  margin-bottom: 2rem;
}

.data {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.input-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

label {
  display: flex;
  flex-direction: column;
  font-size: 1.1rem;
  color: #f0f0f0;
}

label span {
  margin-bottom: 0.5rem;
}

textarea,
input {
  width: 100%;
  padding: 0.5rem 1rem;
  font-size: 1rem;
  background-color: #1e1e1e;
  color: #f0f0f0;
  border: 2px solid #ccc;
  border-radius: 8px;
  outline: none;
  transition: border-color 0.2s ease-in-out;
}

textarea:focus,
input:focus {
  border-color: #42b983;
}

textarea {
  min-height: 10rem;
  max-height: 30rem;
  resize: vertical;
  line-height: 1.5;
  box-sizing: border-box;
}

.input-container textarea[readonly] {
  background-color: #1e1e1e; /* Keep the background dark */
  color: rgba(240, 240, 240, 0.5); /* Dim the text color */
  cursor: not-allowed; /* Show 'not-allowed' cursor */
  resize: none; /* Optionally, prevent resizing if it's readonly */
}

.public-key-field {
  width: 100%;
  min-height: 3rem;
  padding: 0.5rem 1rem;
  font-size: 1rem;
  background-color: #1e1e1e;
  color: #f0f0f0;
  border: 2px solid #ccc;
  border-radius: 8px;
  outline: none;
  resize: none;
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
  font-family: monospace;
}

.signature-field {
  width: 100%;
  min-height: 5rem;
  padding: 0.5rem 1rem;
  font-size: 1rem;
  background-color: #1e1e1e;
  color: #f0f0f0;
  border: 2px solid #ccc;
  border-radius: 8px;
  outline: none;
  resize: none;
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
  font-family: monospace;
}

.action-row {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-top: 1rem;
  flex-wrap: wrap;
}

.button-group {
  display: flex;
  gap: 0.5rem;
}

.button-group button {
  background-color: #42b983;
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 0.6rem 1.2rem;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;
}

.button-group button:hover {
  background-color: #369b6d;
}

.verify-result p {
  margin: 0;
  padding: 0.5rem 1rem;
  font-size: 1rem;
  font-weight: 500;
  color: #ffffff;
  white-space: pre-wrap;
}

</style>
