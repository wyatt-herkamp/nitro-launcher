
<template>
  <div class="flex flex-row">
    <div class="w-full">
      <router-view :key="$route.fullPath" />
    </div>
    <SideBar />
  </div>
</template>


<script lang="ts">
import { defineComponent, ref } from "vue";
import SideBar from "~/components/nav/SideBar.vue";
export default defineComponent({
  components: { SideBar },
  setup() {
    console.log("Hey");
  },
});
</script>