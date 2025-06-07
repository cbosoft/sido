<script setup>
  const { data } = defineProps(['data']);
</script>

<template>
  <div class="node-editor">
    <div class="form" v-if="(!!data) && (!!data.selected_node)">
      <span class="key">node</span> 
      <input type="text" v-model="data.patch.nodes[data.selected_node].name"></input>
      <span class="key">kind</span>
      <span>Mux</span>
    </div>
  </div>
</template>

<style scoped>
.node-editor {
  font-family: monospace;
  padding: 10px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}
.form {
  display: grid;
  grid-template-columns: auto auto;
  gap: 10px 10px;
  align-items: center;
  padding-bottom: auto;
}
.key {
  color: #777;
  text-align: right;
}

input[type="text"] {
  background-color: transparent;
  color: #ddd;
  border: 1px solid #555;
  width: 100%;
  box-sizing: border-box;
  padding: 5px;
  border-radius: 0;
}
</style>
