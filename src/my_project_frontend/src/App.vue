<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let msg = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const newMsg = target.querySelector('#newMsg').value;
  await my_project_backend.set_msg(newMsg);
  await get_msg()
}

async function get_Msg() {
  msg.value = await my_project_backend.get_msg()
}

get_msg()
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="newMsg">Enter your newMsg: &nbsp;</label>
      <input id="newMsg" alt="newMsg" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="msg">{{ msg }}</section>
  </main>
</template>