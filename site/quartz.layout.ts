import { siteLayout } from "./mathstruct/layout"

// For a new game: point GitHub at the game's repository. The giscus comments
// block in mage-game is deliberately omitted here; add it per game once its
// GitHub Discussions category exists.
export const { sharedPageComponents, defaultContentPageLayout, defaultListPageLayout } =
  siteLayout({
    footerLinks: {
      GitHub: "https://github.com/MagierSpieleSchmiede/game-template",
    },
    // search only this vault, not other sites
    searchSites: [],
  })
