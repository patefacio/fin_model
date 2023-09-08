//! TODO: Document Module(html_tag)

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumeration of html tags
pub enum HtmlTag {
    /// The `a` element
    A,
    /// The `abbr` element
    Abbr,
    /// The `address` element
    Address,
    /// The `area` element
    Area,
    /// The `article` element
    Article,
    /// The `aside` element
    Aside,
    /// The `audio` element
    Audio,
    /// The `b` element
    B,
    /// The `base` element
    Base,
    /// The `bdi` element
    Bdi,
    /// The `bdo` element
    Bdo,
    /// The `blockquote` element
    Blockquote,
    /// The `body` element
    Body,
    /// The `br` element
    Br,
    /// The `button` element
    Button,
    /// The `canvas` element
    Canvas,
    /// The `caption` element
    Caption,
    /// The `cite` element
    Cite,
    /// The `code` element
    Code,
    /// The `col` element
    Col,
    /// The `colgroup` element
    Colgroup,
    /// The `data` element
    Data,
    /// The `datalist` element
    Datalist,
    /// The `dd` element
    Dd,
    /// The `del` element
    Del,
    /// The `details` element
    Details,
    /// The `dfn` element
    Dfn,
    /// The `dialog` element
    Dialog,
    /// The `div` element
    Div,
    /// The `dl` element
    Dl,
    /// The `dt` element
    Dt,
    /// The `em` element
    Em,
    /// The `embed` element
    Embed,
    /// The `fieldset` element
    Fieldset,
    /// The `figcaption` element
    Figcaption,
    /// The `figure` element
    Figure,
    /// The `footer` element
    Footer,
    /// The `form` element
    Form,
    /// The `h1` element
    H1,
    /// The `h2` element
    H2,
    /// The `h3` element
    H3,
    /// The `h4` element
    H4,
    /// The `h5` element
    H5,
    /// The `h6` element
    H6,
    /// The `head` element
    Head,
    /// The `header` element
    Header,
    /// The `hgroup` element
    Hgroup,
    /// The `hr` element
    Hr,
    /// The `html` element
    Html,
    /// The `i` element
    I,
    /// The `iframe` element
    Iframe,
    /// The `img` element
    Img,
    /// The `input` element
    Input,
    /// The `ins` element
    Ins,
    /// The `kbd` element
    Kbd,
    /// The `label` element
    Label,
    /// The `legend` element
    Legend,
    /// The `li` element
    Li,
    /// The `link` element
    Link,
    /// The `main` element
    Main,
    /// The `map` element
    Map,
    /// The `mark` element
    Mark,
    /// The `meta` element
    Meta,
    /// The `meter` element
    Meter,
    /// The `nav` element
    Nav,
    /// The `noscript` element
    Noscript,
    /// The `object` element
    Object,
    /// The `ol` element
    Ol,
    /// The `optgroup` element
    Optgroup,
    /// The `option` element
    Option,
    /// The `output` element
    Output,
    /// The `p` element
    P,
    /// The `param` element
    Param,
    /// The `picture` element
    Picture,
    /// The `pre` element
    Pre,
    /// The `progress` element
    Progress,
    /// The `q` element
    Q,
    /// The `rp` element
    Rp,
    /// The `rt` element
    Rt,
    /// The `ruby` element
    Ruby,
    /// The `s` element
    S,
    /// The `samp` element
    Samp,
    /// The `script` element
    Script,
    /// The `section` element
    Section,
    /// The `select` element
    Select,
    /// The `small` element
    Small,
    /// The `source` element
    Source,
    /// The `span` element
    Span,
    /// The `strong` element
    Strong,
    /// The `style` element
    Style,
    /// The `sub` element
    Sub,
    /// The `summary` element
    Summary,
    /// The `sup` element
    Sup,
    /// The `svg` element
    Svg,
    /// The `table` element
    Table,
    /// The `tbody` element
    Tbody,
    /// The `td` element
    Td,
    /// The `template` element
    Template,
    /// The `textarea` element
    Textarea,
    /// The `tfoot` element
    Tfoot,
    /// The `th` element
    Th,
    /// The `thead` element
    Thead,
    /// The `time` element
    Time,
    /// The `title` element
    Title,
    /// The `tr` element
    Tr,
    /// The `track` element
    Track,
    /// The `u` element
    U,
    /// The `ul` element
    Ul,
    /// The `var` element
    Var,
    /// The `video` element
    Video,
    /// The `wbr` element
    Wbr,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl HtmlTag {
    /// Get the tag name
    ///
    ///   * _return_ - The tag name in all caps.
    pub fn as_str(&self) -> &str {
        match self {
            HtmlTag::A => "A",
            HtmlTag::Abbr => "ABBR",
            HtmlTag::Address => "ADDRESS",
            HtmlTag::Area => "AREA",
            HtmlTag::Article => "ARTICLE",
            HtmlTag::Aside => "ASIDE",
            HtmlTag::Audio => "AUDIO",
            HtmlTag::B => "B",
            HtmlTag::Base => "BASE",
            HtmlTag::Bdi => "BDI",
            HtmlTag::Bdo => "BDO",
            HtmlTag::Blockquote => "BLOCKQUOTE",
            HtmlTag::Body => "BODY",
            HtmlTag::Br => "BR",
            HtmlTag::Button => "BUTTON",
            HtmlTag::Canvas => "CANVAS",
            HtmlTag::Caption => "CAPTION",
            HtmlTag::Cite => "CITE",
            HtmlTag::Code => "CODE",
            HtmlTag::Col => "COL",
            HtmlTag::Colgroup => "COLGROUP",
            HtmlTag::Data => "DATA",
            HtmlTag::Datalist => "DATALIST",
            HtmlTag::Dd => "DD",
            HtmlTag::Del => "DEL",
            HtmlTag::Details => "DETAILS",
            HtmlTag::Dfn => "DFN",
            HtmlTag::Dialog => "DIALOG",
            HtmlTag::Div => "DIV",
            HtmlTag::Dl => "DL",
            HtmlTag::Dt => "DT",
            HtmlTag::Em => "EM",
            HtmlTag::Embed => "EMBED",
            HtmlTag::Fieldset => "FIELDSET",
            HtmlTag::Figcaption => "FIGCAPTION",
            HtmlTag::Figure => "FIGURE",
            HtmlTag::Footer => "FOOTER",
            HtmlTag::Form => "FORM",
            HtmlTag::H1 => "H1",
            HtmlTag::H2 => "H2",
            HtmlTag::H3 => "H3",
            HtmlTag::H4 => "H4",
            HtmlTag::H5 => "H5",
            HtmlTag::H6 => "H6",
            HtmlTag::Head => "HEAD",
            HtmlTag::Header => "HEADER",
            HtmlTag::Hgroup => "HGROUP",
            HtmlTag::Hr => "HR",
            HtmlTag::Html => "HTML",
            HtmlTag::I => "I",
            HtmlTag::Iframe => "IFRAME",
            HtmlTag::Img => "IMG",
            HtmlTag::Input => "INPUT",
            HtmlTag::Ins => "INS",
            HtmlTag::Kbd => "KBD",
            HtmlTag::Label => "LABEL",
            HtmlTag::Legend => "LEGEND",
            HtmlTag::Li => "LI",
            HtmlTag::Link => "LINK",
            HtmlTag::Main => "MAIN",
            HtmlTag::Map => "MAP",
            HtmlTag::Mark => "MARK",
            HtmlTag::Meta => "META",
            HtmlTag::Meter => "METER",
            HtmlTag::Nav => "NAV",
            HtmlTag::Noscript => "NOSCRIPT",
            HtmlTag::Object => "OBJECT",
            HtmlTag::Ol => "OL",
            HtmlTag::Optgroup => "OPTGROUP",
            HtmlTag::Option => "OPTION",
            HtmlTag::Output => "OUTPUT",
            HtmlTag::P => "P",
            HtmlTag::Param => "PARAM",
            HtmlTag::Picture => "PICTURE",
            HtmlTag::Pre => "PRE",
            HtmlTag::Progress => "PROGRESS",
            HtmlTag::Q => "Q",
            HtmlTag::Rp => "RP",
            HtmlTag::Rt => "RT",
            HtmlTag::Ruby => "RUBY",
            HtmlTag::S => "S",
            HtmlTag::Samp => "SAMP",
            HtmlTag::Script => "SCRIPT",
            HtmlTag::Section => "SECTION",
            HtmlTag::Select => "SELECT",
            HtmlTag::Small => "SMALL",
            HtmlTag::Source => "SOURCE",
            HtmlTag::Span => "SPAN",
            HtmlTag::Strong => "STRONG",
            HtmlTag::Style => "STYLE",
            HtmlTag::Sub => "SUB",
            HtmlTag::Summary => "SUMMARY",
            HtmlTag::Sup => "SUP",
            HtmlTag::Svg => "SVG",
            HtmlTag::Table => "TABLE",
            HtmlTag::Tbody => "TBODY",
            HtmlTag::Td => "TD",
            HtmlTag::Template => "TEMPLATE",
            HtmlTag::Textarea => "TEXTAREA",
            HtmlTag::Tfoot => "TFOOT",
            HtmlTag::Th => "TH",
            HtmlTag::Thead => "THEAD",
            HtmlTag::Time => "TIME",
            HtmlTag::Title => "TITLE",
            HtmlTag::Tr => "TR",
            HtmlTag::Track => "TRACK",
            HtmlTag::U => "U",
            HtmlTag::Ul => "UL",
            HtmlTag::Var => "VAR",
            HtmlTag::Video => "VIDEO",
            HtmlTag::Wbr => "WBR",
        }
    }

    /// Get the enum value from the str
    ///
    ///   * **value** - The tag name in upper case
    ///   * _return_ - The resolved tag.
    pub fn from_str(value: &str) -> Option<HtmlTag> {
        match value {
            "A" => Some(HtmlTag::A),
            "ABBR" => Some(HtmlTag::Abbr),
            "ADDRESS" => Some(HtmlTag::Address),
            "AREA" => Some(HtmlTag::Area),
            "ARTICLE" => Some(HtmlTag::Article),
            "ASIDE" => Some(HtmlTag::Aside),
            "AUDIO" => Some(HtmlTag::Audio),
            "B" => Some(HtmlTag::B),
            "BASE" => Some(HtmlTag::Base),
            "BDI" => Some(HtmlTag::Bdi),
            "BDO" => Some(HtmlTag::Bdo),
            "BLOCKQUOTE" => Some(HtmlTag::Blockquote),
            "BODY" => Some(HtmlTag::Body),
            "BR" => Some(HtmlTag::Br),
            "BUTTON" => Some(HtmlTag::Button),
            "CANVAS" => Some(HtmlTag::Canvas),
            "CAPTION" => Some(HtmlTag::Caption),
            "CITE" => Some(HtmlTag::Cite),
            "CODE" => Some(HtmlTag::Code),
            "COL" => Some(HtmlTag::Col),
            "COLGROUP" => Some(HtmlTag::Colgroup),
            "DATA" => Some(HtmlTag::Data),
            "DATALIST" => Some(HtmlTag::Datalist),
            "DD" => Some(HtmlTag::Dd),
            "DEL" => Some(HtmlTag::Del),
            "DETAILS" => Some(HtmlTag::Details),
            "DFN" => Some(HtmlTag::Dfn),
            "DIALOG" => Some(HtmlTag::Dialog),
            "DIV" => Some(HtmlTag::Div),
            "DL" => Some(HtmlTag::Dl),
            "DT" => Some(HtmlTag::Dt),
            "EM" => Some(HtmlTag::Em),
            "EMBED" => Some(HtmlTag::Embed),
            "FIELDSET" => Some(HtmlTag::Fieldset),
            "FIGCAPTION" => Some(HtmlTag::Figcaption),
            "FIGURE" => Some(HtmlTag::Figure),
            "FOOTER" => Some(HtmlTag::Footer),
            "FORM" => Some(HtmlTag::Form),
            "H1" => Some(HtmlTag::H1),
            "H2" => Some(HtmlTag::H2),
            "H3" => Some(HtmlTag::H3),
            "H4" => Some(HtmlTag::H4),
            "H5" => Some(HtmlTag::H5),
            "H6" => Some(HtmlTag::H6),
            "HEAD" => Some(HtmlTag::Head),
            "HEADER" => Some(HtmlTag::Header),
            "HGROUP" => Some(HtmlTag::Hgroup),
            "HR" => Some(HtmlTag::Hr),
            "HTML" => Some(HtmlTag::Html),
            "I" => Some(HtmlTag::I),
            "IFRAME" => Some(HtmlTag::Iframe),
            "IMG" => Some(HtmlTag::Img),
            "INPUT" => Some(HtmlTag::Input),
            "INS" => Some(HtmlTag::Ins),
            "KBD" => Some(HtmlTag::Kbd),
            "LABEL" => Some(HtmlTag::Label),
            "LEGEND" => Some(HtmlTag::Legend),
            "LI" => Some(HtmlTag::Li),
            "LINK" => Some(HtmlTag::Link),
            "MAIN" => Some(HtmlTag::Main),
            "MAP" => Some(HtmlTag::Map),
            "MARK" => Some(HtmlTag::Mark),
            "META" => Some(HtmlTag::Meta),
            "METER" => Some(HtmlTag::Meter),
            "NAV" => Some(HtmlTag::Nav),
            "NOSCRIPT" => Some(HtmlTag::Noscript),
            "OBJECT" => Some(HtmlTag::Object),
            "OL" => Some(HtmlTag::Ol),
            "OPTGROUP" => Some(HtmlTag::Optgroup),
            "OPTION" => Some(HtmlTag::Option),
            "OUTPUT" => Some(HtmlTag::Output),
            "P" => Some(HtmlTag::P),
            "PARAM" => Some(HtmlTag::Param),
            "PICTURE" => Some(HtmlTag::Picture),
            "PRE" => Some(HtmlTag::Pre),
            "PROGRESS" => Some(HtmlTag::Progress),
            "Q" => Some(HtmlTag::Q),
            "RP" => Some(HtmlTag::Rp),
            "RT" => Some(HtmlTag::Rt),
            "RUBY" => Some(HtmlTag::Ruby),
            "S" => Some(HtmlTag::S),
            "SAMP" => Some(HtmlTag::Samp),
            "SCRIPT" => Some(HtmlTag::Script),
            "SECTION" => Some(HtmlTag::Section),
            "SELECT" => Some(HtmlTag::Select),
            "SMALL" => Some(HtmlTag::Small),
            "SOURCE" => Some(HtmlTag::Source),
            "SPAN" => Some(HtmlTag::Span),
            "STRONG" => Some(HtmlTag::Strong),
            "STYLE" => Some(HtmlTag::Style),
            "SUB" => Some(HtmlTag::Sub),
            "SUMMARY" => Some(HtmlTag::Summary),
            "SUP" => Some(HtmlTag::Sup),
            "SVG" => Some(HtmlTag::Svg),
            "TABLE" => Some(HtmlTag::Table),
            "TBODY" => Some(HtmlTag::Tbody),
            "TD" => Some(HtmlTag::Td),
            "TEMPLATE" => Some(HtmlTag::Template),
            "TEXTAREA" => Some(HtmlTag::Textarea),
            "TFOOT" => Some(HtmlTag::Tfoot),
            "TH" => Some(HtmlTag::Th),
            "THEAD" => Some(HtmlTag::Thead),
            "TIME" => Some(HtmlTag::Time),
            "TITLE" => Some(HtmlTag::Title),
            "TR" => Some(HtmlTag::Tr),
            "TRACK" => Some(HtmlTag::Track),
            "U" => Some(HtmlTag::U),
            "UL" => Some(HtmlTag::Ul),
            "VAR" => Some(HtmlTag::Var),
            "VIDEO" => Some(HtmlTag::Video),
            "WBR" => Some(HtmlTag::Wbr),
            _ => None,
        }
    }
}

// α <mod-def html_tag>
// ω <mod-def html_tag>
