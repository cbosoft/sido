<script setup>
  import { reactive } from 'vue';
  const model = defineModel('value');

  const data = defineProps(['options']);
  const state = reactive({ opened: false });

  function onclick() {
    state.opened = !state.opened;
  }

  function select_option(v) {
    model.value = v;
  }
</script>

<template>
  <div class="dropdown" @click="onclick">
    <div>{{ model }}</div>
    <div class="dd-value" v-if="state.opened" v-for="opt of data.options" @click="() => select_option(opt)">{{ opt }}</div>
  </div>
</template>

<style scoped>
.dropdown {
  display: flex;
  flex-direction: column;
  border: 1px solid #555;
  padding: 0 5px;
}

.dropdown:hover {
  border-color: #777;
}

.dropdown:active {
  border-color: #ddd;
}

.dd-value {
  color: #555;
}
.dd-value:hover {
  color: #777;
}
.dd-value:active {
  color: #ddd;
}

</style>
