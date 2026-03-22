# Cocos Vue Component Props Reference

Quick reference for all component props. See the .vue source files in `frontend/src/components/` for implementation details.

## CocosButton
`name`(required), `normal?`, `pressed?`, `disabled?`, `width?`(120px), `height?`(40px), `fontSize?`(14px). Slot = button text.

## ProgressBar
`name`(required), `value`(required), `max?`(100), `color?`('green'|'red'|'blue'|'yellow'), `width?`(200px), `height?`(24px).

## CocosSlider
`name`(required), `percent?`(50), `bar?`, `ball?`, `progress?`, `width?`(200px), `height?`(30px).

## CocosCheckBox
`name`(required), `checked?`(false), `bgNormal?`, `bgPressed?`, `bgDisabled?`, `crossNormal?`, `crossDisabled?`, `width?`(40px), `height?`(40px).

## CocosTextField
`name`(required), `text?`(''), `placeholder?`(''), `maxLength?`, `password?`(false), `width?`(200px), `height?`(36px), `fontSize?`(16px).

## CocosScrollView
`name`(required), `direction?`('vertical'|'horizontal'|'both'), `bounce?`(true), `innerWidth?`, `innerHeight?`, `width?`(300px), `height?`(400px). Slot = scroll content.

## CocosListView
`name`(required), `direction?`('vertical'|'horizontal'), `itemMargin?`(0), `gravity?`, `width?`(300px), `height?`(400px). Slot = child items (use CocosProjectNode for dynamic items).

## CocosPageView
`name`(required), `direction?`('horizontal'|'vertical'), `width?`(300px), `height?`(400px). Slot = pages.

## CocosSprite
`name`(required), `texture`(required), `width?`(64px), `height?`(64px).

## CocosTextBMFont
`name`(required), `text`(required), `fntFile`(required), `width?`(auto), `height?`(auto), `fontSize?`(24px).

## CocosTextAtlas
`name`(required), `text`(required), `atlasFile`(required), `charWidth`(required), `charHeight`(required), `startChar?`('.'), `width?`(auto), `height?`(auto).

## CocosProjectNode
`name`(required), `file`(required), `width?`(100px), `height?`(100px).
