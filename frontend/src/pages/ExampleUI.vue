<template>
  <div data-name="root" :style="rootStyle">
    <!-- Top bar -->
    <div data-name="topBar" :style="topBarStyle">
      <span data-name="title" :style="titleStyle">RPG Status</span>
    </div>

    <!-- Status section -->
    <div data-name="statusSection" :style="statusSectionStyle">
      <div data-name="hpRow" :style="rowStyle">
        <span :style="labelStyle">HP</span>
        <ProgressBar name="hpBar" :value="850" :max="1060" color="green" width="180px" height="20px" />
        <span :style="valueStyle">850 / 1060</span>
      </div>

      <div data-name="mpRow" :style="rowStyle">
        <span :style="labelStyle">MP</span>
        <ProgressBar name="mpBar" :value="230" :max="400" color="blue" width="180px" height="20px" />
        <span :style="valueStyle">230 / 400</span>
      </div>

      <div data-name="expRow" :style="rowStyle">
        <span :style="labelStyle">EXP</span>
        <ProgressBar name="expBar" :value="6500" :max="10000" color="yellow" width="180px" height="20px" />
        <span :style="valueStyle">65%</span>
      </div>
    </div>

    <!-- Action button -->
    <button data-name="attackBtn" :style="btnStyle">Attack</button>
  </div>
</template>

<script setup lang="ts">
const rootStyle = {
  width: '640px',
  height: '960px',
  display: 'flex',
  flexDirection: 'column' as const,
  alignItems: 'center',
  padding: '40px 20px',
  gap: '30px',
  background: '#1a1a2e',
  color: '#eee',
  fontFamily: 'sans-serif',
}

const topBarStyle = {
  width: '100%',
  display: 'flex',
  justifyContent: 'center',
  padding: '10px 0',
}

const titleStyle = {
  fontSize: '28px',
  fontWeight: 'bold',
}

const statusSectionStyle = {
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '16px',
  width: '400px',
}

const rowStyle = {
  display: 'flex',
  alignItems: 'center',
  gap: '12px',
}

const labelStyle = {
  width: '40px',
  fontSize: '16px',
  fontWeight: 'bold',
}

const valueStyle = {
  fontSize: '14px',
  color: '#aaa',
  minWidth: '90px',
  textAlign: 'right' as const,
}

const btnStyle = {
  width: '200px',
  height: '50px',
  fontSize: '18px',
  fontWeight: 'bold',
  cursor: 'pointer',
  border: 'none',
  borderRadius: '4px',
  background: '#e94560',
  color: '#fff',
}
</script>
