<template>
  <div
    data-widget="Slider"
    :data-name="name"
    :data-bar="bar"
    :data-ball="ball"
    :data-progress="progress"
    :data-value="String(percent)"
    :style="rootStyle"
    class="cocos-slider"
  >
    <div class="cocos-slider__track">
      <div class="cocos-slider__fill" :style="{ width: percent + '%' }"></div>
    </div>
    <div class="cocos-slider__thumb" :style="{ left: percent + '%' }"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  percent?: number
  bar?: string
  ball?: string
  progress?: string
  width?: string
  height?: string
}>(), {
  percent: 50,
  width: '200px',
  height: '30px',
})

const rootStyle = computed(() => ({
  position: 'relative' as const,
  width: props.width,
  height: props.height,
  display: 'flex',
  alignItems: 'center',
}))
</script>

<style scoped>
.cocos-slider__track {
  width: 100%;
  height: 6px;
  background: #555;
  border-radius: 3px;
  overflow: hidden;
}
.cocos-slider__fill {
  height: 100%;
  background: #4a9;
  border-radius: 3px;
}
.cocos-slider__thumb {
  position: absolute;
  width: 16px;
  height: 16px;
  background: #fff;
  border: 2px solid #4a9;
  border-radius: 50%;
  transform: translateX(-50%);
  cursor: pointer;
}
</style>
