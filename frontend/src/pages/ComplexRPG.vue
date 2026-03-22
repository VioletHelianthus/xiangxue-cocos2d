<template>
  <!-- Root: fills the device frame, flex column layout -->
  <div data-name="root" style="width:100%;height:100%;display:flex;flex-direction:column">

    <!-- ═══ Top Header Bar ═══ -->
    <div data-name="headerBar" style="width:100%;height:80px;display:flex;flex-direction:row;align-items:center;padding-left:12px;padding-right:12px;gap:8px">
      <!-- Player avatar -->
      <img data-name="avatar" src="/ui-pack-rpg/PNG/cursorGauntlet_bronze.png"
           style="width:56px;height:56px" />
      <!-- Name + Level column -->
      <div data-name="nameCol" style="display:flex;flex-direction:column;gap:2px">
        <span data-name="playerName" style="height:24px;color:#ffe082">DarkKnight_42</span>
        <span data-name="playerLevel" style="height:18px;color:#aaa">Lv.58 Shadow Warrior</span>
      </div>
      <!-- Spacer -->
      <div style="flex-grow:1"></div>
      <!-- Gold display -->
      <img data-name="goldIcon" src="/ui-pack-rpg/PNG/iconCircle_brown.png"
           style="width:24px;height:24px" />
      <span data-name="goldText" style="height:22px;color:#ffd700">125,430</span>
      <!-- Settings button -->
      <CocosButton name="settingsBtn"
                   normal="/ui-pack-rpg/PNG/buttonSquare_brown.png"
                   pressed="/ui-pack-rpg/PNG/buttonSquare_brown_pressed.png"
                   width="44px" height="44px">⚙</CocosButton>
    </div>

    <!-- ═══ Status Panel ═══ -->
    <div data-name="statusPanel" style="width:100%;display:flex;flex-direction:column;gap:10px;padding-left:12px;padding-right:12px;padding-top:10px;padding-bottom:10px">
      <!-- HP Row -->
      <div data-name="hpRow" style="display:flex;flex-direction:row;align-items:center;gap:8px;height:28px">
        <span data-name="hpLabel" style="width:36px;height:20px;color:#8f8">HP</span>
        <ProgressBar name="hpBar" :value="4250" :max="5000" color="green" width="auto" height="22px" style="flex:1" />
        <span data-name="hpText" style="width:110px;height:18px;color:#ccc">4,250 / 5,000</span>
      </div>
      <!-- MP Row -->
      <div data-name="mpRow" style="display:flex;flex-direction:row;align-items:center;gap:8px;height:28px">
        <span data-name="mpLabel" style="width:36px;height:20px;color:#88f">MP</span>
        <ProgressBar name="mpBar" :value="1260" :max="2000" color="blue" width="auto" height="22px" style="flex:1" />
        <span data-name="mpText" style="width:110px;height:18px;color:#ccc">1,260 / 2,000</span>
      </div>
      <!-- EXP Row -->
      <div data-name="expRow" style="display:flex;flex-direction:row;align-items:center;gap:8px;height:28px">
        <span data-name="expLabel" style="width:36px;height:20px;color:#ff8">EXP</span>
        <ProgressBar name="expBar" :value="7250" :max="10000" color="yellow" width="auto" height="22px" style="flex:1" />
        <span data-name="expText" style="width:110px;height:18px;color:#ccc">7,250 / 10,000</span>
      </div>
      <!-- Stats grid: STR / DEX / INT / VIT -->
      <div data-name="statsRow" style="display:flex;flex-direction:row;gap:16px;height:22px;padding-left:4px;flex-wrap:wrap">
        <span data-name="strStat" style="height:20px;color:#f88">STR 78</span>
        <span data-name="dexStat" style="height:20px;color:#8f8">DEX 65</span>
        <span data-name="intStat" style="height:20px;color:#88f">INT 42</span>
        <span data-name="vitStat" style="height:20px;color:#ff8">VIT 91</span>
      </div>
    </div>

    <!-- ═══ Equipment Slots ═══ -->
    <div data-name="equipPanel" style="width:100%;display:flex;flex-direction:column;gap:8px;padding-left:12px;padding-right:12px;padding-top:8px">
      <span data-name="equipTitle" style="height:22px;color:#ddd">Equipment</span>
      <div data-name="equipGrid" style="display:flex;flex-direction:row;gap:12px;justify-content:center;flex-wrap:wrap">
        <div data-name="slotWeapon" style="width:80px;height:80px">
          <img data-name="weaponIcon" src="/ui-pack-rpg/PNG/cursorSword_gold.png" style="width:100%;height:100%" />
        </div>
        <div data-name="slotArmor" style="width:80px;height:80px">
          <img data-name="armorIcon" src="/ui-pack-rpg/PNG/iconCircle_blue.png" style="width:100%;height:100%" />
        </div>
        <div data-name="slotHelmet" style="width:80px;height:80px">
          <img data-name="helmetIcon" src="/ui-pack-rpg/PNG/iconCircle_grey.png" style="width:100%;height:100%" />
        </div>
        <div data-name="slotBoots" style="width:80px;height:80px">
          <img data-name="bootsIcon" src="/ui-pack-rpg/PNG/iconCircle_beige.png" style="width:100%;height:100%" />
        </div>
        <div data-name="slotRing" style="width:80px;height:80px">
          <img data-name="ringIcon" src="/ui-pack-rpg/PNG/iconCircle_brown.png" style="width:100%;height:100%" />
        </div>
      </div>
    </div>

    <!-- ═══ Skill List (fills remaining space) ═══ -->
    <div data-name="skillSection" style="width:100%;flex:1;display:flex;flex-direction:column;gap:6px;padding-left:12px;padding-right:12px;padding-top:8px;min-height:0">
      <span data-name="skillTitle" style="height:22px;color:#ddd">Skills</span>
      <CocosListView name="skillList" direction="vertical" :item-margin="8" width="100%" height="auto" style="flex:1;min-height:0">
        <!-- Skill 1: Fire Slash -->
        <div data-name="skill1" style="width:100%;height:64px;display:flex;flex-direction:row;align-items:center;gap:10px;padding-left:6px;padding-right:6px">
          <img data-name="skill1Icon" src="/ui-pack-rpg/PNG/iconCircle_brown.png" style="width:48px;height:48px" />
          <div data-name="skill1Info" style="display:flex;flex-direction:column;gap:2px;flex:1;min-width:0">
            <span data-name="skill1Name" style="height:20px;color:#f90">Fire Slash</span>
            <span data-name="skill1Desc" style="height:16px;color:#999">Deal 250% ATK fire damage</span>
          </div>
          <span data-name="skill1Cost" style="width:60px;height:18px;color:#88f">MP 45</span>
        </div>
        <!-- Skill 2: Shadow Step -->
        <div data-name="skill2" style="width:100%;height:64px;display:flex;flex-direction:row;align-items:center;gap:10px;padding-left:6px;padding-right:6px">
          <img data-name="skill2Icon" src="/ui-pack-rpg/PNG/iconCircle_grey.png" style="width:48px;height:48px" />
          <div data-name="skill2Info" style="display:flex;flex-direction:column;gap:2px;flex:1;min-width:0">
            <span data-name="skill2Name" style="height:20px;color:#a8f">Shadow Step</span>
            <span data-name="skill2Desc" style="height:16px;color:#999">Teleport behind enemy, +30% crit</span>
          </div>
          <span data-name="skill2Cost" style="width:60px;height:18px;color:#88f">MP 60</span>
        </div>
        <!-- Skill 3: Iron Will -->
        <div data-name="skill3" style="width:100%;height:64px;display:flex;flex-direction:row;align-items:center;gap:10px;padding-left:6px;padding-right:6px">
          <img data-name="skill3Icon" src="/ui-pack-rpg/PNG/iconCircle_beige.png" style="width:48px;height:48px" />
          <div data-name="skill3Info" style="display:flex;flex-direction:column;gap:2px;flex:1;min-width:0">
            <span data-name="skill3Name" style="height:20px;color:#8df">Iron Will</span>
            <span data-name="skill3Desc" style="height:16px;color:#999">DEF +50% for 10s</span>
          </div>
          <span data-name="skill3Cost" style="width:60px;height:18px;color:#88f">MP 30</span>
        </div>
      </CocosListView>
    </div>

    <!-- ═══ Bottom Action Bar ═══ -->
    <div data-name="actionBar" style="width:100%;height:80px;display:flex;flex-direction:row;align-items:center;justify-content:center;gap:16px;padding-bottom:12px;flex-shrink:0">
      <CocosButton name="attackBtn"
                   normal="/ui-pack-rpg/PNG/buttonLong_brown.png"
                   pressed="/ui-pack-rpg/PNG/buttonLong_brown_pressed.png"
                   width="160px" height="52px" font-size="16px">Attack</CocosButton>
      <CocosButton name="defendBtn"
                   normal="/ui-pack-rpg/PNG/buttonLong_blue.png"
                   pressed="/ui-pack-rpg/PNG/buttonLong_blue_pressed.png"
                   width="160px" height="52px" font-size="16px">Defend</CocosButton>
      <CocosButton name="itemBtn"
                   normal="/ui-pack-rpg/PNG/buttonLong_beige.png"
                   pressed="/ui-pack-rpg/PNG/buttonLong_beige_pressed.png"
                   width="160px" height="52px" font-size="16px">Items</CocosButton>
    </div>
  </div>
</template>

<script setup lang="ts">
// All components are globally registered via registerComponents()
</script>
