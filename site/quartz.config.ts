import { siteConfig } from "./mathstruct/config"

// Theme, plugins and fixes come from the preset (https://github.com/MathStruct/quartz,
// mathstruct/config.ts); this file only says what is particular to this repository.
// For a new game: set pageTitle and baseUrl to the game's repository.
export default siteConfig({
  pageTitle: "Game Template",
  baseUrl: "magierspieleschmiede.github.io/game-template",
  locale: "en-US",
})
