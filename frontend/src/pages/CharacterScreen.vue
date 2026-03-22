<template>
  <div data-name="root" :style="rootStyle">

    <!-- Top Navigation Bar -->
    <div data-name="topNav" :style="topNavStyle">
      <button v-for="tab in tabs" :key="tab.id"
              :data-name="'tab' + tab.id"
              :style="tabBtnStyle"
              data-pressed="buttonLong_beige_pressed.png">
        {{ tab.label }}
      </button>
    </div>

    <!-- Main Content Area -->
    <div data-name="mainContent" :style="mainContentStyle">

      <!-- Character Info Header -->
      <div data-name="charHeader" :style="charHeaderStyle">
        <img data-name="avatar" src="/ui-pack-rpg/PNG/iconCircle_grey.png"
             style="width:100px;height:100px"/>
        <div data-name="charInfo" :style="charInfoStyle">
          <span data-name="charName" style="width:200px;height:30px">{{ char.name }}</span>
          <span data-name="charClass" style="width:200px;height:22px">{{ char.class }} Lv.{{ char.level }}</span>
          <div data-name="expSection" style="display:flex;flex-direction:row;align-items:center;gap:8px">
            <span data-name="expLabel" style="width:36px;height:20px">EXP</span>
            <ProgressBar name="expBar" :value="char.exp" :max="char.expMax" color="yellow"
                         width="180px" height="18px" />
            <span data-name="expText" style="width:80px;height:18px">{{ formatNum(char.exp) }} / {{ formatNum(char.expMax) }}</span>
          </div>
        </div>
        <div data-name="goldBox" style="display:flex;flex-direction:row;align-items:center;gap:4px">
          <img data-name="goldIcon" src="/ui-pack-rpg/PNG/dotYellow.png" style="width:24px;height:24px"/>
          <span data-name="goldText" style="width:80px;height:22px">{{ formatNum(char.gold) }}</span>
        </div>
      </div>

      <!-- Stats Section -->
      <div data-name="statsPanel" :style="statsPanelStyle" data-anchor="0.5,0">
        <span data-name="statsTitle" style="height:26px">Combat Stats</span>

        <!-- HP / MP -->
        <div v-for="bar in mainBars" :key="bar.name"
             :data-name="bar.name + 'Row'" :style="statRowStyle">
          <span :data-name="bar.name + 'Label'" style="width:36px;height:20px">{{ bar.label }}</span>
          <ProgressBar :name="bar.name + 'Bar'" :value="bar.value" :max="bar.max"
                       :color="bar.color" width="200px" height="18px" />
          <span :data-name="bar.name + 'Val'" style="width:100px;height:18px">
            {{ formatNum(bar.value) }} / {{ formatNum(bar.max) }}
          </span>
        </div>

        <!-- Attribute Grid: 2 columns -->
        <div data-name="statGrid" style="display:flex;flex-direction:column;gap:6px">
          <div v-for="(row, ri) in attrRows" :key="ri"
               :data-name="'statRow' + (ri+1)" style="display:flex;flex-direction:row;gap:20px">
            <div v-for="attr in row" :key="attr.name"
                 :data-name="attr.name + 'Stat'"
                 style="display:flex;flex-direction:row;gap:4px;align-items:center">
              <span :data-name="attr.name + 'Label'" style="width:36px;height:20px">{{ attr.label }}</span>
              <ProgressBar :name="attr.name + 'Bar'" :value="attr.value" :max="100"
                           :color="attr.color" width="120px" height="14px" />
              <span :data-name="attr.name + 'Val'" style="width:32px;height:18px">{{ attr.value }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Inventory Grid (scrollable) -->
      <div data-name="inventoryPanel" style="flex-grow:1;display:flex;flex-direction:column;gap:6px">
        <span data-name="invTitle" style="height:26px">Inventory ({{ items.filter(i => i).length }}/{{ totalSlots }})</span>
        <div data-name="invScroll" style="flex-grow:1;overflow:scroll">
          <div data-name="invGrid" style="display:flex;flex-direction:column;gap:6px;padding:4px">
            <div v-for="(row, ri) in inventoryRows" :key="ri"
                 :data-name="'invRow' + (ri+1)" style="display:flex;flex-direction:row;gap:6px">
              <div v-for="(item, ci) in row" :key="ci"
                   :data-name="'slot_' + (ri * cols + ci)"
                   style="width:68px;height:68px">
                <img v-if="item" :data-name="'item_' + (ri * cols + ci)"
                     :src="'/ui-pack-rpg/PNG/' + item.icon"
                     style="width:56px;height:56px"/>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Action Bar -->
    <div data-name="bottomBar" :style="bottomBarStyle">
      <button v-for="btn in actionButtons" :key="btn.id"
              :data-name="'btn' + btn.id"
              :src="'/ui-pack-rpg/PNG/' + btn.bg"
              :data-pressed="'/ui-pack-rpg/PNG/' + btn.pressed"
              style="width:140px;height:48px">
        {{ btn.label }}
      </button>
    </div>

  </div>
</template>

<script setup lang="ts">
import ProgressBar from '../components/ProgressBar.vue'

const char = {
  name: 'DarkKnight_42',
  class: 'Shadow Warrior',
  level: 58,
  exp: 7250,
  expMax: 10000,
  gold: 125430,
}

const tabs = [
  { id: 'Char', label: 'Character' },
  { id: 'Inv', label: 'Inventory' },
  { id: 'Skill', label: 'Skills' },
  { id: 'Quest', label: 'Quests' },
]

const mainBars = [
  { name: 'hp', label: 'HP', value: 4250, max: 5000, color: 'green' as const },
  { name: 'mp', label: 'MP', value: 1260, max: 2000, color: 'blue' as const },
]

const attrs = [
  { name: 'str', label: 'STR', value: 78, color: 'red' as const },
  { name: 'dex', label: 'DEX', value: 65, color: 'green' as const },
  { name: 'int', label: 'INT', value: 42, color: 'blue' as const },
  { name: 'vit', label: 'VIT', value: 91, color: 'yellow' as const },
]

const attrRows = [attrs.slice(0, 2), attrs.slice(2, 4)]

const cols = 8
const totalSlots = 24

const itemList: (null | { icon: string })[] = [
  { icon: 'swordBronze.png' },
  { icon: 'shieldBronze.png' },
  { icon: 'swordGold.png' },
  { icon: 'gemRed.png' },
  { icon: 'gemGreen.png' },
  { icon: 'gemBlue.png' },
  null, null,
  { icon: 'swordSilver.png' },
  { icon: 'shieldGold.png' },
  null, null, null, null, null, null,
  null, null, null, null, null, null, null, null,
]

const items = itemList
const inventoryRows: (typeof itemList)[] = []
for (let i = 0; i < itemList.length; i += cols) {
  inventoryRows.push(itemList.slice(i, i + cols))
}

const actionButtons = [
  { id: 'Equip', label: 'Equip', bg: 'buttonLong_blue.png', pressed: 'buttonLong_blue_pressed.png' },
  { id: 'Use', label: 'Use', bg: 'buttonLong_grey.png', pressed: 'buttonLong_grey_pressed.png' },
  { id: 'Drop', label: 'Drop', bg: 'buttonLong_brown.png', pressed: 'buttonLong_brown_pressed.png' },
]

function formatNum(n: number): string {
  return n.toLocaleString()
}

// Styles
const rootStyle = {
  width: '640px',
  height: '960px',
  display: 'flex',
  flexDirection: 'column' as const,
}

const topNavStyle = {
  width: '640px',
  height: '60px',
  display: 'flex',
  flexDirection: 'row' as const,
  alignItems: 'center',
  gap: '4px',
  padding: '0 10px',
}

const tabBtnStyle = {
  flexGrow: '1',
  height: '44px',
}

const mainContentStyle = {
  flexGrow: '1',
  display: 'flex',
  flexDirection: 'column' as const,
  padding: '10px',
  gap: '12px',
}

const charHeaderStyle = {
  width: '620px',
  height: '120px',
  display: 'flex',
  flexDirection: 'row' as const,
  gap: '16px',
  alignItems: 'center',
}

const charInfoStyle = {
  flexGrow: '1',
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '6px',
}

const statsPanelStyle = {
  width: '620px',
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '8px',
  padding: '8px',
}

const statRowStyle = {
  display: 'flex',
  flexDirection: 'row' as const,
  alignItems: 'center',
  gap: '8px',
  height: '24px',
}

const bottomBarStyle = {
  width: '640px',
  height: '64px',
  display: 'flex',
  flexDirection: 'row' as const,
  alignItems: 'center',
  justifyContent: 'center',
  gap: '16px',
  padding: '0 20px',
}
</script>
