<template>
      <div class="min-h-screen flex">
      <div class="flex flex-col w-56 bg-slate-800 rounded-l-3xl">
        <div class="flex-col min-h-fit">
          <canvas id="skin_container"></canvas>
          <select
            aria-label="select box"
            onchange="switchMode(event)"
            class="
              w-full
              focus:ring-2
              focus:ring-offset-2
              focus:ring-gray-500
              focus:outline-none
              pl-10
              py-2
              appearance-none
              flex
              items-center
              h-12
              border border-gray-700
              dark:border-gray-50
              rounded-2xl
              text-sm
              leading-5
              dark:bg-gray-900 dark:text-gray-50
            "
          >
            <option selected value="auto">Welcome, KingTux</option>
          </select>
        </div>

        <ul class="flex flex-col min-h-fit">
          <li
            class="
              flex flex-row
              items-center
              h-12
              transform
              ease-in
              duration-200
              text-slate-50
              hover:text-slate-100
            "
          >
            <span class="font-bold text-sm">News</span>
          </li>
        </ul>
      </div>
    </div>
</template><script lang="ts">
import { defineComponent, ref } from "vue";
import { SkinViewer } from "skinview3d";
export default defineComponent({
  setup() {},
  mounted() {
    let value = <HTMLCanvasElement> document.getElementById("skin_container");
    if (value != null) {
      const skinViewer = new SkinViewer({
        canvas: value,
        width: 200,
        height: 200,

        skin: "https://crafatar.com/skins/d087006b-d72c-4cdf-924d-6f903704d05c",
      });
      // Set the background to a panoramic image
      skinViewer.loadPanorama(
        "https://github.com/bs-community/skinview3d/blob/master/examples/img/panorama.png?raw=true"
      );
    }
  },
});
</script>