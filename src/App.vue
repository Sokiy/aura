<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event"
import { reactive } from "vue";

const state = reactive({
  infoLoading: true,
  info: ""
})


function get_pic_binary(path: any) {
  console.log("==============")
  invoke('get_pic_binary', { path: path }).then((res: any) => {
    let binary_data = window.URL.createObjectURL(new Blob([new Uint8Array(res)], { type: 'image/png' }));

    state.infoLoading = false;
    state.info = binary_data;
  })
}

//监听界面的拖拽事件
listen('tauri://file-drop', (files: any) => {
  get_pic_binary(files.payload[0]);
});


</script>

<template>
  <div v-if="!state.infoLoading">
    <img :src="state.info" alt="" class="content">
  </div>
</template>

<style scoped>
.content {
  height: 500px;
  height: 500px;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
