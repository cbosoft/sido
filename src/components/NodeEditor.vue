<script setup>
  import { computed, reactive } from 'vue';
  import Dropdown from "./Dropdown.vue";
  const { data } = defineProps(['data']);

  const node_kinds = {
    // Oscillators
    Sine: { inputs: 1, outputs: 1 },
    Saw: { inputs: 1, outputs: 1 },
    Square: { inputs: 1, outputs: 1 },
    SpecifiedSine: { inputs: 1, outputs: 1, value: 1.0 },
    SpecifiedSaw: { inputs: 1, outputs: 1, value: 1.0 },
    SpecifiedSquare: { inputs: 1, outputs: 1, value: 1.0 },

    // Sampling
    Sample: { inputs: 0, outputs: 1, path: '', looped: false },

    // Noise
    WhiteNoise: { inputs: 1, outputs: 1 },
    PinkNoise: { inputs: 1, outputs: 1 },
    BrownNoise: { inputs: 1, outputs: 1 },

    Constant: { inputs: 0, outputs: 1, value: 1.0 },
    MultChannels: { inputs: 2, outputs: 1 },

    // Effects
    // FlangerSin: { strength: f32, min_delay: f32, max_delay: f32, sin_freq: f32 },
    // ADSR: { attack: f32, decay: f32, sustain: f32, release: f32 },

    // Filters
    PinkPass:  { inputs: 2, outputs: 1 },
    LowPass: { inputs: 2, outputs: 1, gain: 1.0 },
    HighPass: { inputs: 2, outputs: 1, gain: 0.0 },
    BandPass: { inputs: 2, outputs: 1, gain: 0.0 },
    FixedLowPass: { inputs: 2, outputs: 1, freq: 0.0, gain: 0.0 },
    FixedHighPass: { inputs: 2, outputs: 1, freq: 0.0, gain: 0.0 },
    FixedBandPass: { inputs: 2, outputs: 1, freq: 0.0, gain: 0.0 },

    // Maths
    SumChannels: { inputs: 2, outputs: 1 },
    MultChannels: { inputs: 2, outputs: 1 },
    Mux: { inputs: 2, outputs: 1 },
    MultConstant: { inputs: 1, outputs: 1, multiplier: 1.0 },
  };

  const selection = computed(()=>{
    if (!data) return;
    if (!data.selected_nodes) return;
    const l = data.selected_nodes.length;
    if (l == 0) return;
    const node_id = data.selected_nodes[l-1];
    const node = data.patch.nodes[node_id];
    return { node_id, node };
  });

  const node_id = computed(()=>{
    if (data.selected_nodes.length == 0) {
      return;
    }
    return data.selected_nodes[0];
  });

</script>

<template>
  <div class="node-editor">
    <div class="form" v-if="!!selection">
      <!--<span class="key">node</span> <input type="text" v-model="selection.node_id"></input>-->
      <span class="key">id</span>
      <div>{{ node_id }}</div>
      <template v-if="selection.node.reserved">
        <span class="key">kind</span>
        <div>{{ data.patch.nodes[node_id].kind }}</div>
        <span class="key wide">reserved</span>
      </template>
      <template v-else>
        <span class="key">kind</span>
        <Dropdown v-model:value="data.patch.nodes[node_id].kind" :options="Object.keys(node_kinds)" />
      </template>
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
  grid-template-columns: 1fr 1fr;
  gap: 0 10px;
  align-items: start;
  padding-bottom: auto;
}
* {
  border: 1px solid transparent;
}
.key {
  color: #777;
  text-align: right;
}
.key.wide {
  grid-column-start: 1;
  grid-column-end: 3;
  text-align: center;
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
