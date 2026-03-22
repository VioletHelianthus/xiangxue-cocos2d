<template>
  <div
    data-widget="PageView"
    :data-name="name"
    :data-direction="direction"
    :style="rootStyle"
    class="cocos-pageview"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  direction?: 'horizontal' | 'vertical'
  width?: string
  height?: string
}>(), {
  direction: 'horizontal',
  width: '300px',
  height: '400px',
})

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  overflow: 'hidden',
  position: 'relative' as const,
}))
</script>

<style scoped>
.cocos-pageview {
  border: 1px dashed #666;
  background: #1a1a1a;
}
</style>
