<template>
  <div
    data-widget="ListView"
    :data-name="name"
    :data-direction="direction"
    :data-item-margin="String(itemMargin)"
    :data-gravity="gravity"
    :style="rootStyle"
    class="cocos-listview"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  direction?: 'vertical' | 'horizontal'
  itemMargin?: number
  gravity?: 'left' | 'right' | 'center-horizontal' | 'top' | 'bottom' | 'center-vertical'
  width?: string
  height?: string
}>(), {
  direction: 'vertical',
  itemMargin: 0,
  width: '300px',
  height: '400px',
})

const rootStyle = computed(() => ({
  width: props.width,
  height: props.height,
  overflow: 'auto',
  display: 'flex',
  flexDirection: props.direction === 'horizontal' ? 'row' as const : 'column' as const,
  gap: props.itemMargin + 'px',
}))
</script>

<style scoped>
.cocos-listview {
  border: 1px solid #444;
  background: #1a1a1a;
}
</style>
