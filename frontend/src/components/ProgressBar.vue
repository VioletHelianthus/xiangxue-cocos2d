<template>
  <div
    data-widget="ProgressBar"
    :data-name="name"
    :data-value="String(value)"
    :data-max="String(max)"
    :style="rootStyle"
  >
    <!-- Background track -->
    <div class="bar-track" :style="trackStyle">
      <img class="bar-cap-left" :src="bgLeft" />
      <img class="bar-mid" :src="bgMid" />
      <img class="bar-cap-right" :src="bgRight" />
    </div>
    <!-- Fill -->
    <div class="bar-fill" :style="fillStyle">
      <img class="bar-cap-left" :src="fillLeft" />
      <img class="bar-mid" :src="fillMid" />
      <img class="bar-cap-right" :src="fillRight" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  name: string
  value: number
  max?: number
  color?: 'green' | 'red' | 'blue' | 'yellow'
  width?: string
  height?: string
}>(), {
  max: 100,
  color: 'green',
  width: '200px',
  height: '24px',
})

const percent = computed(() =>
  Math.min(100, Math.max(0, (props.value / props.max) * 100))
)

const colorCapitalized = computed(() =>
  props.color.charAt(0).toUpperCase() + props.color.slice(1)
)

const bgLeft = '/ui-pack-rpg/PNG/barBack_horizontalLeft.png'
const bgMid = '/ui-pack-rpg/PNG/barBack_horizontalMid.png'
const bgRight = '/ui-pack-rpg/PNG/barBack_horizontalRight.png'

const fillLeft = computed(() => `/ui-pack-rpg/PNG/bar${colorCapitalized.value}_horizontalLeft.png`)
const fillMid = computed(() => `/ui-pack-rpg/PNG/bar${colorCapitalized.value}_horizontalMid.png`)
const fillRight = computed(() => `/ui-pack-rpg/PNG/bar${colorCapitalized.value}_horizontalRight.png`)

const rootStyle = computed(() => ({
  position: 'relative' as const,
  width: props.width,
  height: props.height,
}))

const trackStyle = {
  position: 'absolute' as const,
  inset: '0',
  display: 'flex',
}

const fillStyle = computed(() => ({
  position: 'absolute' as const,
  top: '0',
  left: '0',
  bottom: '0',
  width: `${percent.value}%`,
  display: 'flex',
  overflow: 'hidden',
}))
</script>

<style scoped>
.bar-cap-left,
.bar-cap-right {
  height: 100%;
  flex-shrink: 0;
}

.bar-mid {
  height: 100%;
  flex-grow: 1;
}
</style>
