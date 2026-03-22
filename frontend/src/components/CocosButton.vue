<template>
  <div
    data-widget="Button"
    :data-name="name"
    :data-pressed="pressed"
    :data-disabled="disabled"
    :style="rootStyle"
    :src="normal"
    class="cocos-button"
  >
    <img v-if="normal" :src="normal" class="cocos-button__bg" />
    <span v-if="$slots.default" class="cocos-button__text"><slot /></span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  normal?: string
  pressed?: string
  disabled?: string
  width?: string
  height?: string
  fontSize?: string
}>(), {
  width: '120px',
  height: '40px',
  fontSize: '14px',
})

const rootStyle = computed(() => ({
  position: 'relative' as const,
  width: props.width,
  height: props.height,
  display: 'inline-flex',
  alignItems: 'center',
  justifyContent: 'center',
  cursor: 'pointer',
  userSelect: 'none' as const,
}))
</script>

<style scoped>
.cocos-button {
  border: 1px solid #666;
  border-radius: 4px;
  background: #444;
  color: #fff;
}
.cocos-button:hover {
  background: #555;
}
.cocos-button__bg {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: fill;
}
.cocos-button__text {
  position: relative;
  z-index: 1;
  font-size: v-bind(fontSize);
}
</style>
