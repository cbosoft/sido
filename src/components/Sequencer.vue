<script setup>
  import { reactive, computed } from "vue";

  const data = reactive({
    n_divisions: 4,
    n_beats: 4,
    possible_divisions: [1, 2, 4, 8, 16],
    events: {}
  });

  function get_matrix() {
    const matrix = [];
    const w = data.n_divisions * data.n_beats;
    for (var i = 0; i < w; i++) {
      const column = [];
      for (var j = 0; j < 8; j++) {
        column.push({ i, j, is_pressed: false });
      }
      matrix.push(column);
    }

    for (const [_, ev] of Object.entries(data.events)) {
      matrix[ev.i][ev.j].is_pressed = ev.is_pressed;
    }
    return matrix;
  };

  function toggle_note(note) {
    const { i, j, is_pressed } = note;
    data.events[`${i}${j}`] = { i, j, is_pressed: !is_pressed };
  }

  function select_n_beats(nbeats) {
    update_length({ nbeats });
  }

  function select_n_divisions(ndiv) {
    update_length({ ndiv });
  }

  function update_length({ndiv, nbeats}) {
    ndiv = ndiv ?? data.n_divisions;
    nbeats = nbeats ?? data.n_beats;
    const new_events = {};
    const w = ndiv*nbeats;
    for (const [k, ev] of Object.entries(data.events)) {
      if (ev.i < w) {
        new_events[k] = ev;
      }
    }
    data.events = new_events;
    data.n_divisions = ndiv;
    data.n_beats = nbeats;
  }

  const matrix = computed(() => get_matrix());
</script>

<template>
  <div class="sequencer-outer">
    <div class="sequencer">
      <div class="greebles">
        <div class="bl"></div>
        <div class="r"></div>
      </div>
      <div class="controls">
        <div class="divisions-selector">
          <div :class="`divisions-radio d${cfg} ${cfg==data.n_divisions ? 'selected' : ''}`" v-for="cfg in data.possible_divisions" @click="() => select_n_divisions(cfg)">
            <div class="division-circle" v-for="x in cfg"></div>
          </div>
        </div>
        <div class="beats-selector">
          <div :class="`beats-marker b${data.n_beats}`"></div>
          <div class="beats-radio" v-for="x in 4" @click="() => select_n_beats(x)"><div class="beats-circle"></div></div>
        </div>
      </div>
      <div class="sequence">
        <div class="division" v-for="(division, i) in matrix">
          <div :class="`note ${note.is_pressed ? 'pressed' : ''} w${data.n_divisions*data.n_beats}`" v-for="note in division" @click="() => toggle_note(note)">
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.greebles {
  position: absolute;
  inset: 0;
  display: none;
}
.greebles .bl {
  position: absolute;
  left: 0;
  bottom: 0;
  width: 30%;
  height: 20%;
  border: 2px solid #777;
  margin: 25px;
  border-right: none;
  border-top: none;
}
.greebles .r {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  height: 30%;
  border: 2px solid #777;
  margin: auto;
  margin-right: 25px;
  border-bottom: none;
  border-top: none;
  border-left: none;
}
.sequencer-outer {
}

.sequencer {
  position: relative;
  width: 1200px;
  height: 600px;
  border: 2px solid #ddd;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 50px;
  gap: 50px;
  transition: all 0.5s;
}
.controls {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  width: 100%;
}
.sequence {
  display: flex;
  flex-direction: row;
  height: min-content;
  box-sizing: border-box;
  justify-content: center;
  align-items: center;
  flex-grow: 1;
}

.division {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}
.note {
  flex-grow: 1;

  border: 2px solid #ddd;
  border-radius: 10%;
  --w: 40px;
  width: var(--w);
  height: var(--w);
  margin: calc(var(--w)*0.25);
}

.note.w24 { --w: 30px; }
.note.w32 { --w: 22px; }
.note.w48 { --w: 12px; }
.note.w64 { --w: 11px; border-width: 1px; }
.note.pressed { background-color: #eee; }
.note.pressed, .beats-circle, .division-circle {
  --blur-r: 20px;
  --glow-r: 2px;
  --glow-c: #eeeeffaa;
  -webkit-box-shadow:0px 0px var(--blur-r) var(--glow-r) var(--glow-c);
  -moz-box-shadow: 0px 0px var(--blur-r) var(--glow-r) var(--glow-c);
  box-shadow: 0px 0px var(--blur-r) var(--glow-r) var(--glow-c);
}

.divisions-selector {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
}
.divisions-radio {
  gap: 2px;
  height: min-content;
  border: 2px solid transparent;
  border-radius: 20%;
  padding: 10px;
}
.divisions-radio.d1 {
  display: flex;
}
.divisions-radio.d2 {
  display: flex;
  flex-direction: row;
}
.divisions-radio.d4 {
  display: grid;
  grid-template-columns: auto auto;
}
.divisions-radio.d8 {
  display: grid;
  grid-template-columns: auto auto auto auto;
}
.divisions-radio.d16 {
  display: grid;
  grid-template-columns: auto auto auto auto;
}
.division-circle, .beats-circle {
  --r: 2px;
  --d: calc(var(--r)*2);
  width: var(--d);
  height: var(--d);
  background-color: #ddd;
  border-radius: var(--r);
}
.divisions-radio.selected {
  border-color: #ddd;
}
.beats-selector {
  --w: 20px;
  --g: 5px;
  --s: calc(var(--w) + var(--g));
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--g);
  position: relative;
}
.beats-radio {
  width: var(--w);
  height: var(--w);
  display: flex;
  align-items: center;
  justify-content: center;
}
.beats-marker {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  border: 2px solid white;
  border-radius: calc(var(--w)*0.5);
  width: var(--w);
  padding: 0px;
  margin: auto;
  height: var(--w);
  box-sizing: border-box;
  z-index: -1;
}
.beats-marker.b1 { width: calc(var(--w)*1 + var(--g)*0); }
.beats-marker.b2 { width: calc(var(--w)*2 + var(--g)*1); }
.beats-marker.b3 { width: calc(var(--w)*3 + var(--g)*2); }
.beats-marker.b4 { width: calc(var(--w)*4 + var(--g)*3); }
</style>
