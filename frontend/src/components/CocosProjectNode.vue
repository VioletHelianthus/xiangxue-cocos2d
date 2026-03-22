<template>
  <div
    data-widget="ProjectNode"
    :data-name="name"
    :data-file="file"
    :style="rootStyle"
    class="cocos-projectnode"
  >
    <span class="cocos-projectnode__label">📎 {{ file }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  file: string
  width?: string
  height?: string
}>(), {
  width: '100px',
  height: '100px',
})

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
}))
</script>

<style scoped>
.cocos-projectnode {
  border: 1px dashed #888;
  background: #2a2a3a;
  border-radius: 4px;
}
.cocos-projectnode__label {
  font-size: 12px;
  color: #aaa;
  text-align: center;
  word-break: break-all;
  padding: 4px;
}
</style>
