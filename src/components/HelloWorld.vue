<template>

  <button @click="invokeTest"
          class="relative inline-flex items-center justify-center px-10 overflow-hidden font-mono font-medium tracking-tighter text-white bg-gray-800 rounded-lg group"
  >
                  <span
                      class="absolute w-0 h-max transition-all duration-500 ease-out bg-slate-900 rounded-full group-hover:w-56 group-hover:h-56"
                  ></span>
    <span
        class="absolute inset-0 w-full -mt-1 rounded-lg opacity-30 bg-gradient-to-b from-transparent via-transparent to-gray-700"
    ></span>
    <span class="text-white">Invoke Test</span>
  </button>

</template>
<script lang="ts">
import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";

export default defineComponent({

  methods: {
    invokeTest() {
      invoke("get_minecraft_versions")
    },
  },
});
</script>
<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
