Here's the edited document with clearer, more concise language:

```
window.helpx = window.helpx || {};
window.helpx.analytics = {"deepLinks": {}, "installLinks": {}, "dunamis": {}};

window.helpx.analytics.deepLinks.regexPatterns = 'https://creativecloud.adobe.com/campaign/*,https://[a-z0-9]*.app.link/*,https://www.adobe.com/([a-z_]+/)?(express|go)/.*,https://(new.)?express.adobe.com/*,https://acrobat.adobe.com/*,https://photoshop.adobe.com/*,https://(preview.)?illustrator.adobe.com/*,https://firefly.adobe.com/*,https://fonts.adobe.com/*,https://stock.adobe.com/*,https://creative.adobe.com/products/download/*,https://creativecloud.adobe.com/apps/(download|updates)/.*,https://creativecloud.adobe.com/apps/all/[a-z0-9_-]*/installation,https://www.adobe.com/([a-z_]+/)?apps/updates/.*,acrobat([0-9]{4})://dc/launchTool.*,https://(adminconsole|global-admin-console).adobe.com.*,https://www.adobe.com/campaign/*';
window.helpx.analytics.deepLinks.label = 'deep link click';

window.helpx.analytics.installLinks.regexPatterns = 'https://creativecloud.adobe.com/apps/all/*';
window.helpx.analytics.installLinks.label = 'install link click';

window.helpx.analytics.dunamis.stats = {"loadStart": Date.now()};
window.helpx.analytics.dunamis.isEnabled = true;
window.helpx.analytics.dunamis.projectKey = 'helpx-web-service';
window.helpx.analytics.dunamis.xApiKey = 'helpx-web-service';
window.helpx.analytics.dunamis.env = 'prod';
window.helpx.analytics.dunamis.ingestType = 'dunamis';

window.helpx.wallet = {};
window.helpx.wallet.isEnabled = true;
window.helpx.wallet.env = 'production';
window.helpx.wallet._data = {};
window.helpx.wallet.isReady = () => !!window.helpx.wallet._data.clientSessionId;
window.helpx.wallet.getDetails = () => window.helpx.wallet._data;

// Working with fonts in InDesign
window.dexter = window.dexter || {};
window.dexter.utils = window.dexter.utils || {};

if (!window.IntersectionObserver) {
    document.dispatchEvent(new Event('dexter:headPolyfillLoaded'));
    window.dexter.utils.headPolyfill = true;
}

function setTheme() {
    document.documentElement.setAttribute('theme', 'system');
    
    var themeQuery = window.location.search
        .slice(1)
        .split('&')
        .find(q => q.indexOf('theme=') !== -1);
        
    if (themeQuery && themeQuery.split('=').length > 1) {
        var theme = themeQuery.split('=')[1];
        if (['light', 'dark'].includes(theme)) {
            document.documentElement.setAttribute('theme', theme);
        }
    }
}
setTheme();

window.showHelpxCommerceModal = true;

var gnavExp = 'acom/cc-mega-menu/indesign-localnav';
var disableSearchTemplates = ['helpx/components/structure/helpxMain','helpx/components/structure/helpxMain-searchResults'];

if(URLSearchParams){
    var searchParams = new URLSearchParams(window.location.href);
    gnavExp = searchParams.get('gnavExp') || gnavExp;
}

var disableGnavTarget = true;

window.fedsConfig = {
    locale: 'pl',
    disableSticky: true,
    disableTarget: disableGnavTarget,
    content: { experience: gnavExp },
    subnav: {"theme":{"base":"light","gradient":{"toColor":"#FAFAFA","fromColor":"#FAFAFA","opacity":1.0}}},
    footer: { regionModal: function () { window.location.hash = 'languageNavigation'; } },
    breadcrumbs: { showLogo: true, links: [] },
    privacy: {
        otDomainId: '7a5eb705-95ed-4cc4-a11d-0cc5760e93db',
        footerLinkSelector: '[data-feds-action="open-adchoices-modal"]'
    },
    search: { context: '' },
    oneTapLogin: false,
    oneTapRedirectURL: '',
    universalNav: true,
    universalNavComponents: 'profile,notifications,appswitcher',
    disableSearch: disableSearchTemplates.indexOf('helpx/components/structure/helpxMain-article') !== -1
};

(function() {
    const gnavExperience = document.querySelector('div[data-param-key]');
    if (gnavExperience === null) return;
    if (window.location.search === '') return;
    
    const key = gnavExperience.getAttribute('data-param-key');
    const val = gnavExperience.getAttribute('data-param-val');
    const exp = gnavExperience.getAttribute('data-experience');
    
    const queryParams = window.location.search.substring(1);
    const keyValPairs = queryParams.split('&');
    const containsParams = keyValPairs.filter(pair => {
        const splitPair = pair.split('=');
        return splitPair[0] === key && splitPair[1] === val;
    });
    
    if (!containsParams.length) return;
    
    if (window.fedsConfig && window.fedsConfig.content && window.fedsConfig.content.experience) {
        window.fedsConfig.content.experience = exp;
    }
})();

window.dexter.jarvis = {
    isDesktop: (window.dexter.personalization && 
               window.dexter.personalization.technology && 
               window.dexter.personalization.technology.platform && 
               window.dexter.personalization.technology.platform.type) ? 
               window.dexter.personalization.technology.platform.type === 'desktop' : false
};

window.dexter.jarvis.desktopEnabled = window.dexter.jarvis.isDesktop && true;
window.dexter.jarvis.mobileEnabled = !window.dexter.jarvis.isDesktop && true;
window.dexter.jarvis.surfaceName = 'helpx-default';
window.dexter.jarvis.surfaceVersion = '1.0';

window.dexter.jarvis.onReady = function (newChatEnabled, jarvisData) {
    if (newChatEnabled) {
        if (typeof enableLE === 'function') enableLE();
    } else {
        if (typeof enableLP === 'function') enableLP();
    }
};

window.dexter.jarvis.onError = function () {
    if (typeof enableLP === 'function') enableLP();
};

window.dexter.jarvis.openExistingChat = function () {
    if (typeof enableLP === 'function') enableLP();
};

window.dexter.jarvis.getContext = (window.dexter && window.dexter.callbacks) ? 
                                  window.dexter.callbacks.getContext : null;

window.fedsConfig = window.fedsConfig || {};

if (window.dexter.jarvis.desktopEnabled || window.dexter.jarvis.mobileEnabled) {
    window.fedsConfig.jarvis = {
        surfaceName: 'helpx-default',
        surfaceVersion: '1.0',
        onReady: function (newChatEnabled, jarvisData) {
            if (typeof enableLE === 'function') enableLE();
        },
        onError: function () {
            if (typeof enableLP === 'function') enableLP();
        },
        openExistingChat: function () {
            if (typeof enableLP === 'function') enableLP();
        },
        getContext: (window.dexter && window.dexter.callbacks) ? 
                    window.dexter.callbacks.getContext : null,
        directConfig: { lazyLoad: true }
    }
}

.globalNavHeader { height: 64px; }
@media screen and (min-width: 600px) { .globalNavHeader { height: 64px; } }
@media screen and (min-width: 1200px) { .globalNavHeader { height: 64px; } }

(function () {
    function f() {
        var scriptEl = document.getElementById("feds-style-page-load");
        if (scriptEl) scriptEl.remove();
    }
    
    if (feds && feds.events && feds.events.experience) {
        f();
    } else {
        window.addEventListener("feds.events.experience.loaded", f, { once: true });
    }
})();

window.helpx.search = window.helpx.search || {};
window.helpx.search.enableAsdeSearch = true;

if (window.helpx.search.enableAsdeSearch) {
    window.feds?.utilities?.getUserApplications();
}

window.helpx.sophiaConfig = {};
window.helpx.sophiaConfig.stageUrl = 'https://p13n-stage.adobe.io/psdk/v2/content';
window.helpx.sophiaConfig.prodUrl = 'https://p13n.adobe.io/psdk/v2/content';
window.helpx.sophiaConfig.surfaceID = 'HelpX_Personalization';
window.helpx.sophiaConfig.apiKey = 'AdobeSupport1';
window.helpx.sophiaConfig.clientCode = 'helpx.adobe.com';

window.helpx.ajoConfig = {};
let ajoConfigsurfaceURI = 'web://helpx.adobe.com/#greeting-message-container,web://helpx.adobe.com/#hva,web://helpx.adobe.com/#plan-account,web://helpx.adobe.com/#content-assets-ql,web://helpx.adobe.com/#content-related-article,web://helpx.adobe.com/#content-ads,web://helpx.adobe.com/#content-recommendations';
window.helpx.ajoConfig.surfaceURI = ajoConfigsurfaceURI ? ajoConfigsurfaceURI.split(',') : [];

window.dexter.Analytics = window.dexter.Analytics || {};
window.dexter.Analytics.language = 'pl_PL';
window.dexter.Analytics.geoRegion = 'PL';
window.dexter.Analytics.targetEnabled = true;

window.dexter.Analytics.launchLoaded = true;
window.dexter.Analytics.audienceManagerEnabled = true;
window.dexter.Analytics.legacyAnalytics = false;

window.alloy_load = window.alloy_load || {};
window.alloy_load.data = window.alloy_load.data || {};

window.alloy_all = window.alloy_all || {};
window.alloy_all.data = window.alloy_all.data || {};
window.alloy_all.data._adobe_corpnew = window.alloy_all.data._adobe_corpnew || {};
window.alloy_all.data._adobe_corpnew.digitalData = window.alloy_all.data._adobe_corpnew.digitalData || {};
window.alloy_all.data._adobe_corpnew.digitalData.page = window.alloy_all.data._adobe_corpnew.digitalData.page || {};
window.alloy_all.data._adobe_corpnew.digitalData.page.pageInfo = window.alloy_all.data._adobe_corpnew.digitalData.page.pageInfo || {};
window.alloy_all.data._adobe_corpnew.digitalData.page.pageInfo.language = window.dexter.Analytics.language;

launchURL = "https://assets.adobedtm.com/d4d114c60e50/a0e989131fd5/launch-5dd5dd2177e6.min.js";
edgeConfigId = "913eac4d-900b-45e8-9ee7-306216765cd2";

window.marketingtech = {
    adobe: {
        launch: { url: launchURL, controlPageLoad: true },
        alloy: { edgeConfigId: edgeConfigId },
        target: window.dexter.Analytics.targetEnabled,
        audienceManager: window.dexter.Analytics.audienceManagerEnabled
    },
    sophia: true
}

window.dexter.Analytics.sophiaEnabled = true;
window.dexter.Analytics.ajoEnabled = true;

window.helpx.feds = window.helpx.feds || {};
window.helpx.feds.subscriptions = {
    'BILLING': true,
    'OFFER.MERCHANDISING': true,
    'OFFER.PRODUCT_ARRANGEMENT_V2': true
}

window.helpx.private = window.helpx.private || {};
window.helpx.private = {
    beta: false,
    featurePack: false,
    admittedDomains: ""
}

[Adobe InDesign](https://www.adobe.com/pl/products/indesign.html)
*   [Features](# "Features")
    *   [What's new](/pl/indesign/using/whats-new.html "What's new")
    *   [Flyer design](https://www.adobe.com/pl/products/indesign/flyer-design-software.html "Flyer design")
    *   [Poster design](https://www.adobe.com/pl/products/indesign/poster-design-software.html "Poster design")
    *   [Postcard design](https://www.adobe.com/pl/products/indesign/postcard-design-software.html "Postcard design")
    *   [Ebook design](https://www.adobe.com/pl/products/indesign/ebook-creator-software.html "Ebook design")
    *   [Page layouts](https://www.adobe.com/pl/products/indesign/page-layouts.html "Page layouts")
    *   [Brochure design](https://www.adobe.com/pl/products/indesign/brochure-design-software.html "Brochure design")
    *   [Resume design](https://www.adobe.com/pl/products/indesign/resume-design-software.html "Resume design")
    *   [Presentation maker](https://www.adobe.com/pl/products/indesign/presentation-maker.html "Presentation maker")
    *   [Menu design](https://www.adobe.com/pl/products/indesign/menu-design-software.html "Menu design")

*   [Learning resources and support](/pl/indesign.html?promoid=ZXL8F59B&mv=other "Learning resources and support")
*   [System requirements](/pl/indesign/system-requirements.html "System requirements")
*   [Free trial](https://www.adobe.com/pl/products/indesign.html#mini-plans-web-cta-indesign-card "Free trial")

[Buy now](https://www.adobe.com/pl/creativecloud/plans.html?filter=design&plan=individual&promoid=TKZTLDFL&mv=other "Buy now")

User Guide
```# Using fonts in InDesign

_Last updated December 11, 2024_

A _font_ is a complete set of characters—letters, numbers, and symbols—that share the same weight, width, and style. Example: "10-point Adobe Garamond Bold font."

A _font family_ (also called a _typeface_) is a collection of fonts with similar design features intended for use together. Example: Adobe Garamond.

A _font style_ is a variant within a font family. The basic font is typically the _Regular_ or _Roman_ style (names vary between font families). Families may also include styles like Regular, Bold, Semibold, Italic, and Bold Italic.

## Font types

Examples of fonts appear in the font family and style menus in the Typography panel and other areas of the application where you select fonts. Different font types are also marked with special icons:

*   OpenType
*   SVG OpenType
*   Variable fonts
*   TrueType
*   Adobe Fonts
*   Multiple Master
*   Composite

In the Text preferences window, you can disable font preview and change font names' point size or sample text used.

To view the list of available fonts in InDesign, do one of the following:

*   Open the Typography panel (Ctrl+T) > Font Family dropdown menu
*   Open the Control panel > Font Family dropdown menu
*   Open the Properties panel > Font Family dropdown menu

## Working with missing fonts

**Important reminder:**

In January 2023, Adobe discontinued support for Type 1 fonts. More information is available in the help article [PostScript Type 1 fonts end of support](/pl/fonts/kb/postscript-type-1-fonts-end-of-support.html).

## Installing fonts

For information about installing and activating fonts for use across all applications, refer to your system or font management software documentation.

You can make fonts available in InDesign by copying their files to the Fonts folder within the InDesign application folder on your hard drive. However, fonts from this folder are only available within InDesign.

When two or more active fonts in InDesign use the same family name but have different Adobe PostScript names, they'll still be available in InDesign. Duplicate fonts appear in menus with their technology abbreviation in parentheses. For example: Helvetica TrueType appears as "Helvetica (TT)", Helvetica PostScript Type 1 as "Helvetica (T1)", and Helvetica OpenType as "Helvetica (OTF)". If two fonts share the same PostScript name but one includes the string .dfont in its name, the other font will be used.

## Auto-activate missing fonts

When documents contain missing fonts, InDesign automatically activates matching Adobe Fonts in the background without displaying the **Missing Fonts** dialog box. Missing fonts get replaced with equivalent Adobe Fonts fonts.

**Auto-activate Adobe Fonts** is disabled by default in InDesign. To enable it, select **Auto-activate Adobe Fonts** in **Edit > Preferences > File Handling**.

### When auto-activation is enabled

If some fonts are missing from your document, InDesign automatically activates them from Adobe Fonts.

The activation process happens in the background:

*   If all missing fonts are available through Adobe Fonts, they'll be automatically activated. You can continue working on your documents.
*   If only some missing fonts are available through Adobe Fonts, those fonts will be activated in the background. The Missing Fonts dialog box will display a list of remaining missing fonts.
    *   Click Replace Fonts to manually download missing fonts from other sources, or
    *   Click Skip to close the dialog box. Missing fonts will be replaced with default fonts.
*   If none of the missing fonts are available through Adobe Fonts, the Missing Fonts dialog box appears with a list of missing fonts.

You can also check the progress of missing font activation in the Background Tasks panel. Access Background Tasks in two ways:

*   Choose Window > Utilities > Background Tasks
*   Click the blue circle icon in the upper-right corner of the InDesign application bar

### When auto-activation is disabled

If Auto-activate Adobe Fonts isn't enabled in Preferences and your document contains missing fonts, the Missing Fonts dialog box appears. Click Activate to manually activate missing fonts through Adobe Fonts.

## OpenType fonts

Each OpenType font is defined in a file compatible with both Windows® and Macintosh® systems, allowing these font files to be transferred between different operating systems without worrying about unwanted font substitutions. They can contain multiple elements such as calligraphic characters and special ligatures not available in current PostScript and TrueType fonts.

  Note:

OpenType fonts display an icon.

When using OpenType fonts, you can automatically substitute alternative glyphs in text, such as ligatures, small caps, fractions, and old-style figures.

**A.** Ordinal numbers **B.** Decorative ligatures **C.** Calligraphic characters

OpenType fonts may include extended character sets and support certain layout features—functions that provide greater linguistic and typographic control over text. Adobe OpenType fonts supporting Central European (CE) languages display "Pro" appended to their menu name. OpenType fonts without European language support are labeled "Standard" and have the "Std" suffix. All OpenType fonts can be installed and used alongside TrueType fonts.

For more information about OpenType fonts, visit [www.adobe.com/go/opentype_pl](https://www.adobe.com/go/opentype_pl).

## Applying OpenType font attributes

### Using Typography or Control panels

The Typography and Control panels support applying OpenType font attributes like fractions or calligraphic characters.

For more information about OpenType fonts, visit [www.adobe.com/go/opentype_pl](https://www.adobe.com/go/opentype_pl).

1.  Ensure an OpenType font is selected in the Typography or Control panel.
2.  From the Typography panel menu, choose OpenType, then select an OpenType attribute such as Decorative Ligatures or Fractions.

Unsupported features appear in brackets, for example: \[Calligraphic\].

  Note:

You can also select OpenType font attributes when defining paragraph or character styles using the OpenType Features section in the Style Options dialog box.

### Using context menu

To apply relevant OpenType font attributes to selected text, use the context menu.

1.  Select text or a text frame.
2.  In the context menu, choose an OpenType attribute such as Ordinal Numbers or Fractions. If a tooltip appears after selecting text or a text frame, click it to view the list of OpenType attributes.

  Note:

*   The OpenType attribute selection tooltip doesn't appear in linked text frames.
*   The option to add OpenType font attributes through the context menu isn't available for World-Ready composers.

### OpenType font attributes

When using OpenType fonts in text, you can select OpenType attributes from the Control and Typography panel menus during formatting or style definition.

  Note:

Remember that OpenType fonts vary significantly in available styles and features. If a specific OpenType feature isn't available, its name appears in brackets on the Control panel (for example, \[Calligraphic\]).

**Decorative ligatures**

Font manufacturers may include additional ligatures that shouldn't be used in all circumstances. Selecting this option enables custom ligatures when present. More information about ligatures is available in [Apply ligatures to letter pairs](/pl/indesign/using/formatting-characters.html#apply_ligatures_to_letter_pairs).

**Fractions**

Numbers separated by a slash (such as 1/2) convert to fraction characters when the fractions feature is available.

**Ordinal numbers**

When available, English ordinal numbers like _1st_ and _2nd_ format with superscript letters (1st and 2nd). Spanish letters like _a_ and _o_ in words such as _segunda_ (2a) and _segundo_ (2o) also format appropriately.

**Calligraphic character**

When available, this enables regular and contextual calligraphic characters, including alternate capitals and alternate end-of-word characters.

**Titling characters**

When available, this enables characters designed for all-cap titles. For some fonts, selecting this option for mixed-case text may produce undesirable results.

**Contextual alternatives**

When available, this enables contextual ligatures and alternate hyphens—alternative characters used in specific fonts for better character combinations. For example, the letter combination "bl" in "blask" mimics handwritten characters. This option is selected by default.

**All caps**

If the font contains true small capitals, enabling this option converts characters to small caps. More information is available in [Change the case of text](/pl/indesign/using/formatting-characters.html#change_the_case_of_type).

**Slashed zero**

Selecting this option displays a diagonal slash through the numeral _0_. For some fonts (especially condensed ones), it's difficult to distinguish between the numeral _0_ and the capital letter _O_.

**Stylistic sets**

Some OpenType fonts include alternate glyph sets that create aesthetic effects. A _stylistic set_ is a group of glyph variants you can apply to individual characters or text ranges. Choosing a different stylistic set uses those glyphs instead of the font's default glyphs. If a glyph in a stylistic set is used with another OpenType setting, the individual setting's glyph overrides the character set's glyph. You can preview glyphs from each set using the Glyphs panel.

**Positional forms**

In some languages (like Arabic), a character's appearance depends on its position within a word. Characters can look different at the beginning (initial position), middle (medial position), end (final position), or outside a word (isolated position). Select the character and choose the appropriate positional form option. Selecting General Form inserts a typical character, while selecting Automatic Form adjusts the character shape to its position within the word or text.

**Superscript/subscript and inferior**

Some OpenType fonts contain raised and lowered glyphs scaled appropriately relative to surrounding characters. If an OpenType font doesn't include such glyphs for custom fractions, use the Numerator and Denominator attributes.

**Numerator and denominator**

Some OpenType fonts only convert basic fractions (like 1/2 or 1/4) to special fraction glyphs, not custom fractions (like 4/13 or 99/100). Use the Numerator and Denominator attributes for custom fractions.

**Tabular lining figures**

Full-height figures with equal widths. Useful when numbers must align evenly across rows, like in tables.

**Proportional oldstyle figures**

Variable-height and variable-width figures. Recommended for classic, refined text styles that don't use all caps.

**Proportional lining figures**

Full-height figures with variable widths. Recommended for text using all caps.

**Proportional oldstyle figures**

Variable-height figures with fixed widths. Recommended when classic oldstyle figure appearance is desired but column alignment is needed, such as in annual reports.

**Default figures**

Figure glyphs use the current font's default figure style.

## SVG OpenType fonts

InDesign supports SVG OpenType fonts like color fonts and emoji fonts. SVG OpenType fonts contain multiple colors and gradients within a single glyph.

SVG OpenType fonts offer multiple colors and gradients

Emoji fonts allow various colorful and graphic characters in documents, including emojis, flags, street signs, animals, people, food, and tourist attractions. SVG OpenType emoji fonts like EmojiOne can create composite glyphs from one or multiple glyphs. For example, the application can create country flags or change skin tones for person or body part glyphs—typically yellow, blue, or gray.

To use SVG OpenType fonts:

1.  Create a text object using the Type tool.
2.  Set an SVG OpenType font. These fonts are marked with an icon in the font list.
3.  Select specific glyphs using the Glyphs panel. To open the Glyphs panel, choose Type > Glyphs. You can also open it through Window > Type and Tables > Glyphs.

### Creating composite glyphs

For this example, we'll use the SVG OpenType emoji font EmojiOne. Glyphs can be created by combining characters from the EmojiOne font.

For example, you can create country flags or change skin colors for default single-person or body-part characters (usually yellow, blue, or gray).

  Note:

Glyphs in emoji fonts like EmojiOne differ from keyboard letters. These glyphs are treated as separate characters and are only available in the Glyphs panel (not accessible from the keyboard).

**Creating country flags**

The "letters" (A, B, C, D, etc.) in EmojiOne don't correspond to keyboard keys. When you combine characters in the Glyphs panel to create a country's ISO code, the two characters form that country's flag. For example, US creates the United States flag, GB creates the United Kingdom flag, AR creates Argentina's flag, and IN creates India's flag.

Combining glyphs creates country flags

**Creating character variants**

Default single-person characters (usually yellow, blue, or gray) and body parts can be combined with any available skin tone. The original skin tone in the default character changes to your selected tone. These composites typically don't work with glyphs containing more than one person.

Characters with set skin tones

Combining single-person characters with skin tones

**Notes:**

*   Only one association between emoji characters representing single people or body parts and any skin tone character is possible.
*   Composite glyphs are a font feature. Not all SVG OpenType fonts allow combining characters to create composite glyphs.
*   Some EmojiOne composite characters can be separated into their component parts.

## Variable fonts

InDesign now supports **variable fonts**—a new OpenType font format allowing custom attributes like weight, width, slant, optical size, etc. You can adjust custom attributes using convenient slider options available in the Control panel, Typography panel, Properties panel, Character Styles panel, and Paragraph Styles panel.

Variable fonts display an identifying icon next to their name.

**Accessing variable fonts in the Typography panel**

**Adding variable fonts to character styles**

## Automatically mapping optical size of variable fonts to font size

The optical size of variable fonts can automatically adjust to the font size.

**A.** Adjust optical size settings **B.** Variable font options in Preferences

To select the option **Map optical size of variable fonts to font size**:

*   macOS: InDesign > Preferences > Type
*   Windows: Edit > Preferences > Type

  Note:

When selected, optical size changes with font size. If unselected, changing font size won't affect optical size.

Mapping optical size to automatically set font size

Use sliders to adjust weight, optical size, and other parameters for variable fonts like Minion Variable Concept.

## Live font preview

Select text in your document to view real-time font previews. Hover your cursor over font names in the Control, Typography, or Properties panels to preview selected text.

## Real-time font style preview

To preview font styles in real-time, expand the font family in the font menu and hover over the desired style.

To disable preview options:

*   Choose Edit > Preferences
*   In Type preferences, uncheck Enable font preview in menu

To change the preview text size when viewing real-time previews, use the options Show smaller sample text size, Show default sample text size, and Show larger sample text size.

## Organizing, searching, and filtering fonts

To help find frequently used fonts, you can mark them as favorites or use the recently used fonts section (appearing at the top of the font list). Recently used fonts and favorite fonts are remembered between InDesign sessions.

**A.** Recently used fonts **B.** Star-marked favorite fonts

When searching for fonts, you can limit results by filtering according to classifications like serif, sans-serif, or script fonts. You can also choose whether to search fonts installed on your computer or activated from Adobe Fonts.

Additionally, you can search fonts by visual similarity. Fonts visually similar to your search term appear at the top of search results. Applied filters display in the status bar within the font menus.

## Font search tools

**A.** Show fonts by classification **B.** Show favorite fonts **C.** Show recently added **D.** Show active fonts

**Show fonts by classification**

Filter the font list by classification such as serif, script, or handwritten.

**Show favorite fonts**

Displays only star-marked favorite fonts.

**Show recently added**

Displays fonts recently added to the font list.

**Show active fonts**

Only shows fonts activated from Adobe Fonts in the font list.

## Activating more fonts

Within InDesign, you can browse thousands of fonts from hundreds of providers, instantly activate them, and use them in your document. Activated fonts are available across all Creative Cloud applications.

1.  In the Typography panel, click the Find More tab.
2.  Select the desired font from the list.

      Note:

To preview how the font appears in real-time with selected text, hover your cursor over the font name.

3.  Click the Activate icon next to the font. The Activate icon displays a checkmark symbol when the font is active and available for use.

**A.** Active fonts filter **B.** Activate font icon **C.** Deactivate font icon **D.** Activation in progress

For more information about Adobe Fonts, visit [fonts.adobe.com](https://fonts.adobe.com/).

## Previewing Japanese fonts

On the Find More tab, you can browse all Japanese fonts available at [fonts.adobe.com](https://fonts.adobe.com/) and preview them. To preview Japanese fonts:

1.  Choose Edit > Preferences > Type.
2.  Check Enable Japanese font preview in Find More menu.

Changes take effect after restarting InDesign.

## Applying fonts to text

When specifying a font, you can independently choose the font family and font style. When changing from one font family to another, InDesign attempts to match the current style to an equivalent style in the new family. For example, changing from Arial to Times would replace Arial Bold with Times Bold.

When applying bold or italic styling to text, InDesign uses the variant provided by the font. In most cases, the appropriate bold or italic version applies as expected. However, some fonts may use variants not explicitly described as bold or italic. For example, some font manufacturers specify that applying bold styling uses a semibold variant instead.

1.  Select the object or text to change.
2.  Do any of the following:
    *   Go to the Typography, Control, or Properties panel and select a font family or style from their respective menus. (On Mac OS, the Font menu includes submenus for selecting text styles.)
    *   Click the beginning of a font family or style name (or double-click the first part) in the Typography, Control, or Properties panel and type the first few letters of your desired font. As you type, InDesign displays matching font families or styles.
    *   Choose a font from the Type > Font menu. Note that this menu allows selecting both font families and font styles.

## Setting font size

By default, font size is measured in _points_ (1 point equals 1/72 inch). You can specify any font size from 0.1 to 1296 points in 0.001-point increments.

  Note:

In Fireworks, text size defaults to pixels.

1.  Select characters or text objects to change. If no text is selected, the font applies to all new text.
2.  Do one of the following:
    *   Specify Font Size in the Control panel or Typography panel.
    *   Choose a size from the Type > Size menu. Selecting Other allows entering a new size value in the Typography panel.

      Note:

You can change text measurement units in the Preferences dialog box. This option isn't available in Fireworks.

When selecting text using a missing font, the **Typography** or **Control** panel indicates the missing font by showing it in brackets in the font style menu.

InDesign [automatically activates missing fonts](#auto-activate-missing-fonts) in documents using available Adobe Fonts. If Adobe Fonts doesn't have appropriate fonts, they're replaced with default fonts. In such cases, you can select the text and apply any available font. Missing fonts replaced by substitutes appear at the top of the **Type > Font** menu under the **Missing Fonts** section. By default, text formatted with missing fonts highlights pink.

If a TrueType font is installed and your document contains a Type 1 (T1) version of that font (a different version of the TrueType font), InDesign marks it as missing.

To locate and change missing fonts, choose Type > Find Font. If a missing font is part of a style, you can modify that style definition by updating the selected font.

InDesign's missing fonts dialog box shows whether Adobe Fonts through Creative Cloud is enabled. If disabled, you can enable Adobe Fonts from the missing fonts dialog box.

## Sharing missing fonts

Do any of the following:

*   Activate missing fonts through Adobe Fonts. More information: [Adding Adobe Fonts](/pl/creative-cloud/help/add-fonts.html)
*   Install missing fonts on your system
*   Place missing fonts in the Fonts folder within your InDesign application folder. Fonts from this folder are only available in InDesign. See [Installing fonts](using-fonts.html#installing_fonts)
*   Activate missing fonts through font management applications

## Highlighting substituted fonts in documents

When the **Substituted Fonts** option is checked in preferences, text formatted with missing fonts highlights pink for quick identification.

1.  Choose Edit > Preferences > Composition (Windows®) or InDesign > Preferences > Composition (Mac OS®).
2.  Check Substituted Font and click OK.

## Fonts installed in documents

Fonts in the Document Fonts folder located in the same directory as your InDesign document temporarily install when you open the document. The Package command can generate a Document Fonts folder if you want to share your document or move it to another computer. (Before sharing document fonts, verify that your font software license allows it.) Fonts activated through Adobe Fonts aren't copied by the Package command.

Fonts in the Document Fonts folder differ from fonts available through your operating system's standard font locations. They install when opening the document and override any font with the same PostScript name. However, they only affect fonts within that specific document. Fonts installed for one document aren't available for others. When you close a document, its installed fonts uninstall. Document-installed fonts appear in the Font menu's submenu.

Some Type1 fonts aren't available in documents. Additionally, Mac OS fonts aren't available when InDesign runs on Windows.

* * *

### Related resources

*   [Inserting glyphs and special characters](/pl/indesign/using/glyphs-special-characters.html#insert_glyphs_and_special_characters)
*   [Find and change fonts](/pl/indesign/using/find-change.html#find_and_change_fonts)
*   [Package files](/pl/indesign/using/preflighting-files-handoff.html#package_files)
*   [Font licensing](/pl/fonts/using/font-licensing.html)

### Contact us

We encourage you to share feedback. Post your thoughts on the [Adobe InDesign User Forum](https://community.adobe.com/t5/indesign/ct-p/ct-indesign).## **Pomoc dostępna szybciej i łatwiej**

[Zaloguj się](#)  
Nowy użytkownik? [Utwórz konto ›](#)  

![Avatar]()

[Zarządzaj kontem](#)

Podręczne łącza  
[Wyświetl swoje plany](#) | [Zarządzaj planami](#)  
[Informacje prawne](/pl/legal/legal-notices.html) | [Zasady ochrony prywatności online](https://www.adobe.com/pl/privacy.html)

![Logo Adobe InDesign](/content/dam/help/mnemonics/id_cc_app_RGB.svg)

**Tworzenie materiałów do druku i publikacji cyfrowej w programie InDesign**  
Projektowanie przyciągających uwagę reklam, układów czasopism i książek oraz innych materiałów.

[Otwórz aplikację](https://www.adobe.com/pl/download/indesign?locale=pl)

### Udostępnij tę stronę

*   [Facebook](http://www.facebook.com/sharer.php)
*   [Twitter](https://twitter.com/share?text=twitter)
*   [LinkedIn](http://www.linkedin.com/shareArticle?mini=true)
*   [Copied](#)

## Adobe InDesign

*   [< Odwiedź Centrum pomocy Adobe](/pl/support.html#/all_products)
*   [Materiały do nauki i pomoc techniczna](/pl/pl/support/indesign.html)
*   [Rozpocznij](/pl/pl/indesign/get-started.html)
*   [Podręcznik użytkownika](/pl/pl/indesign/user-guide.html)
*   [Samouczki](/pl/pl/indesign/tutorials.html)

### Zapytaj społeczność

Zadawaj pytania i otrzymuj odpowiedzi od ekspertów.

[Zapytaj teraz](https://community.adobe.com/t5/indesign/ct-p/ct-indesign)

### Skontaktuj się z nami

Fachowe wsparcie w rozwiązaniu problemów.

[Zacznij teraz](/pl/pl/contact.html?step=IDSN)

[^ Do góry](#)

Language Navigation

[](#)

Wybierz region  
Wybranie regionu spowoduje zmianę języka i/lub zawartości na Adobe.com.

*   **Americas**
    *   Brasil
    *   Canada - English
    *   Canada - Français
    *   Latinoamérica
    *   México
    *   Argentina
    *   Colombia
    *   Perú
    *   Chile
    *   United States

*   **Asia Pacific**
    *   Australia
    *   Hong Kong S.A.R. of China
    *   India - English
    *   Indonesia - English
    *   Malaysia - English
    *   New Zealand
    *   Philippines - English
    *   Vietnam - English
    *   中国
    *   中國香港特別行政區
    *   台灣地區
    *   日本
    *   한국
    *   Singapore
    *   Thailand - English
    *   ประเทศไทย

*   **Europe, Middle East and Africa**
    *   Africa - English
    *   België - Nederlands
    *   Belgique - Français
    *   Belgium - English
    *   Česká republika
    *   Danmark
    *   Deutschland
    *   Eesti
    *   España
    *   France
    *   Greece - English
    *   Ireland
    *   Israel - English
    *   Italia
    *   Latvija
    *   Lietuva
    *   Luxembourg - Deutsch
    *   Luxembourg - English
    *   Luxembourg - Français
    *   Magyarország
    *   Middle East and North Africa - English
    *   Nederland
    *   Norge
    *   Österreich
    *   Polska
    *   Portugal
    *   România
    *   Schweiz
    *   Slovenija
    *   Slovensko
    *   Suisse
    *   Suomi
    *   Sverige
    *   Türkiye
    *   United Kingdom
    *   България
    *   Россия
    *   Україна
    *   الشرق الأوسط وشمال أفريقيا - اللغة العربية
    *   ישראל - עברית
    *   Saudi Arabia - English
    *   United Arab Emirates - English
    *   المملكة العربية السعودية
    *   الإمارات العربية المتحدة