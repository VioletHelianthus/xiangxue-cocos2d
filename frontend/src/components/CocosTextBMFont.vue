<template>
  <div
    data-widget="TextBMFont"
    :data-name="name"
    :data-fnt-file="fntFile"
    :style="rootStyle"
    class="cocos-bmfont"
  >{{ text }}</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  text: string
  fntFile: string
  width?: string
  height?: string
  fontSize?: string
}>(), {
  width: 'auto',
  height: 'auto',
  fontSize: '24px',
})

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  fontSize: props.fontSize,
  fontFamily: 'monospace',
  letterSpacing: '2px',
}))
</script>

<style scoped>
.cocos-bmfont {
  color: #ff0;
  white-space: nowrap;
}
</style>
