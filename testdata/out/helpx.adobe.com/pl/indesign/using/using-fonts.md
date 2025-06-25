window.helpx = window.helpx || {}; window.helpx.analytics = {"deepLinks": {}, "installLinks": {}, "dunamis": {}} window.helpx.analytics.deepLinks.regexPatterns = 'https://creativecloud.adobe.com/campaign/\*,https://\[a-z0-9\]\*.app.link/\*,https://www.adobe.com/(\[a-z\_\]+/)?(express|go)/.\*,https://(new.)?express.adobe.com/\*,https://acrobat.adobe.com/\*,https://photoshop.adobe.com/\*,https://(preview.)?illustrator.adobe.com/\*,https://firefly.adobe.com/\*,https://fonts.adobe.com/\*,https://stock.adobe.com/\*,https://creative.adobe.com/products/download/\*,https://creativecloud.adobe.com/apps/(download|updates)/.\*,https://creativecloud.adobe.com/apps/all/\[a-z0-9\_-\]\*/installation,https://www.adobe.com/(\[a-z\_\]+/)?apps/updates/.\*,acrobat(\[0-9\]{4})://dc/launchTool.\*,(adminconsole|global-admin-console).adobe.com.\*,https://www.adobe.com/campaign/\*'; window.helpx.analytics.deepLinks.label = 'deep link click'; window.helpx.analytics.installLinks.regexPatterns = 'https://creativecloud.adobe.com/apps/all/\*'; window.helpx.analytics.installLinks.label = 'install link click'; window.helpx.analytics.dunamis.stats = {"loadStart": Date.now()}; window.helpx.analytics.dunamis.isEnabled = 'true' === 'true'; window.helpx.analytics.dunamis.projectKey = 'helpx\\u002Dweb\\u002Dservice'; window.helpx.analytics.dunamis.xApiKey = 'helpx\\u002Dweb\\u002Dservice'; window.helpx.analytics.dunamis.env = 'prod'; window.helpx.analytics.dunamis.ingestType = 'dunamis'; 

window.helpx = window.helpx || {}; window.helpx.wallet = {}; window.helpx.wallet.isEnabled = 'true' === 'true'; window.helpx.wallet.env = 'production'; window.helpx.wallet.\_data = {}; window.helpx.wallet.isReady = () => !!window.helpx.wallet.\_data.clientSessionId; window.helpx.wallet.getDetails = () => window.helpx.wallet.\_data; 

Posługiwanie się czcionkami w programie InDesign

window.dexter = window.dexter || {}; window.dexter.utils = window.dexter.utils || {}; if (!window.IntersectionObserver) { document.dispatchEvent(new Event('dexter:headPolyfillLoaded')); window.dexter.utils.headPolyfill = true; } 

function setTheme() { // Set default theme document.documentElement.setAttribute('theme', 'system'); // Set theme per query param; will override default var themeQuery = window.location.search .slice(1) .split('&') .find(function (q) { return q.indexOf('theme=') !== -1; }); if (themeQuery && themeQuery.split('=').length > 1) { var theme = themeQuery.split('=')\[1\]; if (\['light', 'dark'\].includes(theme)) { document.documentElement.setAttribute('theme', theme); } } } setTheme(); 

window.showHelpxCommerceModal = 'true' === 'true'; 

var gnavExp = 'acom/cc-mega-menu/indesign-localnav'; var disableSearchTemplates = \['helpx/components/structure/helpxMain','helpx/components/structure/helpxMain-searchResults'\]; if(URLSearchParams){ var searchParams = new URLSearchParams(window.location.href); gnavExp = searchParams.get('gnavExp') || gnavExp; } var disableGnavTarget = 'true' === "true"; window.fedsConfig = { locale: 'pl', disableSticky: true, disableTarget: disableGnavTarget, content: { experience: gnavExp, }, subnav: {"theme":{"base":"light","gradient":{"toColor":"#FAFAFA","fromColor":"#FAFAFA","opacity":1.0}}}, footer: { regionModal: function () { window.location.hash = 'languageNavigation'; } }, breadcrumbs: { showLogo: true, links: \[\] }, privacy: { otDomainId: '7a5eb705\\u002D95ed\\u002D4cc4\\u002Da11d\\u002D0cc5760e93db' || '7a5eb705-95ed-4cc4-a11d-0cc5760e93db-test', footerLinkSelector: '\[data\\u002Dfeds\\u002Daction=\\x22open\\u002Dadchoices\\u002Dmodal\\x22\]' }, search: { context: '', }, oneTapLogin: false, oneTapRedirectURL: '', universalNav: true, universalNavComponents: 'profile,notifications,appswitcher', disableSearch: disableSearchTemplates.indexOf('helpx/components/structure/helpxMain-article') !== -1 }; (function() { const gnavExperience = document.querySelector('div\[data-param-key\]'); if (gnavExperience === null) return; if (window.location.search === '') return; const key = gnavExperience.getAttribute('data-param-key'); const val = gnavExperience.getAttribute('data-param-val'); const exp = gnavExperience.getAttribute('data-experience'); const queryParams = window.location.search.substring(1); const keyValPairs = queryParams.split('&'); const containsParams = keyValPairs.filter(function(pair) { const splitPair = pair.split('='); return splitPair\[0\] === key && splitPair\[1\] === val; }); if (!containsParams.length) return; if (window.fedsConfig && window.fedsConfig.content && window.fedsConfig.content.experience) { window.fedsConfig.content.experience = exp; } })(); 

window.dexter = window.dexter || {}; window.dexter.jarvis = { isDesktop: (window.dexter.personalization && window.dexter.personalization.technology && window.dexter.personalization.technology.platform && window.dexter.personalization.technology.platform.type) ? window.dexter.personalization.technology.platform.type === 'desktop' : false }; window.dexter.jarvis.desktopEnabled = window.dexter.jarvis.isDesktop && true, window.dexter.jarvis.mobileEnabled = !window.dexter.jarvis.isDesktop && true, window.dexter.jarvis.surfaceName = 'helpx-default', window.dexter.jarvis.surfaceVersion = '1.0', window.dexter.jarvis.onReady = function (newChatEnabled, jarvisData) { if (newChatEnabled) { if (typeof (enableLE) == 'function') { enableLE() }; } else { if (typeof (enableLP) == 'function') { enableLP() }; } }, window.dexter.jarvis.onError = function () { if (typeof (enableLP) == 'function') { enableLP() }; }, window.dexter.jarvis.openExistingChat = function () { if (typeof (enableLP) == 'function') { enableLP() }; }, window.dexter.jarvis.getContext = (window.dexter && window.dexter.callbacks) ? window.dexter.callbacks.getContext : null 

window.fedsConfig = window.fedsConfig || {}; if (window.dexter.jarvis.desktopEnabled || window.dexter.jarvis.mobileEnabled) { window.fedsConfig.jarvis = { surfaceName: 'helpx-default', surfaceVersion: '1.0', onReady: function (newChatEnabled, jarvisData) { // Works for older templates // Disabled for new templates if (typeof (enableLE) == 'function') { enableLE(); } }, onError: function () { // Works for older templates // Disabled for new templates if (typeof (enableLP) == 'function') { enableLP(); } }, openExistingChat: function () { // Works for older templates // Disabled for new templates if (typeof (enableLP) == 'function') { enableLP(); } }, getContext: (window.dexter && window.dexter.callbacks) ? window.dexter.callbacks.getContext : null, directConfig: { lazyLoad: true } } } 

.globalNavHeader { height: 64px; } @media screen and (min-width: 600px) { .globalNavHeader { height: 64px; } } @media screen and (min-width: 1200px) { .globalNavHeader { height: 64px; } } 

(function () { function f() { var scriptEl = document.getElementById("feds-style-page-load"); if (scriptEl) { scriptEl.remove(); } } if (feds && feds.events && feds.events.experience) { f(); } else { window.addEventListener("feds.events.experience.loaded", f, { once: true }); } })(); 

window.helpx = window.helpx || {}; window.helpx.search = window.helpx.search || {}; window.helpx.search.enableAsdeSearch = 'true' === 'true'; if (window.helpx.search.enableAsdeSearch) { window.feds?.utilities?.getUserApplications(); } 

window.helpx = window.helpx || {}; window.helpx.sophiaConfig = {}; window.helpx.sophiaConfig.stageUrl = 'https:\\/\\/p13n\\u002Dstage.adobe.io\\/psdk\\/v2\\/content'; window.helpx.sophiaConfig.prodUrl = 'https:\\/\\/p13n.adobe.io\\/psdk\\/v2\\/content'; window.helpx.sophiaConfig.surfaceID = 'HelpX\_Personalization'; window.helpx.sophiaConfig.apiKey = 'AdobeSupport1'; window.helpx.sophiaConfig.clientCode = 'helpx.adobe.com'; 

window.helpx = window.helpx || {}; window.helpx.ajoConfig = {}; let ajoConfigsurfaceURI = 'web://helpx.adobe.com/#greeting-message-container,web://helpx.adobe.com/#hva,web://helpx.adobe.com/#plan-account,web://helpx.adobe.com/#content-assets-ql,web://helpx.adobe.com/#content-related-article,web://helpx.adobe.com/#content-ads,web://helpx.adobe.com/#content-recommendations'; window.helpx.ajoConfig.surfaceURI = ajoConfigsurfaceURI ? ajoConfigsurfaceURI.split(',') : \[\]; 

window.dexter = window.dexter || {}; window.dexter.Analytics = window.dexter.Analytics || {}; window.dexter.Analytics.language = 'pl\_PL'; window.dexter.Analytics.geoRegion = 'PL'; window.dexter.Analytics.targetEnabled = 'disabled' !== 'disabled'; 

window.dexter.Analytics.launchLoaded = true; window.dexter.Analytics.audienceManagerEnabled = '' !== 'disabled'; window.dexter.Analytics.legacyAnalytics = false; 

window.alloy\_load = window.alloy\_load || {}; window.alloy\_load.data = window.alloy\_load.data || {}; window.alloy\_all = window.alloy\_all || {}; window.alloy\_all.data = window.alloy\_all.data || {}; window.alloy\_all.data.\_adobe\_corpnew = window.alloy\_all.data.\_adobe\_corpnew || {}; window.alloy\_all.data.\_adobe\_corpnew.digitalData = window.alloy\_all.data.\_adobe\_corpnew.digitalData || {}; window.alloy\_all.data.\_adobe\_corpnew.digitalData.page = window.alloy\_all.data.\_adobe\_corpnew.digitalData.page || {}; window.alloy\_all.data.\_adobe\_corpnew.digitalData.page.pageInfo = window.alloy\_all.data.\_adobe\_corpnew.digitalData.page.pageInfo || {}; window.alloy\_all.data.\_adobe\_corpnew.digitalData.page.pageInfo.language = window.dexter.Analytics.language; 

launchURL = "https://assets.adobedtm.com/d4d114c60e50/a0e989131fd5/launch-5dd5dd2177e6.min.js"; edgeConfigId = "913eac4d-900b-45e8-9ee7-306216765cd2"; window.marketingtech = { adobe: { launch: { url: launchURL, controlPageLoad :true }, alloy: { edgeConfigId: edgeConfigId }, target: window.dexter.Analytics.targetEnabled, audienceManager: window.dexter.Analytics.audienceManagerEnabled }, sophia:true } 

window.dexter.Analytics.sophiaEnabled = 'true' !== 'false'; window.dexter.Analytics.ajoEnabled = 'true' === 'true'; 

window.helpx = window.helpx || {}; window.helpx.feds = window.helpx.feds || {}; window.helpx.feds.subscriptions = { 'BILLING': 'true' === 'true', 'OFFER.MERCHANDISING': 'true' === 'true', 'OFFER.PRODUCT\_ARRANGEMENT\_V2': 'true' === 'true' } 

window.helpx = window.helpx || {}; window.helpx.private = window.helpx.private || {}; window.helpx.private = { beta: "" === "true", featurePack: "" === "true", admittedDomains: "" } 

 [_![Adobe InDesign](/content/dam/help/mnemonics/Adobe_InDesign_CC_mnemonic_RGB_64px_no_shadow.png "Adobe InDesign")_ Adobe InDesign](https://www.adobe.com/pl/products/indesign.html)

*   [Funkcje](# "Funkcje")
    
    *   [Nowości](/pl/indesign/using/whats-new.html "Nowości")
    *   [Projektowanie ulotek](https://www.adobe.com/pl/products/indesign/flyer-design-software.html "Projektowanie ulotek")
    *   [Projektowanie plakatów](https://www.adobe.com/pl/products/indesign/poster-design-software.html "Projektowanie plakatów")
    *   [Projektowanie pocztówek](https://www.adobe.com/pl/products/indesign/postcard-design-software.html "Projektowanie pocztówek")
    *   [Projektowanie książek elektronicznych](https://www.adobe.com/pl/products/indesign/ebook-creator-software.html "Projektowanie książek elektronicznych")
    *   [Rozkłady stron](https://www.adobe.com/pl/products/indesign/page-layouts.html "Rozkłady stron")
    *   [Projektowanie broszur](https://www.adobe.com/pl/products/indesign/brochure-design-software.html "Projektowanie broszur")
    *   [Projektowanie CV](https://www.adobe.com/pl/products/indesign/resume-design-software.html "Projektowanie CV")
    *   [Projektowanie prezentacji](https://www.adobe.com/pl/products/indesign/presentation-maker.html "Projektowanie prezentacji")
    *   [Projektowanie menu](https://www.adobe.com/pl/products/indesign/menu-design-software.html "Projektowanie menu")
    
    
    
    
    
*   [Materiały do nauki i pomoc techniczna](/pl/indesign.html?promoid=ZXL8F59B&mv=other "Materiały do nauki i pomoc techniczna")
*   [Wymagania systemowe](/pl/indesign/system-requirements.html "Wymagania systemowe")
*   [Bezpłatna wersja próbna](https://www.adobe.com/pl/products/indesign.html#mini-plans-web-cta-indesign-card "Bezpłatna wersja próbna")

 [Buy now](https://www.adobe.com/pl/creativecloud/plans.html?filter=design&plan=individual&promoid=TKZTLDFL&mv=other "Buy now") 

Podręcznik użytkownikaAnuluj

# Korzystanie z czcionek w programie InDesign

Szukaj

window.usseInfo = { endPoint: 'https://adobesearch.adobe.io/autocomplete/completions', apiKey: 'helpxcomprod', redirectUrl: "" ? "" :'/pl/pl/search.html', autocompleteLocales: 'en,fr,de,ja', }; 

window.helpx = window.helpx || {}; window.helpx.search = window.helpx.search || {}; window.helpx.search.enableAsdeSearch = 'true' === 'true'; 

 Ostatnia aktualizacja 11 gru 2024 

#root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; order: 2;} #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(2) { flex: 0 0 auto; max-width: 100%; width: auto; min-height: auto; order: 3;} #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(3) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; order: 1;} @media screen and (min-width: 600px) { #root\_content\_flex > .dexter-FlexContainer-Items { } #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 58.333333%; max-width: 58.333333%; flex: 1 1 auto; min-height: auto; order: 2; } #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(2) { width: 33.333333%; max-width: 33.333333%; flex: 1 1 auto; min-height: auto; order: 3; } #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(3) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; order: 1; } } @media screen and (min-width: 1200px) { #root\_content\_flex > .dexter-FlexContainer-Items { } #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 66.666667%; max-width: 66.666667%; flex: 1 1 auto; min-height: auto; order: 2; } #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(2) { width: 25%; max-width: 25%; flex: 1 1 auto; min-height: auto; order: 3; } #root\_content\_flex > .dexter-FlexContainer-Items > \*:nth-child(3) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; order: 1; } } 

 [![InDesign](/content/dam/help/mnemonics/id_cc_app_RGB.svg)](https://creativecloud.adobe.com/apps/download/indesign)

[InDesign](https://creativecloud.adobe.com/apps/download/indesign) 

 [Otwórz aplikację](https://creativecloud.adobe.com/apps/download/indesign)  

1.  [Podręcznik użytkownika programu InDesign](/pl/indesign/user-guide.html)
2.  Poznaj program InDesign
    1.  Wprowadzenie do programu InDesign
        1.  [Nowości w programie InDesign](/pl/indesign/using/whats-new.html)
        2.  [Wymagania systemowe](/pl/indesign/system-requirements.html)
        3.  [Często zadawane pytania](/pl/indesign/faq.html)
        4.  [Korzystanie z bibliotek Creative Cloud](/pl/indesign/using/creative-cloud-libraries-sync-share-assets.html)
        5.  [Wydajność GPU](/pl/indesign/kb/gpu-performance.html)
    2.  Przestrzeń robocza
        1.  [Podstawy pracy z przestrzenią roboczą](/pl/indesign/using/workspace-basics.html)
        2.  [Kontekstowy pasek zadań](/pl/indesign/using/contextual-task-bar.html)
        3.  [Dostosowywanie przestrzeni roboczej w programie InDesign](/pl/indesign/using/customizing-workspace.html)
        4.  [Toolbox](/pl/indesign/using/toolbox.html)
        5.  [Ustawianie preferencji](/pl/indesign/using/setting-preferences.html)
        6.  [Panel Właściwości](/pl/indesign/using/properties-panel.html)
        7.  [Dotykowa przestrzeń robocza](/pl/indesign/using/touch-workspace.html)
        8.  [Domyślne skróty klawiaturowe](/pl/indesign/using/default-keyboard-shortcuts.html)
        9.  [Cofanie zmian i zarządzanie panelem Historia](/pl/indesign/using/undo-history-panel.html)
        10.  [Odzyskiwanie dokumentów i cofanie zmian](/pl/indesign/using/recovery-undo.html)
    3.  Generatywna SI (niedostępna w Chinach kontynentalnych)
        1.  [Tekst na obraz](/pl/indesign/using/text-to-image.html)
        2.  [Rozszerzanie generatywne](/pl/indesign/using/generative-expand.html)
        3.  [Wypełnienie generatywne (beta)](#)
        4.  [Często zadawane pytania dotyczące generatywnej SI](/pl/indesign/using/generative-ai-faq-indesign.html)
3.  Tworzenie i układ dokumentów
    1.  Dokumenty i strony
        1.  [Tworzenie dokumentów](/pl/indesign/using/create-documents.html)
        2.  [Praca ze stronami wzorcowymi](/pl/indesign/using/parent-pages.html)
        3.  [Praca ze stronami dokumentu](/pl/indesign/using/pages-spreads-1.html)
        4.  [Ustawianie rozmiaru strony, marginesów i spadów](/pl/indesign/using/adjust-layout.html)
        5.  [Praca z plikami i szablonami](/pl/indesign/using/files-templates.html)
        6.  [Konwertuj pliki PDF na dokumenty InDesign (beta)](#)
        7.  [Tworzenie plików księgi](/pl/indesign/using/creating-book-files.html)
        8.  [Dodawanie podstawowej numeracji stron](/pl/indesign/using/layout-design-9.html)
        9.  [Numerowanie stron, rozdziałów i sekcji](/pl/indesign/using/numbering-pages-chapters-sections.html)
        10.  [Konwertowanie dokumentów QuarkXPress i PageMaker](/pl/indesign/using/converting-quarkxpress-pagemaker-documents.html)
        11.  [Udostępnianie zawartości](/pl/indesign/using/sharing-content.html)
        12.  [Podstawy zarządzania obiegiem pracy](/pl/indesign/using/basic-managed-file-workflow.html)
        13.  [Zapisywanie dokumentów](/pl/indesign/using/saving-documents.html)
    2.  Siatki
        1.  [Siatki](/pl/indesign/using/grids.html)
        2.  [Formatowanie siatek](/pl/indesign/using/formatting-grids.html)
    3.  Narzędzia pomocnicze do tworzenia układu
        1.  [Miarki](/pl/indesign/using/ruler-guides.html)
        2.  [Wyrównywanie i rozmieszczanie obiektów za pomocą miarek](/pl/indesign/using/rulers-measurement-units.html)
        3.  [Mierzenie obiektów za pomocą narzędzia Miarka](/pl/indesign/using/measure-objects.html)
4.  Dodawanie zawartości
    1.  Tekst
        1.  [Dodawanie tekstu do ramek](/pl/indesign/using/adding-text-frames.html)
        2.  [Wątkowanie tekstu](/pl/indesign/using/threading-text.html)
        3.  [Języki Azji Południowo-Wschodniej](/pl/indesign/using/south-east-asian-scripts.html)
        4.  [Obsługa języka arabskiego i hebrajskiego w programie InDesign](/pl/indesign/using/arabic-hebrew.html)
        5.  [Tworzenie tekstu na ścieżce](/pl/indesign/using/creating-type-path.html)
        6.  [Punktowanie i numerowanie](/pl/indesign/using/bullets-numbering.html)
        7.  [Tworzenie wyrażeń matematycznych](/pl/indesign/using/math-expressions.html)
        8.  [Glify i znaki specjalne](/pl/indesign/using/glyphs-special-characters.html)
        9.  [Składanie tekstu](/pl/indesign/using/text-composition.html)
        10.  [Zmienne tekstowe](/pl/indesign/using/text-variables.html)
        11.  [Generowanie kodów QR](/pl/indesign/using/generate-qr-code.html)
        12.  [Edycja tekstu](/pl/indesign/using/editing-text.html)
        13.  [Wyrównywanie tekstu](/pl/indesign/using/aligning-text.html)
        14.  [Oblewanie tekstem wokół obiektów](/pl/indesign/using/text-wrap.html)
        15.  [Zakotwiczone obiekty](/pl/indesign/using/anchored-objects.html)
        16.  [Zawartość połączona](/pl/indesign/using/linked-content.html)
        17.  [Formatowanie akapitów](/pl/indesign/using/formatting-paragraphs.html)
        18.  [Formatowanie znaków](/pl/indesign/using/formatting-characters.html)
    2.  Typografia
        1.  [Korzystanie z czcionek w programie InDesign](/pl/indesign/using/using-fonts.html)
        2.  [Kerning i światło](/pl/indesign/using/kerning-tracking.html)
        3.  [Skalowanie i pochylenie tekstu](/pl/indesign/using/scale-skew-type.html)
        4.  [Stosowanie efektów kolorystycznych do tekstu](/pl/indesign/using/color-type.html)
    3.  Formatowanie tekstu
        1.  [Formatowanie tekstu](/pl/indesign/using/formatting-text.html)
        2.  [Automatyczny styl tekstu](/pl/indesign/using/auto-style-text.html)
        3.  [Praca z pakietami stylów](/pl/indesign/using/work-with-style-packs.html)
        4.  [Tabulatory i wcięcia](/pl/indesign/using/tabs-indents.html)
    4.  Recenzowanie tekstu
        1.  [Śledzenie i przeglądanie zmian](/pl/indesign/using/tracking-reviewing-changes.html)
        2.  [Dodawanie notatek redaktorskich w programie InDesign](/pl/indesign/using/adding-editorial-notes-indesign.html)
        3.  [Importowanie komentarzy z pliku PDF](/pl/indesign/using/import-pdf-comments.html)
    5.  Słowniki ortograficzne i językowe
        1.  [Sprawdzanie pisowni, autokorekta i dynamiczne sprawdzanie pisowni](/pl/indesign/using/spell-checking-language-dictionaries.html)
        2.  [Tworzenie i dodawanie słowników i wyrazów oraz zarządzanie nimi](/pl/indesign/using/create-manage-dictionaries.html)
        3.  [Zmiana preferencji słownika](/pl/indesign/using/change-dictionary-preferences.html)
        4.  [Słownik Duden](/pl/indesign/using/duden-dictionary.html)
    6.  Dodawanie odwołań
        1.  [Tworzenie spisu treści](/pl/indesign/using/creating-table-contents.html)
        2.  [Przypisy dolne](/pl/indesign/using/footnotes.html)
        3.  [Tworzenie indeksu](/pl/indesign/using/creating-index.html)
        4.  [Przypisy końcowe](/pl/indesign/using/endnotes.html)
        5.  [Podpisy](/pl/indesign/using/captions.html)
    7.  Style
        1.  [Style akapitowe i znakowe](/pl/indesign/using/paragraph-character-styles.html)
        2.  [Odwzorowanie i eksportowanie stylów i zarządzanie nimi](/pl/indesign/using/map-export-manage-styles.html)
        3.  [Style obiektowe](/pl/indesign/using/object-styles.html)
        4.  [Inicjały i style zagnieżdżone](/pl/indesign/using/drop-caps-nested-styles.html)
        5.  [Praca ze stylami](/pl/indesign/using/styles.html)
        6.  [Interlinia](/pl/indesign/using/leading.html)
    8.  Tabele
        1.  [Formatowanie tabel](/pl/indesign/using/formatting-tables.html)
        2.  [Tworzenie tabel](/pl/indesign/using/creating-tables.html)
        3.  [Style tabeli i komórki](/pl/indesign/using/table-cell-styles.html)
        4.  [Zaznaczanie i edycja tabel](/pl/indesign/using/selecting-editing-tables.html)
        5.  [Obrysy i wypełnienia tabel](/pl/indesign/using/table-strokes-fills.html)
    9.  Interaktywność
        1.  [Hiperłącza](/pl/indesign/using/hyperlinks.html)
        2.  [Dynamiczne dokumenty PDF](/pl/indesign/using/dynamic-pdf-documents.html)
        3.  [Zakładki](/pl/indesign/using/bookmarks.html)
        4.  [Przyciski](/pl/indesign/using/interactivity-5.html)
        5.  [Formularze](/pl/indesign/using/forms.html)
        6.  [Animacja](/pl/indesign/using/animation.html)
        7.  [Odsyłacze](/pl/indesign/using/cross-references.html)
        8.  [Pliki PDF ze strukturą](/pl/indesign/using/structuring-pdfs.html)
        9.  [Przejścia stron](/pl/indesign/using/page-transitions.html)
        10.  [Filmy i dźwięki](/pl/indesign/using/movies-sounds.html)
    10.  Grafika
        1.  [Informacje o ścieżkach i kształtach](/pl/indesign/using/paths-shapes.html)
        2.  [Rysowanie narzędziem Ołówek](/pl/indesign/using/drawing-pencil-tool.html)
        3.  [Rysowanie narzędziem Pióro](/pl/indesign/using/drawing-pen-tool.html)
        4.  [Stosowanie ustawień linii (obrysów)](/pl/indesign/using/applying-line-stroke-settings.html)
        5.  [Ścieżki i kształty złożone](/pl/indesign/using/compound-paths-shapes.html)
        6.  [Edycja ścieżek](/pl/indesign/using/editing-paths.html)
        7.  [Ścieżki przycinające](/pl/indesign/using/clipping-paths.html)
        8.  [Zmiana wyglądu narożnika](/pl/indesign/using/change-corner-appearance.html)
        9.  [Ramki i obiekty](/pl/indesign/using/frames-objects.html)
        10.  [Wyrównywanie i rozmieszczanie obiektów](/pl/indesign/using/aligning-distributing-objects.html)
        11.  [Grafika połączona i osadzona](/pl/indesign/using/graphics-links.html)
        12.  [Integrowanie zasobów z systemu AEM](/pl/enterprise/using/adobe-asset-link.html)
    11.  Kolory i przezroczystość
        1.  [Stosowanie koloru](/pl/indesign/using/apply-color.html)
        2.  [Używanie kolorów z grafiki importowanej](/pl/indesign/using/using-colors-imported-graphics.html)
        3.  [Praca z próbkami](/pl/indesign/using/swatches.html)
        4.  [Mieszanie farb](/pl/indesign/using/mixing-inks.html)
        5.  [Tinty](/pl/indesign/using/tints.html)
        6.  [Informacje o kolorach dodatkowych i podstawowych](/pl/indesign/using/spot-process-colors.html)
        7.  [Mieszanie kolorów](/pl/indesign/using/blending-colors.html)
        8.  [Gradienty](/pl/indesign/using/gradients.html)
        9.  [Spłaszczanie grafiki zawierającej przezroczystość](/pl/indesign/using/flattening-transparent-artwork.html)
        10.  [Dodawanie efektów przezroczystości](/pl/indesign/using/adding-transparency-effects.html)
5.  Wyszukiwanie i zastępowanie
    1.  [Wyszukiwanie i zastępowanie tekstu](/pl/indesign/using/find-change.html)
    2.  [Wyszukiwanie i zastępowanie czcionek](/pl/indesign/using/find-replace-fonts.html)
    3.  [Wyszukiwanie i zastępowanie glifów](/pl/indesign/using/find-replace-glyphs.html)
    4.  [Wyszukiwanie i zastępowanie w wyrażeniach i zapytaniach GREP](/pl/indesign/using/find-replace-grep-queries.html)
    5.  [Wyszukiwanie i zastępowanie obiektów](/pl/indesign/using/find-replace-objects.html)
    6.  [Wyszukiwanie i zastępowanie kolorów](/pl/indesign/using/find-replace-colors.html)
    7.  [Opcje wyszukiwania do znajdowania i zastępowania](/pl/indesign/using/search-options.html)
6.  Udostępnianie
    1.  [Zapisywanie dokumentów w chmurze i dostęp do nich](/pl/indesign/using/cloud-documents.html)
    2.  [Organizowanie i udostępnianie dokumentów w chmurze oraz zarządzanie nimi](/pl/indesign/using/manage-cloud-documents.html)
    3.  [Wyświetlanie wersji dokumentów w chmurze i zarządzanie nimi](/pl/indesign/using/view-manage-versions.html)
    4.  [Często zadawane pytania dotyczące dokumentów InDesign w chmurze](/pl/indesign/using/cloud-documents-faq.html)
    5.  [InCopy w sieci (Beta)](#)
    6.  [Udostępnianie i współpraca](/pl/indesign/using/share-and-collaborate.html)
    7.  [Udostępnianie do recenzji](/pl/indesign/using/share-for-review.html)
    8.  [Recenzja udostępnionego dokumentu InDesign](/pl/indesign/using/review-indesign-document.html)
    9.  [Zarządzanie opiniami](/pl/indesign/using/manage-feedback.html)
    10.  [Zapraszanie do edycji](/pl/indesign/using/invite-to-edit.html)
7.  Eksportowanie, importowanie i publikowanie
    1.  Umieszczanie, eksportowanie i publikowanie
        1.  [Publikacja elektroniczna](/pl/indesign/using/publish-online.html)
        2.  [Konsola funkcji Publikacja elektroniczna](/pl/indesign/using/publish-online-dashboard.html)
        3.  [Kopiowanie i wstawianie grafiki](/pl/indesign/using/placing-graphics.html)
        4.  [Eksportowanie do programu Adobe Express](/pl/indesign/using/export-to-express.html)
        5.  [Eksport zawartości do formatu EPUB](/pl/indesign/using/export-content-epub-cc.html)
        6.  [Opcje Adobe PDF](/pl/indesign/using/pdf-options.html)
        7.  [Eksportowanie do HTML5](/pl/indesign/using/export-to-html5.html)
        8.  [Eksportowanie zawartości do formatu HTML (starsza wersja)](/pl/indesign/using/export-content-html-cc.html)
        9.  [Eksportowanie do formatu Adobe PDF](/pl/indesign/using/exporting-publishing-pdf.html)
        10.  [Eksportowanie do formatu JPEG lub PNG](/pl/indesign/using/export-jpeg-format.html)
        11.  [Importowanie plików SVG](/pl/indesign/using/import-svg-files.html)
        12.  [Obsługiwane formaty plików](/pl/indesign/using/supported-file-formats.html)
        13.  [Eksport i import ustawień użytkownika](/pl/indesign/using/export-import-user-settings-indesign.html)
    2.  Drukowanie
        1.  [Drukowanie broszur](/pl/indesign/using/printing-booklets.html)
        2.  [Znaczniki drukarskie i spady](/pl/indesign/using/printers-marks-bleeds.html)
        3.  [Drukowanie dokumentów](/pl/indesign/using/printing-documents.html)
        4.  [Farby, rozbarwienia i liniatura rastra](/pl/indesign/using/inks-separations-screen-frequency.html)
        5.  [Nadruk](/pl/indesign/using/overprinting.html)
        6.  [Tworzenie plików PostScript i EPS](/pl/indesign/using/creating-postscript-eps-files.html)
        7.  [Inspekcja wstępna plików przed przekazaniem](/pl/indesign/using/preflighting-files-handoff.html)
        8.  [Drukowanie miniaturek i bardzo dużych dokumentów](/pl/indesign/using/printing-thumbnails-oversized-documents.html)
        9.  [Przygotowywanie plików PDF dla usługodawców](/pl/indesign/using/preparing-pdfs-service-providers.html)
        10.  [Przygotowanie do drukowania rozbarwień](/pl/indesign/using/preparing-print-separations.html)
8.  Rozszerzanie programu InDesign
    1.  Automatyzacja
        1.  [Scalanie danych](/pl/indesign/using/data-merge.html)
        2.  [Wtyczki](/pl/indesign/using/plug-ins.html)
        3.  [Rozszerzenie Capture w programie InDesign](/pl/indesign/using/capture-extension-in-indesign.html)
        4.  [Tworzenie skryptów](/pl/indesign/using/scripting.html)
9.  Rozwiązywanie problemów
    1.  [Rozwiązane problemy](/pl/indesign/kb/fixed-issues.html)
    2.  [Znane problemy](/pl/indesign/kb/known-issues.html)
    3.  [Awaria podczas uruchamiania programu](/pl/indesign/kb/crash-on-launch.html)
    4.  [Błąd — folder preferencji tylko do odczytu](/pl/indesign/kb/preferences-folder-read-only-issue.html)
    5.  [Rozwiązywanie problemów z plikami](/pl/indesign/kb/troubleshoot-file-issues.html)
    6.  [Nie można wyeksportować pliku programu InDesign do pliku PDF](/pl/indesign/kb/unable-to-export-pdf.html)
    7.  [Odzyskiwanie dokumentów InDesign](/pl/indesign/kb/indesign-document-recovery.html)

_Czcionka_ to pełny zestaw znaków — liter, cyfr i symboli — które mają tę samą grubość, szerokość i styl; na przykład: „10‑punktowa pogrubiona czcionka Adobe Garamond Bold”. 

_Kroje pisma_ ( zwane też _rodzinami czcionek_) to zestawy czcionek o podobnym wyglądzie .przeznaczonych do użytku obok siebie. Przykładem jest Adobe Garamond.

_Styl czcionki_ to wariant jednej z czcionek w rodzinie. Na ogół podstawową czcionką jest czcionka _Łacińska_ lub _Zwykła_ (nazwy te są różne w różnych rodzinach czcionek). Oprócz tego rodzina może zawierać takie style, jak zwykła, pogrubiona, półgruba, kursywa i pogrubiona kursywa. 

## Typy czcionki

Przykłady czcionki są dostępne w menu rodziny czcionek i stylu czcionki w palecie Typografia oraz innych obszarach aplikacji, skąd można wybierać czcionki. Ponadto różne rodzaje czcionek są oznaczane przez specjalne ikony:

*   OpenType
*   SVG OpenType
*   Czcionki zmienne

*   TrueType
*   Adobe Fonts

*   Multiple Master
*   Kompozytowe

W oknie preferencji tekstu można wyłączyć funkcję podglądu oraz zmienić wielkość nazw czcionek w punktach lub używane przykłady.

Aby wyświetlić listę czcionek dostępnych w programie InDesign, należy wykonać jedną z następujących czynności:

*   Otwórz panel Typografia (Ctrl + T) > menu rozwijane Rodzina czcionek
*   Otwórz panel Sterowanie > menu rozwijane Rodzina czcionek
*   Otwórz panel Właściwości > menu rozwijane Rodzina czcionek

## Praca z brakującymi czcionkami

Ważne przypomnienie:![](/content/dam/download.svg)

W styczniu 2023 r. firma Adobe zakończyła obsługę tworzenia za pomocą czcionek Type 1. Więcej informacji podano w artykule pomocy [Zakończenie obsługi czcionek PostScript Type 1](/pl/fonts/kb/postscript-type-1-fonts-end-of-support.html).  

#root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table { width: 100%; } #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > th, #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > td { border: 1px solid #bdbdbd; } #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > th:before { border: 1px solid #bdbdbd; left: -1px; top: -1px; } #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > th.row-r0, #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > td.row-r0 { } #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > th.row-r1, #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > td.row-r1 { } #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > th.column-c0, #root\_content\_flex\_items\_position\_position-par\_table\_copy\_copy > .dexter-Table > tbody > tr > td.column-c0 { vertical-align: top; } 

## Instalowanie czcionek

Informacje na temat instalowania i uaktywniania czcionek do użytku we wszystkich aplikacjach można znaleźć w dokumentacji systemu lub oprogramowania do zarządzania czcionkami.

Czcionki można udostępnić w programie InDesign, kopiując ich pliki do folderu Fonts (Czcionki) w folderze programu InDesign na dysku twardym. Czcionki z tego folderu są jednak dostępne tylko w programie InDesign.

Jeśli dwie lub więcej czcionek aktywnych w programie InDesign używa tej samej nazwy rodziny, ale ma różne nazwy Adobe PostScript, czcionki te będą dostępne w programie InDesign. Powielone czcionki są wymieniane w odpowiednich menu ze skróconą nazwą technologii czcionki w nawiasach. Na przykład, czcionka Helvetica TrueType ma postać „Helvetica (TT)”, czcionka Helvetica PostScript Type 1 ma postać „Helvetica (T1)”, a czcionka Helvetica OpenType ma postać „Helvetica (OTF)”. Jeżeli dwie czcionki mają taką samą nazwę postscriptową, ale jedna z nich zawiera w nazwie ciąg .dfont, to używana będzie ta druga czcionka.

## Automatyczna aktywacja brakujących czcionek

Jeśli dokument programu InDesign zawiera brakujące czcionki, są one automatyczne aktywowane w tle za pomocą Adobe Fonts i nie jest wyświetlane okno dialogowe **Brakujące czcionki**. Brakujące czcionki zostaną zastąpione zgodnymi czcionkami z usługi Adobe Fonts.

#root\_content\_flex\_items\_position\_position-par\_image\_1327926115\_cop { width:{Long}800px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115\_cop { width:{Long}800px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115\_cop { width:{Long}800px; } } 

![Automatyczna aktywacja brakujących czcionek](/content/dam/help/pl/indesign/using/using-fonts/jcr_content/main-pars/image_1327926115_cop/auto-activate-adobe-fonts_22.gif "Automatyczna aktywacja czcionek Adobe Fonts")

Gdy włączona jest automatyczna aktywacja czcionek Adobe Fonts 

#root\_content\_flex\_items\_position\_position-par\_image\_1327926115\_cop { width:{Long}800px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115\_cop { width:{Long}800px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115\_cop { width:{Long}800px; } } 

![Automatyczna aktywacja brakujących czcionek](/content/dam/help/pl/indesign/using/using-fonts/jcr_content/main-pars/image_1327926115_cop/auto-activate-adobe-fonts_22.gif "Automatyczna aktywacja czcionek Adobe Fonts")

Gdy włączona jest automatyczna aktywacja czcionek Adobe Fonts 

**Automatyczna aktywacja czcionek Adobe Fonts** jest domyślnie wyłączona w programie InDesign. Aby ją włączyć, zaznacz opcję **Automatyczna aktywacja czcionek Adobe Fonts** w oknie **Edycja >** **Preferencje > Obsługa plików.**

## Gdy włączona jest automatyczna aktywacja czcionek Adobe Fonts

Jeśli w dokumencie brakuje niektórych czcionek, program InDesign automatycznie aktywuje takie czcionki z serwisu Adobe Fonts. 

#root\_content\_flex\_items\_position\_position-par\_image\_1327926115 { width:{Long}800px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115 { width:{Long}800px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115 { width:{Long}800px; } } 

    ![Automatyczna aktywacja brakujących czcionek](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4MDAiIGhlaWdodD0iMjUwIj48cmVjdCB3aWR0aD0iODAwIiBoZWlnaHQ9IjI1MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Automatyczna aktywacja czcionek Adobe Fonts") 

Włączona automatyczna aktywacja czcionek z usługi Adobe Fonts 

#root\_content\_flex\_items\_position\_position-par\_image\_1327926115 { width:{Long}800px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115 { width:{Long}800px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1327926115 { width:{Long}800px; } } 

    ![Automatyczna aktywacja brakujących czcionek](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4MDAiIGhlaWdodD0iMjUwIj48cmVjdCB3aWR0aD0iODAwIiBoZWlnaHQ9IjI1MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Automatyczna aktywacja czcionek Adobe Fonts") 

Włączona automatyczna aktywacja czcionek z usługi Adobe Fonts 

Proces aktywacji odbywa się w tle.

*   Jeżeli wszystkie brakujące czcionki są dostępne w Adobe Fonts, zostaną one aktywowane w tle. Można będzie kontynuować pracę nad dokumentami.
*   Jeżeli w Adobe Fonts są dostępne tylko niektóre z brakujących czcionek, to zostaną one aktywowane w tle. W oknie dialogowym Brakujące czcionki wyświetlona zostanie lista brakujących czcionek.
    *   Kliknij przycisk Zastąp czcionki i pobierz brakujące czcionki z innych źródeł lub
    *   kliknij przycisk Pomiń, aby zamknąć to okno dialogowe. Brakujące czcionki zostaną zastąpione domyślnymi.
*   Jeśli żadna z brakujących czcionek nie jest dostępna w usłudze Adobe Fonts, wyświetlone zostanie okno Brakujące czcionki z listą brakujących czcionek.

#root\_content\_flex\_items\_position\_position-par\_image\_344546644 { width:{Long}800px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_344546644 { width:{Long}800px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_344546644 { width:{Long}800px; } } 

    ![okno dialogowe Brakujące czcionki](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4MDAiIGhlaWdodD0iMzYwIj48cmVjdCB3aWR0aD0iODAwIiBoZWlnaHQ9IjM2MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Gdy żadna z brakujących czcionek nie jest dostępna w usłudze Adobe Fonts") 

Gdy żadna z brakujących czcionek nie jest dostępna w usłudze Adobe Fonts 

#root\_content\_flex\_items\_position\_position-par\_image\_344546644 { width:{Long}800px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_344546644 { width:{Long}800px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_344546644 { width:{Long}800px; } } 

    ![okno dialogowe Brakujące czcionki](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4MDAiIGhlaWdodD0iMzYwIj48cmVjdCB3aWR0aD0iODAwIiBoZWlnaHQ9IjM2MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Gdy żadna z brakujących czcionek nie jest dostępna w usłudze Adobe Fonts") 

Gdy żadna z brakujących czcionek nie jest dostępna w usłudze Adobe Fonts 

Można również sprawdzić postęp aktywacji brakujących czcionek na panelu Zadania w tle. Zadania w tle można wyświetlić na dwa sposoby:

*   Kliknij polecenie Okno **>** Użytki **>** Zadania w tle.
*   Kliknij ikonę niebieskiego kółka w prawym górnym rogu paska aplikacji InDesign.

## Gdy wyłączona jest automatyczna aktywacja czcionek Adobe Fonts

Jeśli nie włączono opcji Automatyczna aktywacja czcionek Adobe Fonts w oknie dialogowym Preferencje, a w dokumencie są brakujące czcionki, wyświetlone zostanie okno dialogowe Brakujące czcionki. Kliknij przycisk Aktywuj, aby ręcznie aktywować brakujące czcionki w usłudze Adobe Fonts.

## Czcionki OpenType

Każda z czcionek OpenType jest zdefiniowana w pliku zgodnym zarówno z systemem Windows®, jak i Macintosh®, dzięki czemu pliki takich czcionek mogą być przenoszone między różnymi platformami systemowymi bez obaw o niepożądane podstawienia czcionek. Mogą one zawierać wiele różnych elementów, takich jak znaki kaligraficzne i ligatury specjalne, które nie są dostępne w oferowanych obecnie czcionkach PostScript i TrueType.

  Uwaga:

Przy czcionkach OpenType wyświetlana jest ikona. 

Korzystając z czcionek OpenType, można automatycznie podstawiać w tekście glify alternatywne, takie jak ligatury, kapitaliki, ułamki i antykwy. 

#root\_content\_flex\_items\_position\_position-par\_image { width:{Long}374px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image { width:{Long}374px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image { width:{Long}374px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNzQiIGhlaWdodD0iMTkwIj48cmVjdCB3aWR0aD0iMzc0IiBoZWlnaHQ9IjE5MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "stx_opentype")

Czcionki zwykłe (po lewej) i czcionki OpenType (po prawej) 

**A.** Liczebniki porządkowe **B.** Ligatury ozdobne **C.** Znaki kaligraficzne  

#root\_content\_flex\_items\_position\_position-par\_image { width:{Long}374px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image { width:{Long}374px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image { width:{Long}374px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNzQiIGhlaWdodD0iMTkwIj48cmVjdCB3aWR0aD0iMzc0IiBoZWlnaHQ9IjE5MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "stx_opentype")

Czcionki zwykłe (po lewej) i czcionki OpenType (po prawej) 

Czcionki OpenType mogą obejmować znaki z zestawu rozszerzonego, mogą też obsługiwać pewne funkcje układu — funkcje zapewniające większą kontrolę lingwistyczną i typograficzną nad tekstem. Czcionki OpenType od firmy Adobe, które obsługują języki Europy środkowej (CE), do nazwy wyświetlanej w menu czcionek aplikacji mają dołączony wyraz „Pro”. Czcionki OpenType, które nie obsługują języków europejskich, są oznaczone etykietą „Standard,” i posiadają przyrostek „Std”. Wszystkie czcionki OpenType mogą być instalowane i używane razem z czcionkami TrueType.

Więcej informacji o czcionkach OpenType można uzyskać pod adresem [www.adobe.com/go/opentype\_pl](https://www.adobe.com/go/opentype_pl).

## Stosowanie atrybutów czcionek OpenType

### Stosowanie atrybutów czcionek OpenType za pomocą panelu Typografia lub Sterowanie

Stosowanie atrybutów czcionek OpenType, takich jak ułamki czy znaki kaligraficzne, umożliwiają panele Typografia i Sterowanie. 

Więcej informacji o czcionkach OpenType można uzyskać pod adresem [www.adobe.com/go/opentype\_pl](https://www.adobe.com/go/opentype_pl).

1.  Sprawdź, czy na panelu Typografia lub w panelu Sterowanie jest wybrana czcionka OpenType.
    
    
    
    
    
2.  Z menu panelu Typografia wybierz polecenie OpenType, a następnie atrybut OpenType, np. Ligatury specjalne lub Ułamki.
    
    
    
    
    
    
    

Funkcje nieobsługiwane przez wybraną czcionkę są ujmowane w nawiasy, np. \[Kaligraficzne\].

  Uwaga:

Atrybuty czcionek OpenType można wybierać także podczas definiowania stylu akapitowego lub znakowego. Służy do tego sekcja Cechy OpenType w oknie dialogowym Opcje stylu.

### Stosowanie atrybutów czcionek OpenType za pomocą menu kontekstowego

Aby użyć do zaznaczonego tekstu atrybutów czcionek OpenType, które mają do niego zastosowanie, skorzystaj z menu kontekstowego.

1.  Zaznacz tekst lub ramkę tekstową.
    
    
    
    
    
    
    
2.  W wyskakującym oknie wybierz atrybut OpenType, np. Liczby porządkowe lub Ułamki. Jeśli po zaznaczeniu tekstu lub ramki tekstowej na ekranie pojawi się etykietka, kliknij ją, aby wyświetlić listę atrybutów OpenType.
    
    
    
    
    
    
    

  Uwaga:

*   Etykietka wyboru atrybutów OpenType nie jest wyświetlana w powiązanej ramce tekstowej.
*   Opcja dodawania atrybutów czcionek OpenType za pomocą menu kontekstowego nie jest dostępna w przypadku układaczy World-Ready.

### Atrybuty czcionek OpenType

Jeżeli w tekście są używane czcionki OpenType, to przy formatowaniu lub definiowaniu stylów można wybierać z menu panelów Sterowanie i Typografia atrybuty OpenType.

  Uwaga:

Należy pamiętać, że czcionki OpenType różnią się ogromnie między sobą pod względem oferowanych stylów i funkcji. Jeżeli dana funkcja OpenType nie jest dostępna, jej nazwa będzie wyświetlana na panelu Sterowanie w nawiasach (np. \[Kaligraficzne\]).

 Ligatury ozdobne 

Producenci czcionki mogą dołączyć do niej dodatkowe ligatury, które nie powinny być używane we wszystkich okolicznościach. Wybranie tej opcji pozwoli na używanie ligatur niestandardowych, o ile są one obecne. Więcej informacji o ligaturach znajduje się w części [Stosowanie ligatur do par liter](/pl/indesign/using/formatting-characters.html#apply_ligatures_to_letter_pairs).

 Ułamki 

Liczby oddzielone ukośnikiem (np. 1/2) są konwertowane na znak ułamka, o ile funkcja ułamków jest dostępna.

 Liczebniki porządkowe 

Gdy opcja ta jest dostępna, angielskie liczebniki porządkowe, takie jak _1st_ i _2nd_, są formatowane z literami w indeksie górnym (1st i 2nd). Odpowiednio formatowane są także takie litery, jak _a_ i _o_ w hiszpańskich słowach _segunda_ (2a) i _segundo_ (2o).

 Znak kaligraficzny 

Gdy opcja ta jest dostępna, można korzystać z kaligraficznych znaków zwykłych i kontekstowych, które mogą obejmować alternatywne wersaliki i alternatywne znaki na końcu wyrazu.

 Znaki tytułowe 

Gdy opcja ta jest dostępna, uaktywniane są znaki używane w tytułach pisanych wielkimi literami. W przypadku niektórych czcionek wybranie tej opcji dla tekstu zawierającego zarówno wielkie, jak i małe litery może spowodować niepożądane efekty.

 Warianty kontekstowe 

Gdy opcja ta jest dostępna, uaktywniane są ligatury kontekstowe i alternatywne znaki łącznikowe. Są to znaki alternatywne, używane w niektórych tylko krojach czcionek, a zapewniające lepszy wygląd niektórych połączeń znakowych. Na przykład, połączenie liter „bl” w słowie „blask” sprawia, że przypominają one litery pisane odręcznie. Opcja ta jest domyślnie zaznaczona.

 Tylko kapitaliki 

Jeżeli czcionka zawiera prawdziwe kapitaliki, to włączenie tej opcji spowoduje przekształcenie znaków na kapitaliki. Więcej informacji na ten temat znajduje się w części [Zmiana wielkości liter w tekście](/pl/indesign/using/formatting-characters.html#change_the_case_of_type).

 Przekreślone zero 

Zaznaczenie tej opcji powoduje, że na cyfrze _0_ jest wyświetlana ukośna kreska. W przypadku niektórych czcionek (zwłaszcza zagęszczonych), trudno jest odróżnić cyfrę _0_ od dużej litery _O_.

 Zestawy stylistyczne 

Niektóre czcionki OpenType zawierają alternatywne zestawy glifów, dające efekt estetyczny. _Zestaw stylistyczny_ to grupa wariantów glifów, które można zastosować do pojedynczych znaków albo do zakresu tekstu. Jeżeli wybierze się inny zestaw stylistyczny, to użyte zostaną glify z tego zestawu, a nie domyślne glify czcionki. Jeśli glif w zestawie stylistycznym używany jest w połączeniu z innym ustawieniem OpenType, glif z indywidualnego ustawienia zastąpi glif z zestawu znaków. Glify z każdego zestawu można oglądać za pomocą panelu Glify.

 Kształty pozycyjne 

W niektórych językach, np. arabskich, wygląd znaku zależy od jego położenia wewnątrz słowa. Dany znak może wyglądać inaczej, gdy stoi na początku słowa (pozycja początkowa), w środku słowa (pozycja środkowa), na końcu słowa (pozycja końcowa) lub w ogóle poza słowem (pozycja izolowana). Należy zaznaczyć znak i wybrać odpowiednią, tj. określającą format znaku, opcję Kształty pozycyjne. Zaznaczenie opcji Kształt ogólny powoduje wstawienie typowego znaku, natomiast zaznaczenie opcji Kształt automatyczny powoduje dostosowanie kształtu znaku do jego położenia wewnątrz słowa lub tekstu. 

 Indeks górny/dolny i opuszczenie 

Niektóre czcionki OpenType zawierają podniesione i opuszczone glify, które są odpowiednio przeskalowane w stosunku do otaczających znaków. Jeżeli czcionka OpenType nie zawiera takich glifów dla niestandardowych ułamków, można skorzystać z atrybutów Liczebnik i Mianownik.

 Liczebnik i mianownik 

Niektóre czcionki OpenType konwertują na glify specjalne tylko podstawowe ułamki (np. 1/2 lub 1/4), nie konwertują natomiast na glify ułamkowe ułamków niestandardowych (np. 4/13 lub 99/100). W takich przypadkach można użyć do niestandardowych ułamków atrybutów Liczebnik i Mianownik.

 Cyfry o stałej szerokości 

Cyfry o pełnej wysokości mają te same szerokości. Opcja ta jest przydatna w sytuacjach, gdzie liczby muszą być równo ustawione w kolejnych wierszach, tak jak np. w tabelach.

 Cyfry nautyczne proporcjonalne 

Oferuje cyfry o różnej wysokości i szerokości. Opcja ta jest zalecana dla klasycznego, wyrafinowanego stylu tekstu, który nie używa samych wersalików.

 Cyfry proporcjonalne 

Oferuje cyfry o pełnej wysokości i różnych szerokościach. Opcja ta jest zalecana dla tekstu, który używa samych wersalików.

 Cyfry nautyczne o stałej szerokości 

Oferuje cyfry o różnej wysokości, ale takiej samej, stałej szerokości. Opcja ta jest zalecana w sytuacjach, gdy pożądany jest klasyczny wygląd cyfr typu antykwa, ale muszą się one układać w kolumny, jak np. w raporcie rocznym.

 Cyfry domyślne 

Glify cyfr używają domyślnego stylu cyfry bieżącej czcionki.

## Czcionki SVG OpenType

Program InDesign obsługuje czcionki SVG OpenType, takie jak czcionki kolorowe i czcionki emotikonów. Czcionki SVG OpenType zawierają wiele kolorów i gradientów w pojedynczym glifie. 

#root\_content\_flex\_items\_position\_position-par\_image\_2001365663 { width:{Long}400px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_2001365663 { width:{Long}400px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_2001365663 { width:{Long}400px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MDAiIGhlaWdodD0iMTkxIj48cmVjdCB3aWR0aD0iNDAwIiBoZWlnaHQ9IjE5MSIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Czcionki SVG OpenType")

Czcionki SVG OpenType: wiele kolorów i gradientów 

#root\_content\_flex\_items\_position\_position-par\_image\_2001365663 { width:{Long}400px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_2001365663 { width:{Long}400px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_2001365663 { width:{Long}400px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MDAiIGhlaWdodD0iMTkxIj48cmVjdCB3aWR0aD0iNDAwIiBoZWlnaHQ9IjE5MSIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Czcionki SVG OpenType")

Czcionki SVG OpenType: wiele kolorów i gradientów 

Czcionki emotikonów pozwalają używać w dokumentach różnych kolorowych i graficznych znaków, zawierających na przykład emotikony, flagi, znaki uliczne, zwierzęta, osoby, jedzenie i atrakcje turystyczne. Czcionki emotikonów SVG OpenType, na przykład czcionka EmojiOne, umożliwiają tworzenie niektórych glifów kompozytowych na podstawie glifu lub kilku glifów. Program pozwala na przykład tworzyć flagi państw lub zmieniać kolory skóry w przypadku glifów przedstawiających osoby lub części ciała — takie jak dłonie czy nos.

Aby użyć czcionek SVG OpenType, wykonaj następujące czynności:

1.  Utwórz obiekt tekstowy za pomocą narzędzia Tekst.
    
    
    
    
    
    
    
    
2.  Ustaw czcionkę SVG OpenType. Takie czcionki są oznaczone przy użyciu ikony na liście czcionek.
    
    
    
    
    
    
    
3.  Wybierz określone glify za pomocą panelu Glify. Aby wyświetlić panel Glify, wybierz opcję Tekst \> Glify. Panel Glify można również otworzyć, wybierając opcje Okno \> Tekst i tabele \> Glify.
    
    
    
    
    
    
    
    

### Tworzenie glifów kompozytowych

Na potrzeby tej ilustracji rozważamy czcionkę SVG OpenType emotikonów EmojiOne. Glify można tworzyć, łącząc szereg znaków z czcionki SVG OpenType EmojiOne.

Można na przykład tworzyć flagi państw lub zmieniać kolory skóry w domyślnych znakach przedstawiających pojedynczą osobę lub część ciała (zazwyczaj kolorem tym jest żółty, niebieski lub szary).

  Uwaga:

Glify w czcionce emotikonów, takiej jak EmojiOne, różnią się od liter na klawiaturze. Glify te są traktowane jako oddzielne znaki i są dostępne tylko w panelu Glify (bez dostępu na klawiaturze).

 Tworzenie flag państw 

„Litery” (A, B, C, D itd.) w przypadku czcionki EmojiOne nie odpowiadają klawiszom klawiatury. Po połączeniu znaków w panelu Glify w celu utworzenia kodu ISO kraju dwa znaki tworzą flagę tego kraju. Na przykład kombinacja US tworzy flagę Stanów Zjednoczonych, GB tworzy flagę Wielkiej Brytanii, AR tworzy flagę Argentyny, a IN tworzy flagę Indii.

#root\_content\_flex\_items\_position\_position-par\_image\_1177217975 { width:{Long}403px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1177217975 { width:{Long}403px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1177217975 { width:{Long}403px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MDMiIGhlaWdodD0iMTA2Ij48cmVjdCB3aWR0aD0iNDAzIiBoZWlnaHQ9IjEwNiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Łączenie glifów")

Łącząc glify, można tworzyć flagi państw 

#root\_content\_flex\_items\_position\_position-par\_image\_1177217975 { width:{Long}403px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1177217975 { width:{Long}403px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1177217975 { width:{Long}403px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MDMiIGhlaWdodD0iMTA2Ij48cmVjdCB3aWR0aD0iNDAzIiBoZWlnaHQ9IjEwNiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Łączenie glifów")

Łącząc glify, można tworzyć flagi państw 

 Tworzenie wariantów znaków 

Domyślne znaki pojedynczych postaci, zwykle w kolorach żółtym, niebieskim lub szarym, oraz części ciała można łączyć z dowolnym dostępnym kolorem skóry. Oryginalny kolor skóry postaci w domyślnym znaku zostanie zmieniony na wybrany kolor. Takie kompozyty nie działają zazwyczaj w przypadku glifów, które zawierają z więcej niż jedną postać.

#root\_content\_flex\_items\_position\_position-par\_multi\_column > .dexter-FlexContainer-Items { min-height: 50px; } #root\_content\_flex\_items\_position\_position-par\_multi\_column > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 50%; max-width: 50%; min-height: auto; order: 0;} #root\_content\_flex\_items\_position\_position-par\_multi\_column > .dexter-FlexContainer-Items > \*:nth-child(2) { width: 50%; max-width: 50%; min-height: auto; order: 1;} 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1 { background-color: #FFFFFF; } 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1\_position-par\_image { width:{Long}358px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1\_position-par\_image { width:{Long}358px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1\_position-par\_image { width:{Long}358px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNTgiIGhlaWdodD0iNjgiPjxyZWN0IHdpZHRoPSIzNTgiIGhlaWdodD0iNjgiIGZpbGwtb3BhY2l0eT0iMCIgLz48L3N2Zz4= "Postaci z ustalonym kolorem skóry")

Postaci z ustalonym kolorem skóry 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1\_position-par\_image { width:{Long}358px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1\_position-par\_image { width:{Long}358px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c1\_position-par\_image { width:{Long}358px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNTgiIGhlaWdodD0iNjgiPjxyZWN0IHdpZHRoPSIzNTgiIGhlaWdodD0iNjgiIGZpbGwtb3BhY2l0eT0iMCIgLz48L3N2Zz4= "Postaci z ustalonym kolorem skóry")

Postaci z ustalonym kolorem skóry 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2 { background-color: #FFFFFF; } 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2\_position-par\_image { width:{Long}403px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2\_position-par\_image { width:{Long}403px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2\_position-par\_image { width:{Long}403px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MDMiIGhlaWdodD0iMTA2Ij48cmVjdCB3aWR0aD0iNDAzIiBoZWlnaHQ9IjEwNiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Łączenie znaków pojedynczych postaci z kolorami skóry")

Łączenie znaków pojedynczych postaci z kolorami skóry 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2\_position-par\_image { width:{Long}403px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2\_position-par\_image { width:{Long}403px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_items\_col-50-50-c2\_position-par\_image { width:{Long}403px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MDMiIGhlaWdodD0iMTA2Ij48cmVjdCB3aWR0aD0iNDAzIiBoZWlnaHQ9IjEwNiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Łączenie znaków pojedynczych postaci z kolorami skóry")

Łączenie znaków pojedynczych postaci z kolorami skóry 

_**Uwagi:**_

*   _Możliwe jest utworzenie tylko jednego powiązania między znakami emoji przedstawiającymi pojedyncze osoby lub części ciała a dowolnym znakiem reprezentującym kolor skóry._
*   _Glify kompozytowe są funkcją czcionki. Nie wszystkie czcionki SVG OpenType umożliwiają łączenie postaci w celu tworzenia glifów kompozytowych._
*   _Niektóre kompozytowe znaki EmojiOne można rozdzielić na ich elementy składowe._

## Czcionki zmienne

![](/content/dam/help/en/illustrator/using/fonts/Weight_2.gif)

![](/content/dam/help/en/illustrator/using/fonts/width_2.gif)

![](/content/dam/help/en/illustrator/using/slant.gif)

#root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table { width: 100%; } #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > th, #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > td { border: 1px solid #bdbdbd; } #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > th:before { border: 1px solid #bdbdbd; left: -1px; top: -1px; } #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > th.row-r0, #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > td.row-r0 { } #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > th.column-c0, #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > td.column-c0 { vertical-align: top; } #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > th.column-c1, #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > td.column-c1 { vertical-align: top; } #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > th.column-c2, #root\_content\_flex\_items\_position\_position-par\_table\_1999642253\_cop > .dexter-Table > tbody > tr > td.column-c2 { vertical-align: top; } 

Program InDesign obsługuje teraz **czcionki o zmiennych parametrach** — nowy format czcionek OpenType umożliwiający stosowanie własnych atrybutów, takich jak grubość, szerokość, pochylenie, rozmiar optyczny itd. Atrybuty niestandardowe można zmieniać za pomocą wygodnych opcji suwaka, dostępnych po kliknięciu na panelu sterowania, panelu Typografia, panelu Właściwości, panelu Style znaków i panelu Style akapitu.

Obok nazwy takiej czcionki znajduje się ikona identyfikująca ją jako czcionkę o zmiennych parametrach.

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456 > .dexter-FlexContainer-Items { min-height: 50px; } #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 50%; max-width: 50%; min-height: auto; order: 0;} #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456 > .dexter-FlexContainer-Items > \*:nth-child(2) { width: 50%; max-width: 50%; min-height: auto; order: 1;} 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1 { background-color: #FFFFFF; } 

**Dostęp do czcionek zmiennych na panelu Typografia**

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1\_position-par\_image { width:{Long}378px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1\_position-par\_image { width:{Long}378px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1\_position-par\_image { width:{Long}378px; } } 

![Panel Typografia](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNzgiIGhlaWdodD0iMzkwIj48cmVjdCB3aWR0aD0iMzc4IiBoZWlnaHQ9IjM5MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Id_character_panel")

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1\_position-par\_image { width:{Long}378px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1\_position-par\_image { width:{Long}378px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c1\_position-par\_image { width:{Long}378px; } } 

![Panel Typografia](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNzgiIGhlaWdodD0iMzkwIj48cmVjdCB3aWR0aD0iMzc4IiBoZWlnaHQ9IjM5MCIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Id_character_panel")

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2 { background-color: #FFFFFF; } 

**Dodawanie czcionek o zmiennych parametrach do stylu znaków**

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2\_position-par\_image { width:{Long}941px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2\_position-par\_image { width:{Long}941px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2\_position-par\_image { width:{Long}941px; } } 

    ![Panel Styl znaków](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI5NDEiIGhlaWdodD0iNTQ2Ij48cmVjdCB3aWR0aD0iOTQxIiBoZWlnaHQ9IjU0NiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Id_char_style") 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2\_position-par\_image { width:{Long}941px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2\_position-par\_image { width:{Long}941px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_1464456\_items\_col-50-50-c2\_position-par\_image { width:{Long}941px; } } 

    ![Panel Styl znaków](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI5NDEiIGhlaWdodD0iNTQ2Ij48cmVjdCB3aWR0aD0iOTQxIiBoZWlnaHQ9IjU0NiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Id_char_style") 

## Automatyczne dostosowywanie rozmiaru optycznego czcionek zmiennych do rozmiaru czcionki

Rozmiar optyczny czcionek zmiennych można automatycznie dopasować do rozmiaru czcionki.  

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392 > .dexter-FlexContainer-Items { min-height: 50px; } #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 50%; max-width: 50%; min-height: auto; order: 0;} #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392 > .dexter-FlexContainer-Items > \*:nth-child(2) { width: 50%; max-width: 50%; min-height: auto; order: 1;} 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1 { background-color: #FFFFFF; } 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1\_position-par\_image { width:{Long}1280px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1\_position-par\_image { width:{Long}1280px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1\_position-par\_image { width:{Long}1280px; } } 

    ![dostosowywanie rozmiaru optycznego](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMjgwIiBoZWlnaHQ9Ijc4MiI+PHJlY3Qgd2lkdGg9IjEyODAiIGhlaWdodD0iNzgyIiBmaWxsLW9wYWNpdHk9IjAiIC8+PC9zdmc+ "Dostosowywanie rozmiaru optycznego") 

Dostęp do opcji dostosowywania rozmiaru optycznego 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1\_position-par\_image { width:{Long}1280px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1\_position-par\_image { width:{Long}1280px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c1\_position-par\_image { width:{Long}1280px; } } 

    ![dostosowywanie rozmiaru optycznego](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMjgwIiBoZWlnaHQ9Ijc4MiI+PHJlY3Qgd2lkdGg9IjEyODAiIGhlaWdodD0iNzgyIiBmaWxsLW9wYWNpdHk9IjAiIC8+PC9zdmc+ "Dostosowywanie rozmiaru optycznego") 

Dostęp do opcji dostosowywania rozmiaru optycznego 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2 { background-color: #FFFFFF; } 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2\_position-par\_image { width:{Long}1280px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2\_position-par\_image { width:{Long}1280px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2\_position-par\_image { width:{Long}1280px; } } 

    ![Preferencje](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMjgwIiBoZWlnaHQ9IjExNDIiPjxyZWN0IHdpZHRoPSIxMjgwIiBoZWlnaHQ9IjExNDIiIGZpbGwtb3BhY2l0eT0iMCIgLz48L3N2Zz4= "Preferencje") 

Dostosowywanie rozmiaru optycznego można włączyć lub wyłączyć w oknie Preferencje > Tekst 

#root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2\_position-par\_image { width:{Long}1280px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2\_position-par\_image { width:{Long}1280px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_multi\_column\_423727392\_items\_col-50-50-c2\_position-par\_image { width:{Long}1280px; } } 

    ![Preferencje](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMjgwIiBoZWlnaHQ9IjExNDIiPjxyZWN0IHdpZHRoPSIxMjgwIiBoZWlnaHQ9IjExNDIiIGZpbGwtb3BhY2l0eT0iMCIgLz48L3N2Zz4= "Preferencje") 

Dostosowywanie rozmiaru optycznego można włączyć lub wyłączyć w oknie Preferencje > Tekst 

Aby zaznaczyć opcję Dostosuj rozmiar optyczny czcionek zmiennych do rozmiaru czcionki**,** wykonaj następujące czynności**:**

*   macOS: Otwórz ekran InDesign **>** Preferencje **>** Tekst.
*   Windows: Otwórz ekran Edycja **>** Preferencje **>** Tekst.

  Uwaga:

Po wybraniu tej opcji rozmiar optyczny będzie się zmieniać się wraz z rozmiarem czcionki. Jeśli opcja nie jest zaznaczona, zmiana rozmiaru czcionki nie wpłynie na rozmiar optyczny.  

#root\_content\_flex\_items\_position\_position-par\_image\_182019483 { width:{Long}1082px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_182019483 { width:{Long}1082px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_182019483 { width:{Long}1082px; } } 

![Dostosowywanie](/content/dam/help/en/indesign/using/using-fonts/jcr_content/main-pars/image_182019483/Mapping-Optical-Size-to-automatically-set-the-Font-Size.gif "Automatyczne dostosowywanie rozmiaru optycznego czcionek zmiennych do rozmiaru czcionki")

Dostosowywanie rozmiaru optycznego do rozmiaru czcionki. 

#root\_content\_flex\_items\_position\_position-par\_image\_182019483 { width:{Long}1082px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_182019483 { width:{Long}1082px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_182019483 { width:{Long}1082px; } } 

![Dostosowywanie](/content/dam/help/en/indesign/using/using-fonts/jcr_content/main-pars/image_182019483/Mapping-Optical-Size-to-automatically-set-the-Font-Size.gif "Automatyczne dostosowywanie rozmiaru optycznego czcionek zmiennych do rozmiaru czcionki")

Dostosowywanie rozmiaru optycznego do rozmiaru czcionki. 

Użyj suwaków, aby dostosować grubość, rozmiar optyczny i inne parametry czcionek zmiennych, takich jak Minion Variable Concept.

## Aktywny podgląd czcionki

Możesz wybrać tekst w dokumencie, aby wyświetlić podgląd czcionek w czasie rzeczywistym. Aby wyświetlić podgląd wybranego tekstu, należy zatrzymać wskaźnik nad nazwą czcionki na liście dostępnej na panelu Sterowanie, Typografia lub Właściwości.

## Podgląd stylu czcionki w czasie rzeczywistym

Aby wyświetlić podgląd stylu czcionki w czasie rzeczywistym, rozwiń rodzinę czcionek w menu czcionki i zatrzymaj kursor myszy nad wybranym stylem.

Aby wyłączyć opcje podglądu, wykonaj następujące czynności:

*   Wybierz polecenie Edycja > Preferencje.
*   W preferencjach tekstu usuń zaznaczenie opcji Włącz podgląd czcionek w menu.

Aby zmienić rozmiar czcionki tekstu zaznaczonego lub przykładowego, wyświetlając podgląd w czasie rzeczywistym, użyj opcji Pokaż mniejszy rozmiar tekstu przykładowego, Pokaż domyślny rozmiar tekstu przykładowego i Pokaż większy rozmiar tekstu przykładowego.

## Porządkowanie, wyszukiwanie i filtrowanie czcionek

Aby ułatwić sobie znajdowanie często używanych czcionek, można oznaczyć je jako ulubione lub skorzystać z sekcji niedawno używanych czcionek (jest ona wyświetlana na początku listy czcionek). Ostatnio używane czcionki i czcionki oznaczone jako ulubione są zapamiętywane pomiędzy sesjami programu InDesign.

#root\_content\_flex\_items\_position\_position-par\_image\_1692608780 { width:{Long}350px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1692608780 { width:{Long}350px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1692608780 { width:{Long}350px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNTAiIGhlaWdodD0iNDA5Ij48cmVjdCB3aWR0aD0iMzUwIiBoZWlnaHQ9IjQwOSIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "organizowanie czcionek")

**A.** Niedawno używane czcionki **B.** Czcionki oznaczone gwiazdką jako ulubione  

#root\_content\_flex\_items\_position\_position-par\_image\_1692608780 { width:{Long}350px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_1692608780 { width:{Long}350px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_1692608780 { width:{Long}350px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzNTAiIGhlaWdodD0iNDA5Ij48cmVjdCB3aWR0aD0iMzUwIiBoZWlnaHQ9IjQwOSIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "organizowanie czcionek")

Podczas wyszukiwania czcionek można ograniczyć wyniki, filtrując według klasyfikacji, na przykład czcionek szeryfowych, bezszeryfowych lub odręcznych. Można też wybrać, czy wyszukiwanie ma dotyczyć czcionek zainstalowanych na komputerze, czy też aktywowanych z usługi Adobe Fonts.

Można także wyszukiwać czcionki na podstawie podobieństwa wizualnego. Czcionki przypominające wyglądem wyszukiwaną czcionkę umieszczane są na górze listy wyników wyszukiwania. Na pasku stanu w menu czcionek wyświetlane są informacje na temat zastosowanych filtrów. 

## Narzędzia do wyszukiwania czcionek

#root\_content\_flex\_items\_position\_position-par\_image\_289533975 { width:{Long}490px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_289533975 { width:{Long}490px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_289533975 { width:{Long}490px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0OTAiIGhlaWdodD0iMTYyIj48cmVjdCB3aWR0aD0iNDkwIiBoZWlnaHQ9IjE2MiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "wyszukiwanie czcionek")

**A.** Pokaż czcionki według klasyfikacji **B.** Pokaż ulubione czcionki **C.** Pokaż ostatnio dodane **D.** Pokaż aktywne czcionki  

#root\_content\_flex\_items\_position\_position-par\_image\_289533975 { width:{Long}490px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_image\_289533975 { width:{Long}490px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_image\_289533975 { width:{Long}490px; } } 

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0OTAiIGhlaWdodD0iMTYyIj48cmVjdCB3aWR0aD0iNDkwIiBoZWlnaHQ9IjE2MiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "wyszukiwanie czcionek")

 Pokaż czcionki według klasyfikacji 

Listę czcionek można filtrować według klasyfikacji, na przykład: szeryfowe, skryptowe i odręczne.  

 Pokaż ulubione czcionki 

Wyświetla tylko czcionki oznaczone gwiazdkami jako ulubione.  

 Pokaż ostatnio dodane 

Wyświetla czcionki ostatnio dodane do listy czcionek.  

 Pokaż aktywne czcionki 

Na liście czcionek są wyświetlane tylko czcionki aktywowane z usługi Adobe Fonts.

## Aktywacja większej liczby czcionek

Wewnątrz programu InDesign możesz przeglądać tysiące czcionek od setek dostawców, błyskawicznie je aktywować i używać ich w dokumencie. Aktywowane czcionki są dostępne do użytku we wszystkich aplikacjach Creative Cloud.

1.  Na panelu Typografia kliknij kartę Znajdź więcej.
    
    
    
    
    
    
    
    
2.  Wybierz odpowiednią czcionkę z listy.
    
      Uwaga:
    
    Aby uzyskać podgląd czcionki w czasie rzeczywistym w zaznaczonym tekście, zatrzymaj wskaźnik myszy na nazwie czcionki.
    
    
    
    
    
    
    
    
    
    
3.  Kliknij ikonę Aktywuj wyświetlaną obok czcionki. Na ikonie Aktywuj wyświetlany jest symbol zaznaczenia, jeśli czcionka jest aktywna i dostępna do użycia.
    
    #root\_content\_flex\_items\_position\_position-par\_procedure\_151527447\_proc\_par\_step\_2\_step\_par\_image { width:{Long}552px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_procedure\_151527447\_proc\_par\_step\_2\_step\_par\_image { width:{Long}552px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_procedure\_151527447\_proc\_par\_step\_2\_step\_par\_image { width:{Long}552px; } }
    
    ![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1NTIiIGhlaWdodD0iMjUyIj48cmVjdCB3aWR0aD0iNTUyIiBoZWlnaHQ9IjI1MiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Aktywacja większej liczby czcionek")
    
    **A.** Filtr Aktywne czcionki **B.** Ikona Aktywuj czcionkę **C.** Ikona Dezaktywuj czcionkę **D.** Ikona Aktywacja w toku
    
    #root\_content\_flex\_items\_position\_position-par\_procedure\_151527447\_proc\_par\_step\_2\_step\_par\_image { width:{Long}552px; } @media (min-width: 600px) { #root\_content\_flex\_items\_position\_position-par\_procedure\_151527447\_proc\_par\_step\_2\_step\_par\_image { width:{Long}552px; } } @media (min-width: 1200px) { #root\_content\_flex\_items\_position\_position-par\_procedure\_151527447\_proc\_par\_step\_2\_step\_par\_image { width:{Long}552px; } }
    
    ![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1NTIiIGhlaWdodD0iMjUyIj48cmVjdCB3aWR0aD0iNTUyIiBoZWlnaHQ9IjI1MiIgZmlsbC1vcGFjaXR5PSIwIiAvPjwvc3ZnPg== "Aktywacja większej liczby czcionek")
    
    Więcej informacji na temat usługi Adobe Fonts podano na stronie [fonts.adobe.com](https://fonts.adobe.com/).
    
    
    
    
    
    
    

## Podgląd czcionek japońskich

Na karcie Znajdź więcej można przeglądać wszystkie czcionki japońskie dostępne na stronie [fonts.adobe.com](https://fonts.adobe.com/) i wyświetlać ich podgląd. Wykonaj następujące czynności, aby wyświetlić podgląd czcionek japońskich:

1.  Wybierz polecenia Edycja > Preferencje > Tekst.
    
    
    
    
    
    
    
2.  Zaznacz opcję Włącz podgląd czcionek japońskich w menu „Znajdź więcej”.
    
    
    
    
    
    
    

Zmiany zostaną zastosowane po ponownym uruchomieniu programu InDesign.

## Stosowanie czcionek do tekstu

Określając czcionkę, można wybrać niezależnie rodzinę czcionek i styl czcionki. Po zmianie jednej rodziny czcionki na drugą program InDesign stara się dopasować bieżący styl do stylu dostępnego w nowej rodzinie czcionek. Na przykład przy zmianie rodziny z Arial na Times, styl Arial Bold zostałby zamieniony na Times Bold. 

Gdy nadaje się tekstowi styl pogrubienia lub kursywy, program InDesign stosuje krój podany przez czcionkę. W większości przypadków, zgodnie z oczekiwaniami stosowana jest konkretna wersja pogrubienia lub kursywy. Niektóre czcionki mogą jednak stosować warianty pogrubione lub w kursywie, które nie są wyraźnie opisane jako pogrubienie lub kursywa. Na przykład, niektórzy producenci czcionek określają, że przy nadawaniu czcionce atrybutu pogrubienia stosowany jest wariant półgruby. 

1.  Zaznacz obiekt lub tekst, który ma zostać zmieniony.
    
    
    
    
    
2.  Wykonaj dowolną z następujących czynności:
    
    *   Przejdź do panelu Typografia, Sterowanie lub Właściwości, zaznacz czcionkę w menu Rodzina czcionek lub styl z menu Styl czcionek. (W systemie Mac OS menu Czcionka zawiera podmenu umożliwiające wybranie stylów tekstu).
    
    *   Kliknij początek nazwy rodziny czcionek lub nazwy stylu (albo dwukrotnie kliknij pierwszy człon nazwy) na panelu Typografia, Sterowanie lub Właściwości i wpisz kilka pierwszych liter wybranej nazwy. Podczas wprowadzania tekstu program InDesign wyświetla nazwę rodziny lub stylu czcionki, które odpowiadają wpisanym znakom.
    
    *   Wybierz czcionkę z menu Tekst \> Czcionka. Należy zauważyć, że za pomocą tego menu można wybrać zarówno rodzinę, jak i styl czcionki.
    
    
    
    
    
    
    
    

## Określanie rozmiaru kroju pisma

Domyślnie, rozmiar kroju pisma mierzony jest w _punktach_ (1 punkt wynosi 1/72 cala). Można podać dowolny rozmiar czcionki od 0,1 do 1296 punktów, w skokach co 0,001 punktu.

  Uwaga:

w programie Fireworks rozmiar tekstu jest domyślnie mierzony w pikselach.

1.  Zaznacz znaki lub obiekty tekstowe, które mają zostać zmienione. Jeśli nie zostanie zaznaczony żaden tekst, czcionka będzie stosowana do każdego nowego tekstu.
    
    
    
    
    
2.  Wykonaj jedną z następujących czynności:
    
    *   Określ opcję Rozmiar czcionki w panelu sterowania lub w panelu Typografia.
    
    *   Wybierz rozmiar z menu Tekst \> Rozmiar. Wybranie opcji Inne pozwala wpisać nową wartość rozmiaru na panelu Typografia.
    
    
      Uwaga:
    
    W oknie dialogowym Preferencje można zmienić jednostki miary tekstu. Opcja ta nie jest dostępna w programie Fireworks.
    
    
    
    
    
    
    
    
    

Po zaznaczeniu tekstu używającego brakującej czcionki panel **Typografia** lub **Sterowanie** wskazuje, że czcionki tej nie znaleziono, wyświetlając ją w nawiasach w menu stylu czcionki.

Program InDesign [automatycznie aktywuje brakujące czcionki](#auto-activate-missing-fonts) w dokumencie, używając czcionki dostępnej w usłudze Adobe Fonts. Jeśli w usłudze Adobe Fonts nie ma odpowiednich czcionek, zostaną one zastąpione czcionkami domyślnymi. W takiej sytuacji można zaznaczyć tekst i zastosować dowolną inną dostępną czcionkę. Brakujące czcionki, które zostały zastąpione substytutami, będą wyświetlane na górze menu **Tekst > Czcionka** w sekcji oznaczonej jako **Brakujące czcionki**. Domyślnie tekst sformatowany z użyciem brakującej czcionki zostanie wyróżniony na różowo.

Jeżeli zainstalowana jest czcionka TrueType, a dokument zawiera czcionkę Type 1 (T1) (czyli pewną wersję czcionki TrueType), to program oznacza tę czcionkę jako brakującą.

Aby odnaleźć i zmienić brakujące czcionki, można wybrać polecenie Tekst \> Znajdź czcionkę. Jeśli brakująca czcionka jest częścią stylu, to można zmienić definicję tego stylu, uaktualniając wybraną czcionkę.

Wyświetlane w programie InDesign okno dialogowe brakujących czcionek informuje, czy usługa Adobe Fonts w aplikacji Creative Cloud jest włączona. Jeśli funkcja ta jest wyłączona, to można także wybrać opcję włączenia usługi Adobe Fonts, dostępną w oknie dialogowym brakujących czcionek.

## Udostępnianie brakujących czcionek

Wykonaj dowolną z następujących czynności: 

*   Aktywuj brakujące czcionki z usługi Adobe Fonts. Więcej informacji: [Dodawanie czcionek z usługi Adobe Fonts](/pl/creative-cloud/help/add-fonts.html)[](http://help.typekit.com/customer/portal/articles/1145956-how-to-sync-fonts-to-your-desktop).
*   Zainstaluj brakujące czcionki w systemie.
*   Umieść brakujące czcionki w folderze Czcionki znajdującym się w folderze aplikacji InDesign. Czcionki z tego folderu są dostępne tylko w programie InDesign. Zobacz [Instalacja czcionek](using-fonts.html#installing_fonts).
*   Uaktywnić brakujące czcionki za pomocą aplikacji do zarządzania czcionkami.

## Wyróżnianie podstawionych czcionek w dokumencie

Gdy w preferencjach zaznaczona jest opcja **Podmienione czcionki**, tekst sformatowany z użyciem brakującej czcionki jest wyróżniony na różowo. Pozwala to szybko odszukać taki tekst.

1.  Wybierz polecenie Edycja \> Preferencje \> Skład (Windows®) lub InDesign \> Preferencje \> Skład (Mac OS®).
    
    
    
    
    
    
    
2.  Zaznacz opcję Czcionka podmieniona, a następnie kliknij przycisk OK.
    
    
    
    
    
    
    

## Czcionki zainstalowane w dokumentach

Czcionki w folderze Czcionki dokumentu, które znajdują się w tym samym miejscu co dokument programu InDesign, są tymczasowo instalowane po jego otworzeniu. Polecenie Pakiet może wygenerować folder Czcionki dokumentu, jeśli użytkownik chce udostępnić swój dokument lub przenieść go do innego komputera. (Przed udostępnieniem jakichkolwiek czcionek dokumentu upewnij się, że pozwala na to licencja oprogramowania czcionki). Czcionki aktywowane przy pomocy Adobe Fonts nie są kopiowane poleceniem Pakiet.

Czcionki w folderze Document Fonts różnią się od czcionek dostępnych z poziomu standardowej lokalizacji czcionek systemu operacyjnego. Są one instalowane w momencie otwierania dokumentu i zastępują każdą czcionkę o tej samej nazwie postscriptowej. Zastępują one jednak tylko czcionki w danym dokumencie. Czcionki zainstalowane przez jeden dokument nie są dostępne dla innych dokumentów. Gdy zamykasz dany dokument, zainstalowane dla niego czcionki zostają odinstalowane. Czcionki zainstalowane w dokumentach wymienione są w podmenu menu Czcionka.

Niektóre czcionki Type1 nie są dostępne w dokumencie. Ponadto czcionki systemu Mac OS nie są dostępne, gdy program InDesign pracuje w systemie Windows.

* * *

### zasoby powiązane

*   [Wstawianie glifów i znaków specjalnych](/pl/indesign/using/glyphs-special-characters.html#insert_glyphs_and_special_characters)
*   [Wyszukiwanie i zastępowanie czcionek](/pl/indesign/using/find-change.html#find_and_change_fonts)
*   [Pakiety plików](/pl/indesign/using/preflighting-files-handoff.html#package_files)
*   [Licencjonowanie czcionek](/pl/fonts/using/font-licensing.html)

### Skontaktuj się z nami

#root\_content\_flex\_items\_position\_position-par\_imageandtext\_copy\_co > .dexter-FlexContainer-Items { min-height: 50px; margin: -4px; } #root\_content\_flex\_items\_position\_position-par\_imageandtext\_copy\_co > .dexter-FlexContainer-Items > \* { border: 0 solid transparent; border-width: 4px;} #root\_content\_flex\_items\_position\_position-par\_imageandtext\_copy\_co > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 50px; max-width: 50px; min-height: auto; } 

#root\_content\_flex\_items\_position\_position-par\_imageandtext\_copy\_co\_items\_imageandtextimage { width:50px; } 

![](/content/dam/help/pl/xd/help/get-started-with-artboards-in-XD/jcr_content/main-pars/imageandtext/imageandtextimage/ask-the-community.svg)

#root\_content\_flex\_items\_position\_position-par\_imageandtext\_copy\_co\_items\_imageandtextimage { width:50px; } 

![](/content/dam/help/pl/xd/help/get-started-with-artboards-in-XD/jcr_content/main-pars/imageandtext/imageandtextimage/ask-the-community.svg)

Zachęcamy do dzielenia się opiniami. Podziel się swoimi opiniami na [forum użytkowników programu Adobe InDesign](https://community.adobe.com/t5/indesign/ct-p/ct-indesign).   

#id-5388d41983e5aba670c30a46691cd8ad { background-image: url(https:\\2f\\2fhelpx-prod.scene7.com\\2fis\\2fimage\\2fHelpxProdLoc\\2fsign-in-card-bg%20copy?$png$\\26jpegSize=100\\26wid=272); background-size: cover; background-position: 50% 50%; } @media screen and (min-width: 600px) { #id-5388d41983e5aba670c30a46691cd8ad { background-image: url(https:\\2f\\2fhelpx-prod.scene7.com\\2fis\\2fimage\\2fHelpxProdLoc\\2fsign-in-card-bg%20copy?$png$\\26jpegSize=100\\26wid=272); background-size: cover; background-position: 50% 50%; } } @media screen and (min-width: 1200px) { #id-5388d41983e5aba670c30a46691cd8ad { background-image: url(https:\\2f\\2fhelpx-prod.scene7.com\\2fis\\2fimage\\2fHelpxProdLoc\\2fsign-in-card-bg%20copy?$png$\\26jpegSize=100\\26wid=272); background-size: cover; background-position: 50% 50%; } } #id-5388d41983e5aba670c30a46691cd8ad { color: #2C2C2C; } #id-5388d41983e5aba670c30a46691cd8ad { border-radius: 5px; } 

#id-62603173498c77f77ead25e0ef303042 { height: 8px; } 

![](/content/dam/helpx/icons/adobe-logo.svg)

## **Pomoc dostępna szybciej i łatwiej**

#id-0c96ebf0f47ceb70a053e09d5abb4fab { height: 20px; } 

 [Zaloguj się](#) 

#id-eb51f74e3d9193225597f35af799ef17 { height: 40px; } 

Nowy użytkownik?

 [Utwórz konto ›](#) 

#id-51e1a7f06b592e731e9b6611dba4715f { height: 20px; } 

![Avatar]()

[Zarządzaj kontem](#)

Podręczne łącza

[Wyświetl swoje plany](#)[Zarządzaj planami](#)

 [Wyświetl podręczne łącza](#) 

 [Ukryj podręczne łącza](#) 

[Informacje prawne](/pl/legal/legal-notices.html)    |    [Zasady ochrony prywatności online](https://www.adobe.com/pl/privacy.html)

#id-3ce8b01d8545269ea807b036bb770cf2 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; } @media screen and (min-width: 600px) { #id-3ce8b01d8545269ea807b036bb770cf2 > .dexter-FlexContainer-Items { } #id-3ce8b01d8545269ea807b036bb770cf2 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 58.333333%; max-width: 58.333333%; flex: 1 1 auto; min-height: auto; } } @media screen and (min-width: 1200px) { #id-3ce8b01d8545269ea807b036bb770cf2 > .dexter-FlexContainer-Items { } #id-3ce8b01d8545269ea807b036bb770cf2 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 750px; max-width: 750px; min-height: auto; } } 

#id-26c90bdd0c7efec21f5be51af1bc6fe2 { background-color: #F5F5F5; } #id-26c90bdd0c7efec21f5be51af1bc6fe2 { border: solid 2px #EAEAEA; border-radius: 8px; } 

#id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 100px; max-width: 100px; min-height: auto; } #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items > \*:nth-child(2) { flex: 1 1 1%; max-width: 100%; min-height: auto; } @media screen and (min-width: 600px) { #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items { } #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 100px; max-width: 100px; min-height: auto; } #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items > \*:nth-child(2) { flex: 1 1 1%; max-width: 100%; min-height: auto; } } @media screen and (min-width: 1200px) { #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items { } #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 100px; max-width: 100px; min-height: auto; } #id-2a99973529f5f79cf4c853b841b56682 > .dexter-FlexContainer-Items > \*:nth-child(2) { flex: 1 1 1%; max-width: 100%; min-height: auto; } } 

#id-ee7504b9b827d6c2decce148cf1337a3 { border-radius: 6px 0px 0px 6px; } 

#id-60638be97741c0636826f3d9d5ecd2bb { width:auto; } 

![Logo Adobe InDesign](/content/dam/help/mnemonics/id_cc_app_RGB.svg)

#id-60638be97741c0636826f3d9d5ecd2bb { width:auto; } 

![Logo Adobe InDesign](/content/dam/help/mnemonics/id_cc_app_RGB.svg)

#id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items { margin: -5px; } #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items > \* { border: 0 solid transparent; border-width: 5px;} #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; } #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items > \*:nth-child(2) { width: 100%; max-width: 100%; flex: 1 1 auto; min-height: auto; } #id-13a30140aa91c7a4f72bafaf9bbb084b { background-color: #FBFBFB; } @media screen and (min-width: 1200px) { #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items { margin: -5px; } #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items > \* { border: 0 solid transparent; border-width: 5px; } #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items > \*:nth-child(1) { width: 66.666667%; max-width: 66.666667%; flex: 1 1 auto; min-height: auto; } #id-13a30140aa91c7a4f72bafaf9bbb084b > .dexter-FlexContainer-Items > \*:nth-child(2) { flex: 1 1 1%; max-width: 100%; min-height: auto; } } 

**Tworzenie materiałów do druku i publikacji cyfrowej w programie InDesign**  
Projektowanie przyciągających uwagę reklam, układów czasopism i książek oraz innych materiałów.

   [Otwórz aplikację](https://www.adobe.com/pl/download/indesign?locale=pl) 

   [Otwórz aplikację](https://www.adobe.com/pl/download/indesign?locale=pl) 

#id-8ac494f3c694ae4e8eb8df1b1b0c688a > .dexter-FlexContainer-Items > \*:nth-child(1) { flex: 0 0 auto; max-width: 100%; width: auto; min-height: auto; } 

### Udostępnij tę stronę

*   [](http://www.facebook.com/sharer.php)
*   [](https://twitter.com/share?text=twitter)
*   [](http://www.linkedin.com/shareArticle?mini=true)
*   [Copied](#)

![Adobe InDesign](/content/dam/help/mnemonics/id_cc_app_RGB.svg)

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

Fachowe wsparcie w rozwiązywaniu problemów.

[Zacznij teraz](/pl/pl/contact.html?step=IDSN)

[^ Do góry](#)

Language Navigation

Language Navigation

[](#)

Wybierz region

Wybranie regionu spowoduje zmianę języka i/lub zawartości w witrynie Adobe.com.

*   Americas
    
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
*   Asia Pacific
    
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
*   Indonesia
*   Malaysia
*   Pilipinas
*   Việt Nam
*   भारत
*   한국
*   Singapore
*   Thailand - English
*   ประเทศไทย
*   Europe, Middle East and Africa
    
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
*   Svizzera
*   Türkiye
*   United Kingdom
*   България
*   Россия
*   Україна
*   الشرق الأوسط وشمال أفريقيا - اللغة العربية
*   ישראל - עברית
*   Sverige
*   Saudi Arabia - English
*   United Arab Emirates - English
*   الإمارات العربية المتحدة
*   المملكة العربية السعودية

 

var adobeid = { env: '//ims-na1.adobelogin.com', environment: 'prod', jumpToken: { api: '/ims/jumptoken/v1', }, client\_id: 'AdobeSupport1', scope: 'AdobeID,openid,gnav,creative\_cloud,read\_organizations,additional\_info.projectedProductContext,additional\_info.roles,pps.read,firefly\_api,account\_cluster.read', uses\_redirect\_mode: true, locale: 'pl\_PL', uses\_modal\_mode: false, autoValidateToken: true, api\_parameters: { authorize: { state: { ac: 'AdobeSupport1', } } }, redirect\_uri: window.location.url, onReady: function () { window.dispatchEvent(new Event('dexter:IMSReady')); } }; 

window.dexter.checkout = window.dexter.checkout || {}; window.dexter.checkout.ims = { clientId: { ucv2: 'unified\_checkout\_client', ucv3: 'unified\_checkout\_client\_v3' }, targetScope: { ucv2: 'AdobeID,openid,additional\_info.roles,additional\_info.vat\_id,additional\_info.dob,update\_profile.countryCode', ucv3: 'AdobeID,openid,additional\_info.roles,additional\_info.vat\_id,additional\_info.dob,update\_profile.countryCode, additional\_info.authenticatingAccount' }, timeout: '' } 

window.dexter.config = window.dexter.config || {}; window.dexter.config.lazyThreshold = '3000px 0px';