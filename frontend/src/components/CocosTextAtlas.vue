<template>
  <div
    data-widget="TextAtlas"
    :data-name="name"
    :data-atlas-file="atlasFile"
    :data-char-width="String(charWidth)"
    :data-char-height="String(charHeight)"
    :data-start-char="startChar"
    :style="rootStyle"
    class="cocos-textatlas"
  >{{ text }}</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  text: string
  atlasFile: string
  charWidth: number
  charHeight: number
  startChar?: string
  width?: string
  height?: string
}>(), {
  startChar: '.',
  width: 'auto',
  height: 'auto',
})

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  fontSize: props.charHeight + 'px',
  fontFamily: 'monospace',
  letterSpacing: '1px',
}))
</script>

<style scoped>
.cocos-textatlas {
  color: #fff;
  white-space: nowrap;
}
</style>
