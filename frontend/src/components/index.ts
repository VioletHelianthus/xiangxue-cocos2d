import type { App } from 'vue'

// Engine-agnostic preview component
import ProgressBar from './ProgressBar.vue'

// Cocos-specific components
import CocosButton from './CocosButton.vue'
import CocosCheckBox from './CocosCheckBox.vue'
import CocosSlider from './CocosSlider.vue'
import CocosTextField from './CocosTextField.vue'
import CocosScrollView from './CocosScrollView.vue'
import CocosListView from './CocosListView.vue'
import CocosPageView from './CocosPageView.vue'
import CocosSprite from './CocosSprite.vue'
import CocosTextBMFont from './CocosTextBMFont.vue'
import CocosTextAtlas from './CocosTextAtlas.vue'
import CocosProjectNode from './CocosProjectNode.vue'

const components: Record<string, any> = {
  ProgressBar,
  CocosButton,
  CocosCheckBox,
  CocosSlider,
  CocosTextField,
  CocosScrollView,
  CocosListView,
  CocosPageView,
  CocosSprite,
  CocosTextBMFont,
  CocosTextAtlas,
  CocosProjectNode,
}

export function registerComponents(app: App) {
  for (const [name, component] of Object.entries(components)) {
    app.component(name, component)
  }
}

export {
  ProgressBar,
  CocosButton,
  CocosCheckBox,
  CocosSlider,
  CocosTextField,
  CocosScrollView,
  CocosListView,
  CocosPageView,
  CocosSprite,
  CocosTextBMFont,
  CocosTextAtlas,
  CocosProjectNode,
}
