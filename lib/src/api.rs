use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiObject {
    Status(Status),
    User(User),
    SearchResults(SearchResults),
    Deletion(Deletion),
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coordinates(pub Vec<f64>);
impl std::ops::Deref for Coordinates {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Count(pub u32);
impl std::ops::Deref for Count {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Deletion {
    pub delete: DeletionDelete,
}
impl Deletion {
    pub fn builder() -> builder::Deletion {
        builder::Deletion::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeletionDelete {
    pub status: DeletionDeleteStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_ms: Option<DeletionDeleteTimestampMs>,
}
impl DeletionDelete {
    pub fn builder() -> builder::DeletionDelete {
        builder::DeletionDelete::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeletionDeleteStatus {
    pub id: Id,
    pub id_str: IdStr,
    pub user_id: Id,
    pub user_id_str: IdStr,
}
impl DeletionDeleteStatus {
    pub fn builder() -> builder::DeletionDeleteStatus {
        builder::DeletionDeleteStatus::default()
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct DeletionDeleteTimestampMs(String);
impl std::ops::Deref for DeletionDeleteTimestampMs {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for DeletionDeleteTimestampMs {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^\\d+$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for DeletionDeleteTimestampMs {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for DeletionDeleteTimestampMs {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletionDeleteTimestampMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Entities {
    pub hashtags: Vec<Hashtag>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<Media>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<Symbol>,
    pub urls: Vec<Url>,
    pub user_mentions: Vec<UserMention>,
}
impl Entities {
    pub fn builder() -> builder::Entities {
        builder::Entities::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedEntities {
    pub media: Vec<Media>,
}
impl ExtendedEntities {
    pub fn builder() -> builder::ExtendedEntities {
        builder::ExtendedEntities::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweet {
    pub display_text_range: Range,
    pub entities: Entities,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_entities: Option<ExtendedEntities>,
    pub full_text: String,
}
impl ExtendedTweet {
    pub fn builder() -> builder::ExtendedTweet {
        builder::ExtendedTweet::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hashtag {
    pub indices: Range,
    pub text: String,
}
impl Hashtag {
    pub fn builder() -> builder::Hashtag {
        builder::Hashtag::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Id(pub u64);
impl std::ops::Deref for Id {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdNullable(pub Option<Id>);
impl std::ops::Deref for IdNullable {
    type Target = Option<Id>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct IdStr(String);
impl std::ops::Deref for IdStr {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for IdStr {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^\\d+$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for IdStr {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for IdStr {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdStrNullable(pub Option<IdStr>);
impl std::ops::Deref for IdStrNullable {
    type Target = Option<IdStr>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Media {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_media_info: Option<MediaAdditionalMediaInfo>,
    pub display_url: String,
    pub expanded_url: String,
    pub id: Id,
    pub id_str: IdStr,
    pub indices: Range,
    pub media_url: String,
    pub media_url_https: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sizes: Option<MediaSizes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_status_id: Option<Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_status_id_str: Option<IdStr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_user_id: Option<Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_user_id_str: Option<IdStr>,
    #[serde(rename = "type")]
    pub type_: MediaType,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_info: Option<VideoInfo>,
}
impl Media {
    pub fn builder() -> builder::Media {
        builder::Media::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaAdditionalMediaInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_to_actions: Option<MediaAdditionalMediaInfoCallToActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeddable: Option<bool>,
    pub monetizable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_user: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl MediaAdditionalMediaInfo {
    pub fn builder() -> builder::MediaAdditionalMediaInfo {
        builder::MediaAdditionalMediaInfo::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaAdditionalMediaInfoCallToActions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visit_site: Option<MediaAdditionalMediaInfoCallToActionsVisitSite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watch_now: Option<MediaAdditionalMediaInfoCallToActionsWatchNow>,
}
impl MediaAdditionalMediaInfoCallToActions {
    pub fn builder() -> builder::MediaAdditionalMediaInfoCallToActions {
        builder::MediaAdditionalMediaInfoCallToActions::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaAdditionalMediaInfoCallToActionsVisitSite {
    pub url: String,
}
impl MediaAdditionalMediaInfoCallToActionsVisitSite {
    pub fn builder() -> builder::MediaAdditionalMediaInfoCallToActionsVisitSite {
        builder::MediaAdditionalMediaInfoCallToActionsVisitSite::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaAdditionalMediaInfoCallToActionsWatchNow {
    pub url: String,
}
impl MediaAdditionalMediaInfoCallToActionsWatchNow {
    pub fn builder() -> builder::MediaAdditionalMediaInfoCallToActionsWatchNow {
        builder::MediaAdditionalMediaInfoCallToActionsWatchNow::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaSizes {
    pub large: Size,
    pub medium: Size,
    pub small: Size,
    pub thumb: Size,
}
impl MediaSizes {
    pub fn builder() -> builder::MediaSizes {
        builder::MediaSizes::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MediaType {
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "animated_gif")]
    AnimatedGif,
}
impl ToString for MediaType {
    fn to_string(&self) -> String {
        match *self {
            Self::Photo => "photo".to_string(),
            Self::Video => "video".to_string(),
            Self::AnimatedGif => "animated_gif".to_string(),
        }
    }
}
impl std::str::FromStr for MediaType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "photo" => Ok(Self::Photo),
            "video" => Ok(Self::Video),
            "animated_gif" => Ok(Self::AnimatedGif),
            _ => Err("invalid value"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Place {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    pub bounding_box: Option<Polygon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contained_within: Option<serde_json::Value>,
    pub country: String,
    pub country_code: String,
    pub full_name: String,
    pub id: String,
    pub name: String,
    pub place_type: String,
    pub url: String,
}
impl Place {
    pub fn builder() -> builder::Place {
        builder::Place::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub coordinates: Coordinates,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
impl Point {
    pub fn builder() -> builder::Point {
        builder::Point::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Polygon {
    pub coordinates: Vec<Vec<Coordinates>>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
impl Polygon {
    pub fn builder() -> builder::Polygon {
        builder::Polygon::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Range(pub Vec<u32>);
impl std::ops::Deref for Range {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResults {
    pub search_metadata: SearchResultsSearchMetadata,
    pub statuses: Vec<Status>,
}
impl SearchResults {
    pub fn builder() -> builder::SearchResults {
        builder::SearchResults::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResultsSearchMetadata {
    pub completed_in: f64,
    pub count: Count,
    pub max_id: Id,
    pub max_id_str: IdStr,
    pub next_results: String,
    pub query: String,
    pub refresh_url: String,
    pub since_id: Id,
    pub since_id_str: IdStr,
}
impl SearchResultsSearchMetadata {
    pub fn builder() -> builder::SearchResultsSearchMetadata {
        builder::SearchResultsSearchMetadata::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Size {
    pub h: Count,
    pub resize: SizeResize,
    pub w: Count,
}
impl Size {
    pub fn builder() -> builder::Size {
        builder::Size::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SizeResize {
    #[serde(rename = "crop")]
    Crop,
    #[serde(rename = "fit")]
    Fit,
}
impl ToString for SizeResize {
    fn to_string(&self) -> String {
        match *self {
            Self::Crop => "crop".to_string(),
            Self::Fit => "fit".to_string(),
        }
    }
}
impl std::str::FromStr for SizeResize {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "crop" => Ok(Self::Crop),
            "fit" => Ok(Self::Fit),
            _ => Err("invalid value"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub contributors: (),
    pub coordinates: Option<Point>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_text_range: Option<Range>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_entities: Option<ExtendedEntities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_tweet: Option<ExtendedTweet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorite_count: Option<Count>,
    pub favorited: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_level: Option<StatusFilterLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_text: Option<String>,
    pub geo: Option<Point>,
    pub id: Id,
    pub id_str: IdStr,
    pub in_reply_to_screen_name: Option<String>,
    pub in_reply_to_status_id: IdNullable,
    pub in_reply_to_status_id_str: IdStrNullable,
    pub in_reply_to_user_id: IdNullable,
    pub in_reply_to_user_id_str: IdStrNullable,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_quote_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<StatusMetadata>,
    pub place: Option<Place>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub possibly_sensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted_status: Option<Box<Status>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted_status_id: Option<Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted_status_id_str: Option<IdStr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted_status_permalink: Option<StatusQuotedStatusPermalink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retweet_count: Option<StatusRetweetCount>,
    pub retweeted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retweeted_status: Option<Box<Status>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<StatusScopes>,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_ms: Option<StatusTimestampMs>,
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withheld_copyright: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub withheld_in_countries: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withheld_scope: Option<WithheldScope>,
}
impl Status {
    pub fn builder() -> builder::Status {
        builder::Status::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StatusFilterLevel {
    #[serde(rename = "low")]
    Low,
}
impl ToString for StatusFilterLevel {
    fn to_string(&self) -> String {
        match *self {
            Self::Low => "low".to_string(),
        }
    }
}
impl std::str::FromStr for StatusFilterLevel {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "low" => Ok(Self::Low),
            _ => Err("invalid value"),
        }
    }
}
#[doc = "Only included in search API results"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusMetadata {
    pub iso_language_code: String,
    pub result_type: StatusMetadataResultType,
}
impl StatusMetadata {
    pub fn builder() -> builder::StatusMetadata {
        builder::StatusMetadata::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StatusMetadataResultType {
    #[serde(rename = "mixed")]
    Mixed,
    #[serde(rename = "recent")]
    Recent,
    #[serde(rename = "popular")]
    Popular,
}
impl ToString for StatusMetadataResultType {
    fn to_string(&self) -> String {
        match *self {
            Self::Mixed => "mixed".to_string(),
            Self::Recent => "recent".to_string(),
            Self::Popular => "popular".to_string(),
        }
    }
}
impl std::str::FromStr for StatusMetadataResultType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "mixed" => Ok(Self::Mixed),
            "recent" => Ok(Self::Recent),
            "popular" => Ok(Self::Popular),
            _ => Err("invalid value"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusQuotedStatusPermalink {
    pub display: String,
    pub expanded: String,
    pub url: String,
}
impl StatusQuotedStatusPermalink {
    pub fn builder() -> builder::StatusQuotedStatusPermalink {
        builder::StatusQuotedStatusPermalink::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StatusRetweetCount {
    Variant0(Count),
    Variant1(serde_json::Value),
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusScopes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub place_ids: Vec<String>,
}
impl StatusScopes {
    pub fn builder() -> builder::StatusScopes {
        builder::StatusScopes::default()
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct StatusTimestampMs(String);
impl std::ops::Deref for StatusTimestampMs {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for StatusTimestampMs {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^\\d+$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for StatusTimestampMs {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for StatusTimestampMs {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StatusTimestampMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Symbol {
    pub indices: Range,
    pub text: String,
}
impl Symbol {
    pub fn builder() -> builder::Symbol {
        builder::Symbol::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    pub expanded_url: Option<String>,
    pub indices: Range,
    pub url: String,
}
impl Url {
    pub fn builder() -> builder::Url {
        builder::Url::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub contributors_enabled: bool,
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<UserEntities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_has_nft_avatar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_is_blue_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_verified_type: Option<UserExtVerifiedType>,
    pub favourites_count: Count,
    pub follow_request_sent: Option<bool>,
    pub followers_count: Count,
    pub following: Option<bool>,
    pub friends_count: Count,
    pub geo_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_extended_profile: Option<bool>,
    pub id: Id,
    pub id_str: IdStr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_translation_enabled: Option<bool>,
    pub is_translator: bool,
    pub lang: Option<String>,
    pub listed_count: Count,
    pub location: Option<String>,
    pub name: String,
    pub notifications: Option<bool>,
    pub profile_background_color: UserProfileBackgroundColor,
    pub profile_background_image_url: Option<String>,
    pub profile_background_image_url_https: Option<String>,
    pub profile_background_tile: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_banner_url: Option<String>,
    pub profile_image_url: String,
    pub profile_image_url_https: String,
    pub profile_link_color: UserProfileLinkColor,
    pub profile_sidebar_border_color: UserProfileSidebarBorderColor,
    pub profile_sidebar_fill_color: UserProfileSidebarFillColor,
    pub profile_text_color: UserProfileTextColor,
    pub profile_use_background_image: bool,
    pub protected: bool,
    pub screen_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_all_inline_media: Option<bool>,
    #[doc = "A special non-API field that we use in some tooling to record access time"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<Status>>,
    pub statuses_count: Count,
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translator_type: Option<UserTranslatorType>,
    pub url: Option<String>,
    pub utc_offset: Option<i32>,
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub withheld_in_countries: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withheld_scope: Option<WithheldScope>,
}
impl User {
    pub fn builder() -> builder::User {
        builder::User::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntities {
    pub description: UserEntitiesDescription,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<UserEntitiesUrl>,
}
impl UserEntities {
    pub fn builder() -> builder::UserEntities {
        builder::UserEntities::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntitiesDescription {
    pub urls: Vec<Url>,
}
impl UserEntitiesDescription {
    pub fn builder() -> builder::UserEntitiesDescription {
        builder::UserEntitiesDescription::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntitiesUrl {
    pub urls: Vec<Url>,
}
impl UserEntitiesUrl {
    pub fn builder() -> builder::UserEntitiesUrl {
        builder::UserEntitiesUrl::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UserExtVerifiedType {
    Business,
    Government,
    User,
}
impl ToString for UserExtVerifiedType {
    fn to_string(&self) -> String {
        match *self {
            Self::Business => "Business".to_string(),
            Self::Government => "Government".to_string(),
            Self::User => "User".to_string(),
        }
    }
}
impl std::str::FromStr for UserExtVerifiedType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Business" => Ok(Self::Business),
            "Government" => Ok(Self::Government),
            "User" => Ok(Self::User),
            _ => Err("invalid value"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserMention {
    pub id: Id,
    pub id_str: IdStr,
    pub indices: Range,
    pub name: String,
    pub screen_name: String,
}
impl UserMention {
    pub fn builder() -> builder::UserMention {
        builder::UserMention::default()
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct UserProfileBackgroundColor(String);
impl std::ops::Deref for UserProfileBackgroundColor {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for UserProfileBackgroundColor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^[0-9a-fA-F]{6}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{6}$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for UserProfileBackgroundColor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for UserProfileBackgroundColor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserProfileBackgroundColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct UserProfileLinkColor(String);
impl std::ops::Deref for UserProfileLinkColor {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for UserProfileLinkColor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^[0-9a-fA-F]{6}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{6}$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for UserProfileLinkColor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for UserProfileLinkColor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserProfileLinkColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct UserProfileSidebarBorderColor(String);
impl std::ops::Deref for UserProfileSidebarBorderColor {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for UserProfileSidebarBorderColor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^[0-9a-fA-F]{6}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{6}$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for UserProfileSidebarBorderColor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for UserProfileSidebarBorderColor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserProfileSidebarBorderColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct UserProfileSidebarFillColor(String);
impl std::ops::Deref for UserProfileSidebarFillColor {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for UserProfileSidebarFillColor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^([0-9a-fA-F]{6})?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^([0-9a-fA-F]{6})?$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for UserProfileSidebarFillColor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for UserProfileSidebarFillColor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserProfileSidebarFillColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct UserProfileTextColor(String);
impl std::ops::Deref for UserProfileTextColor {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for UserProfileTextColor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if regress::Regex::new("^[0-9a-fA-F]{6}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{6}$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for UserProfileTextColor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for UserProfileTextColor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserProfileTextColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UserTranslatorType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "badged")]
    Badged,
    #[serde(rename = "moderator")]
    Moderator,
}
impl ToString for UserTranslatorType {
    fn to_string(&self) -> String {
        match *self {
            Self::None => "none".to_string(),
            Self::Regular => "regular".to_string(),
            Self::Badged => "badged".to_string(),
            Self::Moderator => "moderator".to_string(),
        }
    }
}
impl std::str::FromStr for UserTranslatorType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "regular" => Ok(Self::Regular),
            "badged" => Ok(Self::Badged),
            "moderator" => Ok(Self::Moderator),
            _ => Err("invalid value"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<Count>,
    pub content_type: VariantContentType,
    pub url: String,
}
impl Variant {
    pub fn builder() -> builder::Variant {
        builder::Variant::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VariantContentType {
    #[serde(rename = "video/mp4")]
    VideoMp4,
    #[serde(rename = "application/x-mpegURL")]
    ApplicationXMpegUrl,
}
impl ToString for VariantContentType {
    fn to_string(&self) -> String {
        match *self {
            Self::VideoMp4 => "video/mp4".to_string(),
            Self::ApplicationXMpegUrl => "application/x-mpegURL".to_string(),
        }
    }
}
impl std::str::FromStr for VariantContentType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "video/mp4" => Ok(Self::VideoMp4),
            "application/x-mpegURL" => Ok(Self::ApplicationXMpegUrl),
            _ => Err("invalid value"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoInfo {
    pub aspect_ratio: Range,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<Count>,
    pub variants: Vec<Variant>,
}
impl VideoInfo {
    pub fn builder() -> builder::VideoInfo {
        builder::VideoInfo::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WithheldScope {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "user")]
    User,
}
impl ToString for WithheldScope {
    fn to_string(&self) -> String {
        match *self {
            Self::Status => "status".to_string(),
            Self::User => "user".to_string(),
        }
    }
}
impl std::str::FromStr for WithheldScope {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "status" => Ok(Self::Status),
            "user" => Ok(Self::User),
            _ => Err("invalid value"),
        }
    }
}
mod builder {
    pub struct Deletion {
        delete: Result<super::DeletionDelete, String>,
    }
    impl Default for Deletion {
        fn default() -> Self {
            Self {
                delete: Err("no value supplied for delete".to_string()),
            }
        }
    }
    impl Deletion {
        pub fn delete<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DeletionDelete>,
            T::Error: std::fmt::Display,
        {
            self.delete = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delete: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Deletion> for super::Deletion {
        type Error = String;
        fn try_from(value: Deletion) -> Result<Self, Self::Error> {
            Ok(Self {
                delete: value.delete?,
            })
        }
    }
    pub struct DeletionDelete {
        status: Result<super::DeletionDeleteStatus, String>,
        timestamp_ms: Result<Option<super::DeletionDeleteTimestampMs>, String>,
    }
    impl Default for DeletionDelete {
        fn default() -> Self {
            Self {
                status: Err("no value supplied for status".to_string()),
                timestamp_ms: Ok(Default::default()),
            }
        }
    }
    impl DeletionDelete {
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DeletionDeleteStatus>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn timestamp_ms<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DeletionDeleteTimestampMs>>,
            T::Error: std::fmt::Display,
        {
            self.timestamp_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp_ms: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DeletionDelete> for super::DeletionDelete {
        type Error = String;
        fn try_from(value: DeletionDelete) -> Result<Self, Self::Error> {
            Ok(Self {
                status: value.status?,
                timestamp_ms: value.timestamp_ms?,
            })
        }
    }
    pub struct DeletionDeleteStatus {
        id: Result<super::Id, String>,
        id_str: Result<super::IdStr, String>,
        user_id: Result<super::Id, String>,
        user_id_str: Result<super::IdStr, String>,
    }
    impl Default for DeletionDeleteStatus {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                id_str: Err("no value supplied for id_str".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
                user_id_str: Err("no value supplied for user_id_str".to_string()),
            }
        }
    }
    impl DeletionDeleteStatus {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id_str: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
        pub fn user_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.user_id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id_str: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DeletionDeleteStatus> for super::DeletionDeleteStatus {
        type Error = String;
        fn try_from(value: DeletionDeleteStatus) -> Result<Self, Self::Error> {
            Ok(Self {
                id: value.id?,
                id_str: value.id_str?,
                user_id: value.user_id?,
                user_id_str: value.user_id_str?,
            })
        }
    }
    pub struct Entities {
        hashtags: Result<Vec<super::Hashtag>, String>,
        media: Result<Vec<super::Media>, String>,
        symbols: Result<Vec<super::Symbol>, String>,
        urls: Result<Vec<super::Url>, String>,
        user_mentions: Result<Vec<super::UserMention>, String>,
    }
    impl Default for Entities {
        fn default() -> Self {
            Self {
                hashtags: Err("no value supplied for hashtags".to_string()),
                media: Ok(Default::default()),
                symbols: Ok(Default::default()),
                urls: Err("no value supplied for urls".to_string()),
                user_mentions: Err("no value supplied for user_mentions".to_string()),
            }
        }
    }
    impl Entities {
        pub fn hashtags<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Hashtag>>,
            T::Error: std::fmt::Display,
        {
            self.hashtags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hashtags: {}", e));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Media>>,
            T::Error: std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {}", e));
            self
        }
        pub fn symbols<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Symbol>>,
            T::Error: std::fmt::Display,
        {
            self.symbols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbols: {}", e));
            self
        }
        pub fn urls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Url>>,
            T::Error: std::fmt::Display,
        {
            self.urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urls: {}", e));
            self
        }
        pub fn user_mentions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::UserMention>>,
            T::Error: std::fmt::Display,
        {
            self.user_mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_mentions: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Entities> for super::Entities {
        type Error = String;
        fn try_from(value: Entities) -> Result<Self, Self::Error> {
            Ok(Self {
                hashtags: value.hashtags?,
                media: value.media?,
                symbols: value.symbols?,
                urls: value.urls?,
                user_mentions: value.user_mentions?,
            })
        }
    }
    pub struct ExtendedEntities {
        media: Result<Vec<super::Media>, String>,
    }
    impl Default for ExtendedEntities {
        fn default() -> Self {
            Self {
                media: Err("no value supplied for media".to_string()),
            }
        }
    }
    impl ExtendedEntities {
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Media>>,
            T::Error: std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ExtendedEntities> for super::ExtendedEntities {
        type Error = String;
        fn try_from(value: ExtendedEntities) -> Result<Self, Self::Error> {
            Ok(Self {
                media: value.media?,
            })
        }
    }
    pub struct ExtendedTweet {
        display_text_range: Result<super::Range, String>,
        entities: Result<super::Entities, String>,
        extended_entities: Result<Option<super::ExtendedEntities>, String>,
        full_text: Result<String, String>,
    }
    impl Default for ExtendedTweet {
        fn default() -> Self {
            Self {
                display_text_range: Err("no value supplied for display_text_range".to_string()),
                entities: Err("no value supplied for entities".to_string()),
                extended_entities: Ok(Default::default()),
                full_text: Err("no value supplied for full_text".to_string()),
            }
        }
    }
    impl ExtendedTweet {
        pub fn display_text_range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.display_text_range = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for display_text_range: {}",
                    e
                )
            });
            self
        }
        pub fn entities<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Entities>,
            T::Error: std::fmt::Display,
        {
            self.entities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entities: {}", e));
            self
        }
        pub fn extended_entities<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ExtendedEntities>>,
            T::Error: std::fmt::Display,
        {
            self.extended_entities = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for extended_entities: {}",
                    e
                )
            });
            self
        }
        pub fn full_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.full_text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ExtendedTweet> for super::ExtendedTweet {
        type Error = String;
        fn try_from(value: ExtendedTweet) -> Result<Self, Self::Error> {
            Ok(Self {
                display_text_range: value.display_text_range?,
                entities: value.entities?,
                extended_entities: value.extended_entities?,
                full_text: value.full_text?,
            })
        }
    }
    pub struct Hashtag {
        indices: Result<super::Range, String>,
        text: Result<String, String>,
    }
    impl Default for Hashtag {
        fn default() -> Self {
            Self {
                indices: Err("no value supplied for indices".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl Hashtag {
        pub fn indices<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.indices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for indices: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Hashtag> for super::Hashtag {
        type Error = String;
        fn try_from(value: Hashtag) -> Result<Self, Self::Error> {
            Ok(Self {
                indices: value.indices?,
                text: value.text?,
            })
        }
    }
    pub struct Media {
        additional_media_info: Result<Option<super::MediaAdditionalMediaInfo>, String>,
        display_url: Result<String, String>,
        expanded_url: Result<String, String>,
        id: Result<super::Id, String>,
        id_str: Result<super::IdStr, String>,
        indices: Result<super::Range, String>,
        media_url: Result<String, String>,
        media_url_https: Result<String, String>,
        sizes: Result<Option<super::MediaSizes>, String>,
        source_status_id: Result<Option<super::Id>, String>,
        source_status_id_str: Result<Option<super::IdStr>, String>,
        source_user_id: Result<Option<super::Id>, String>,
        source_user_id_str: Result<Option<super::IdStr>, String>,
        type_: Result<super::MediaType, String>,
        url: Result<String, String>,
        video_info: Result<Option<super::VideoInfo>, String>,
    }
    impl Default for Media {
        fn default() -> Self {
            Self {
                additional_media_info: Ok(Default::default()),
                display_url: Err("no value supplied for display_url".to_string()),
                expanded_url: Err("no value supplied for expanded_url".to_string()),
                id: Err("no value supplied for id".to_string()),
                id_str: Err("no value supplied for id_str".to_string()),
                indices: Err("no value supplied for indices".to_string()),
                media_url: Err("no value supplied for media_url".to_string()),
                media_url_https: Err("no value supplied for media_url_https".to_string()),
                sizes: Ok(Default::default()),
                source_status_id: Ok(Default::default()),
                source_status_id_str: Ok(Default::default()),
                source_user_id: Ok(Default::default()),
                source_user_id_str: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                url: Err("no value supplied for url".to_string()),
                video_info: Ok(Default::default()),
            }
        }
    }
    impl Media {
        pub fn additional_media_info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaAdditionalMediaInfo>>,
            T::Error: std::fmt::Display,
        {
            self.additional_media_info = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for additional_media_info: {}",
                    e
                )
            });
            self
        }
        pub fn display_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.display_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_url: {}", e));
            self
        }
        pub fn expanded_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.expanded_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expanded_url: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id_str: {}", e));
            self
        }
        pub fn indices<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.indices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for indices: {}", e));
            self
        }
        pub fn media_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.media_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_url: {}", e));
            self
        }
        pub fn media_url_https<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.media_url_https = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_url_https: {}", e));
            self
        }
        pub fn sizes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaSizes>>,
            T::Error: std::fmt::Display,
        {
            self.sizes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sizes: {}", e));
            self
        }
        pub fn source_status_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.source_status_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for source_status_id: {}",
                    e
                )
            });
            self
        }
        pub fn source_status_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdStr>>,
            T::Error: std::fmt::Display,
        {
            self.source_status_id_str = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for source_status_id_str: {}",
                    e
                )
            });
            self
        }
        pub fn source_user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.source_user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_user_id: {}", e));
            self
        }
        pub fn source_user_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdStr>>,
            T::Error: std::fmt::Display,
        {
            self.source_user_id_str = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for source_user_id_str: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MediaType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn video_info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VideoInfo>>,
            T::Error: std::fmt::Display,
        {
            self.video_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_info: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Media> for super::Media {
        type Error = String;
        fn try_from(value: Media) -> Result<Self, Self::Error> {
            Ok(Self {
                additional_media_info: value.additional_media_info?,
                display_url: value.display_url?,
                expanded_url: value.expanded_url?,
                id: value.id?,
                id_str: value.id_str?,
                indices: value.indices?,
                media_url: value.media_url?,
                media_url_https: value.media_url_https?,
                sizes: value.sizes?,
                source_status_id: value.source_status_id?,
                source_status_id_str: value.source_status_id_str?,
                source_user_id: value.source_user_id?,
                source_user_id_str: value.source_user_id_str?,
                type_: value.type_?,
                url: value.url?,
                video_info: value.video_info?,
            })
        }
    }
    pub struct MediaAdditionalMediaInfo {
        call_to_actions: Result<Option<super::MediaAdditionalMediaInfoCallToActions>, String>,
        description: Result<Option<String>, String>,
        embeddable: Result<Option<bool>, String>,
        monetizable: Result<bool, String>,
        source_user: Result<Option<super::User>, String>,
        title: Result<Option<String>, String>,
    }
    impl Default for MediaAdditionalMediaInfo {
        fn default() -> Self {
            Self {
                call_to_actions: Ok(Default::default()),
                description: Ok(Default::default()),
                embeddable: Ok(Default::default()),
                monetizable: Err("no value supplied for monetizable".to_string()),
                source_user: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl MediaAdditionalMediaInfo {
        pub fn call_to_actions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaAdditionalMediaInfoCallToActions>>,
            T::Error: std::fmt::Display,
        {
            self.call_to_actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for call_to_actions: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn embeddable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.embeddable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for embeddable: {}", e));
            self
        }
        pub fn monetizable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.monetizable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for monetizable: {}", e));
            self
        }
        pub fn source_user<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::User>>,
            T::Error: std::fmt::Display,
        {
            self.source_user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_user: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaAdditionalMediaInfo> for super::MediaAdditionalMediaInfo {
        type Error = String;
        fn try_from(value: MediaAdditionalMediaInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                call_to_actions: value.call_to_actions?,
                description: value.description?,
                embeddable: value.embeddable?,
                monetizable: value.monetizable?,
                source_user: value.source_user?,
                title: value.title?,
            })
        }
    }
    pub struct MediaAdditionalMediaInfoCallToActions {
        visit_site: Result<Option<super::MediaAdditionalMediaInfoCallToActionsVisitSite>, String>,
        watch_now: Result<Option<super::MediaAdditionalMediaInfoCallToActionsWatchNow>, String>,
    }
    impl Default for MediaAdditionalMediaInfoCallToActions {
        fn default() -> Self {
            Self {
                visit_site: Ok(Default::default()),
                watch_now: Ok(Default::default()),
            }
        }
    }
    impl MediaAdditionalMediaInfoCallToActions {
        pub fn visit_site<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaAdditionalMediaInfoCallToActionsVisitSite>>,
            T::Error: std::fmt::Display,
        {
            self.visit_site = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for visit_site: {}", e));
            self
        }
        pub fn watch_now<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaAdditionalMediaInfoCallToActionsWatchNow>>,
            T::Error: std::fmt::Display,
        {
            self.watch_now = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for watch_now: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaAdditionalMediaInfoCallToActions>
        for super::MediaAdditionalMediaInfoCallToActions
    {
        type Error = String;
        fn try_from(value: MediaAdditionalMediaInfoCallToActions) -> Result<Self, Self::Error> {
            Ok(Self {
                visit_site: value.visit_site?,
                watch_now: value.watch_now?,
            })
        }
    }
    pub struct MediaAdditionalMediaInfoCallToActionsVisitSite {
        url: Result<String, String>,
    }
    impl Default for MediaAdditionalMediaInfoCallToActionsVisitSite {
        fn default() -> Self {
            Self {
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl MediaAdditionalMediaInfoCallToActionsVisitSite {
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaAdditionalMediaInfoCallToActionsVisitSite>
        for super::MediaAdditionalMediaInfoCallToActionsVisitSite
    {
        type Error = String;
        fn try_from(
            value: MediaAdditionalMediaInfoCallToActionsVisitSite,
        ) -> Result<Self, Self::Error> {
            Ok(Self { url: value.url? })
        }
    }
    pub struct MediaAdditionalMediaInfoCallToActionsWatchNow {
        url: Result<String, String>,
    }
    impl Default for MediaAdditionalMediaInfoCallToActionsWatchNow {
        fn default() -> Self {
            Self {
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl MediaAdditionalMediaInfoCallToActionsWatchNow {
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaAdditionalMediaInfoCallToActionsWatchNow>
        for super::MediaAdditionalMediaInfoCallToActionsWatchNow
    {
        type Error = String;
        fn try_from(
            value: MediaAdditionalMediaInfoCallToActionsWatchNow,
        ) -> Result<Self, Self::Error> {
            Ok(Self { url: value.url? })
        }
    }
    pub struct MediaSizes {
        large: Result<super::Size, String>,
        medium: Result<super::Size, String>,
        small: Result<super::Size, String>,
        thumb: Result<super::Size, String>,
    }
    impl Default for MediaSizes {
        fn default() -> Self {
            Self {
                large: Err("no value supplied for large".to_string()),
                medium: Err("no value supplied for medium".to_string()),
                small: Err("no value supplied for small".to_string()),
                thumb: Err("no value supplied for thumb".to_string()),
            }
        }
    }
    impl MediaSizes {
        pub fn large<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Size>,
            T::Error: std::fmt::Display,
        {
            self.large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for large: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Size>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn small<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Size>,
            T::Error: std::fmt::Display,
        {
            self.small = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for small: {}", e));
            self
        }
        pub fn thumb<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Size>,
            T::Error: std::fmt::Display,
        {
            self.thumb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thumb: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaSizes> for super::MediaSizes {
        type Error = String;
        fn try_from(value: MediaSizes) -> Result<Self, Self::Error> {
            Ok(Self {
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
                thumb: value.thumb?,
            })
        }
    }
    pub struct Place {
        attributes: Result<Option<serde_json::Value>, String>,
        bounding_box: Result<Option<super::Polygon>, String>,
        contained_within: Result<Option<serde_json::Value>, String>,
        country: Result<String, String>,
        country_code: Result<String, String>,
        full_name: Result<String, String>,
        id: Result<String, String>,
        name: Result<String, String>,
        place_type: Result<String, String>,
        url: Result<String, String>,
    }
    impl Default for Place {
        fn default() -> Self {
            Self {
                attributes: Ok(Default::default()),
                bounding_box: Err("no value supplied for bounding_box".to_string()),
                contained_within: Ok(Default::default()),
                country: Err("no value supplied for country".to_string()),
                country_code: Err("no value supplied for country_code".to_string()),
                full_name: Err("no value supplied for full_name".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                place_type: Err("no value supplied for place_type".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl Place {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {}", e));
            self
        }
        pub fn bounding_box<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Polygon>>,
            T::Error: std::fmt::Display,
        {
            self.bounding_box = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bounding_box: {}", e));
            self
        }
        pub fn contained_within<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.contained_within = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for contained_within: {}",
                    e
                )
            });
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country: {}", e));
            self
        }
        pub fn country_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.country_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country_code: {}", e));
            self
        }
        pub fn full_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.full_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_name: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn place_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.place_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for place_type: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Place> for super::Place {
        type Error = String;
        fn try_from(value: Place) -> Result<Self, Self::Error> {
            Ok(Self {
                attributes: value.attributes?,
                bounding_box: value.bounding_box?,
                contained_within: value.contained_within?,
                country: value.country?,
                country_code: value.country_code?,
                full_name: value.full_name?,
                id: value.id?,
                name: value.name?,
                place_type: value.place_type?,
                url: value.url?,
            })
        }
    }
    pub struct Point {
        coordinates: Result<super::Coordinates, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Point {
        fn default() -> Self {
            Self {
                coordinates: Err("no value supplied for coordinates".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Point {
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Coordinates>,
            T::Error: std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Point> for super::Point {
        type Error = String;
        fn try_from(value: Point) -> Result<Self, Self::Error> {
            Ok(Self {
                coordinates: value.coordinates?,
                type_: value.type_?,
            })
        }
    }
    pub struct Polygon {
        coordinates: Result<Vec<Vec<super::Coordinates>>, String>,
        type_: Result<serde_json::Value, String>,
    }
    impl Default for Polygon {
        fn default() -> Self {
            Self {
                coordinates: Err("no value supplied for coordinates".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Polygon {
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<Vec<super::Coordinates>>>,
            T::Error: std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Polygon> for super::Polygon {
        type Error = String;
        fn try_from(value: Polygon) -> Result<Self, Self::Error> {
            Ok(Self {
                coordinates: value.coordinates?,
                type_: value.type_?,
            })
        }
    }
    pub struct SearchResults {
        search_metadata: Result<super::SearchResultsSearchMetadata, String>,
        statuses: Result<Vec<super::Status>, String>,
    }
    impl Default for SearchResults {
        fn default() -> Self {
            Self {
                search_metadata: Err("no value supplied for search_metadata".to_string()),
                statuses: Err("no value supplied for statuses".to_string()),
            }
        }
    }
    impl SearchResults {
        pub fn search_metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SearchResultsSearchMetadata>,
            T::Error: std::fmt::Display,
        {
            self.search_metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for search_metadata: {}", e));
            self
        }
        pub fn statuses<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Status>>,
            T::Error: std::fmt::Display,
        {
            self.statuses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for statuses: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SearchResults> for super::SearchResults {
        type Error = String;
        fn try_from(value: SearchResults) -> Result<Self, Self::Error> {
            Ok(Self {
                search_metadata: value.search_metadata?,
                statuses: value.statuses?,
            })
        }
    }
    pub struct SearchResultsSearchMetadata {
        completed_in: Result<f64, String>,
        count: Result<super::Count, String>,
        max_id: Result<super::Id, String>,
        max_id_str: Result<super::IdStr, String>,
        next_results: Result<String, String>,
        query: Result<String, String>,
        refresh_url: Result<String, String>,
        since_id: Result<super::Id, String>,
        since_id_str: Result<super::IdStr, String>,
    }
    impl Default for SearchResultsSearchMetadata {
        fn default() -> Self {
            Self {
                completed_in: Err("no value supplied for completed_in".to_string()),
                count: Err("no value supplied for count".to_string()),
                max_id: Err("no value supplied for max_id".to_string()),
                max_id_str: Err("no value supplied for max_id_str".to_string()),
                next_results: Err("no value supplied for next_results".to_string()),
                query: Err("no value supplied for query".to_string()),
                refresh_url: Err("no value supplied for refresh_url".to_string()),
                since_id: Err("no value supplied for since_id".to_string()),
                since_id_str: Err("no value supplied for since_id_str".to_string()),
            }
        }
    }
    impl SearchResultsSearchMetadata {
        pub fn completed_in<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.completed_in = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completed_in: {}", e));
            self
        }
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn max_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.max_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_id: {}", e));
            self
        }
        pub fn max_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.max_id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_id_str: {}", e));
            self
        }
        pub fn next_results<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.next_results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_results: {}", e));
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for query: {}", e));
            self
        }
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {}", e));
            self
        }
        pub fn since_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.since_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for since_id: {}", e));
            self
        }
        pub fn since_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.since_id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for since_id_str: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SearchResultsSearchMetadata> for super::SearchResultsSearchMetadata {
        type Error = String;
        fn try_from(value: SearchResultsSearchMetadata) -> Result<Self, Self::Error> {
            Ok(Self {
                completed_in: value.completed_in?,
                count: value.count?,
                max_id: value.max_id?,
                max_id_str: value.max_id_str?,
                next_results: value.next_results?,
                query: value.query?,
                refresh_url: value.refresh_url?,
                since_id: value.since_id?,
                since_id_str: value.since_id_str?,
            })
        }
    }
    pub struct Size {
        h: Result<super::Count, String>,
        resize: Result<super::SizeResize, String>,
        w: Result<super::Count, String>,
    }
    impl Default for Size {
        fn default() -> Self {
            Self {
                h: Err("no value supplied for h".to_string()),
                resize: Err("no value supplied for resize".to_string()),
                w: Err("no value supplied for w".to_string()),
            }
        }
    }
    impl Size {
        pub fn h<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.h = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for h: {}", e));
            self
        }
        pub fn resize<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SizeResize>,
            T::Error: std::fmt::Display,
        {
            self.resize = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resize: {}", e));
            self
        }
        pub fn w<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.w = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for w: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Size> for super::Size {
        type Error = String;
        fn try_from(value: Size) -> Result<Self, Self::Error> {
            Ok(Self {
                h: value.h?,
                resize: value.resize?,
                w: value.w?,
            })
        }
    }
    pub struct Status {
        contributors: Result<(), String>,
        coordinates: Result<Option<super::Point>, String>,
        created_at: Result<String, String>,
        display_text_range: Result<Option<super::Range>, String>,
        entities: Result<Option<super::Entities>, String>,
        extended_entities: Result<Option<super::ExtendedEntities>, String>,
        extended_tweet: Result<Option<super::ExtendedTweet>, String>,
        favorite_count: Result<Option<super::Count>, String>,
        favorited: Result<bool, String>,
        filter_level: Result<Option<super::StatusFilterLevel>, String>,
        full_text: Result<Option<String>, String>,
        geo: Result<Option<super::Point>, String>,
        id: Result<super::Id, String>,
        id_str: Result<super::IdStr, String>,
        in_reply_to_screen_name: Result<Option<String>, String>,
        in_reply_to_status_id: Result<super::IdNullable, String>,
        in_reply_to_status_id_str: Result<super::IdStrNullable, String>,
        in_reply_to_user_id: Result<super::IdNullable, String>,
        in_reply_to_user_id_str: Result<super::IdStrNullable, String>,
        is_quote_status: Result<Option<bool>, String>,
        lang: Result<Option<String>, String>,
        metadata: Result<Option<super::StatusMetadata>, String>,
        place: Result<Option<super::Place>, String>,
        possibly_sensitive: Result<Option<bool>, String>,
        quoted_status: Result<Option<Box<super::Status>>, String>,
        quoted_status_id: Result<Option<super::Id>, String>,
        quoted_status_id_str: Result<Option<super::IdStr>, String>,
        quoted_status_permalink: Result<Option<super::StatusQuotedStatusPermalink>, String>,
        retweet_count: Result<Option<super::StatusRetweetCount>, String>,
        retweeted: Result<bool, String>,
        retweeted_status: Result<Option<Box<super::Status>>, String>,
        scopes: Result<Option<super::StatusScopes>, String>,
        source: Result<String, String>,
        text: Result<Option<String>, String>,
        timestamp_ms: Result<Option<super::StatusTimestampMs>, String>,
        truncated: Result<bool, String>,
        user: Result<Option<super::User>, String>,
        withheld_copyright: Result<Option<serde_json::Value>, String>,
        withheld_in_countries: Result<Vec<String>, String>,
        withheld_scope: Result<Option<super::WithheldScope>, String>,
    }
    impl Default for Status {
        fn default() -> Self {
            Self {
                contributors: Err("no value supplied for contributors".to_string()),
                coordinates: Err("no value supplied for coordinates".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                display_text_range: Ok(Default::default()),
                entities: Ok(Default::default()),
                extended_entities: Ok(Default::default()),
                extended_tweet: Ok(Default::default()),
                favorite_count: Ok(Default::default()),
                favorited: Err("no value supplied for favorited".to_string()),
                filter_level: Ok(Default::default()),
                full_text: Ok(Default::default()),
                geo: Err("no value supplied for geo".to_string()),
                id: Err("no value supplied for id".to_string()),
                id_str: Err("no value supplied for id_str".to_string()),
                in_reply_to_screen_name: Err(
                    "no value supplied for in_reply_to_screen_name".to_string()
                ),
                in_reply_to_status_id: Err(
                    "no value supplied for in_reply_to_status_id".to_string()
                ),
                in_reply_to_status_id_str: Err(
                    "no value supplied for in_reply_to_status_id_str".to_string()
                ),
                in_reply_to_user_id: Err("no value supplied for in_reply_to_user_id".to_string()),
                in_reply_to_user_id_str: Err(
                    "no value supplied for in_reply_to_user_id_str".to_string()
                ),
                is_quote_status: Ok(Default::default()),
                lang: Ok(Default::default()),
                metadata: Ok(Default::default()),
                place: Err("no value supplied for place".to_string()),
                possibly_sensitive: Ok(Default::default()),
                quoted_status: Ok(Default::default()),
                quoted_status_id: Ok(Default::default()),
                quoted_status_id_str: Ok(Default::default()),
                quoted_status_permalink: Ok(Default::default()),
                retweet_count: Ok(Default::default()),
                retweeted: Err("no value supplied for retweeted".to_string()),
                retweeted_status: Ok(Default::default()),
                scopes: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                text: Ok(Default::default()),
                timestamp_ms: Ok(Default::default()),
                truncated: Err("no value supplied for truncated".to_string()),
                user: Ok(Default::default()),
                withheld_copyright: Ok(Default::default()),
                withheld_in_countries: Ok(Default::default()),
                withheld_scope: Ok(Default::default()),
            }
        }
    }
    impl Status {
        pub fn contributors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.contributors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contributors: {}", e));
            self
        }
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Point>>,
            T::Error: std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {}", e));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {}", e));
            self
        }
        pub fn display_text_range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Range>>,
            T::Error: std::fmt::Display,
        {
            self.display_text_range = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for display_text_range: {}",
                    e
                )
            });
            self
        }
        pub fn entities<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Entities>>,
            T::Error: std::fmt::Display,
        {
            self.entities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entities: {}", e));
            self
        }
        pub fn extended_entities<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ExtendedEntities>>,
            T::Error: std::fmt::Display,
        {
            self.extended_entities = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for extended_entities: {}",
                    e
                )
            });
            self
        }
        pub fn extended_tweet<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ExtendedTweet>>,
            T::Error: std::fmt::Display,
        {
            self.extended_tweet = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extended_tweet: {}", e));
            self
        }
        pub fn favorite_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Count>>,
            T::Error: std::fmt::Display,
        {
            self.favorite_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for favorite_count: {}", e));
            self
        }
        pub fn favorited<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.favorited = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for favorited: {}", e));
            self
        }
        pub fn filter_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StatusFilterLevel>>,
            T::Error: std::fmt::Display,
        {
            self.filter_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filter_level: {}", e));
            self
        }
        pub fn full_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.full_text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_text: {}", e));
            self
        }
        pub fn geo<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Point>>,
            T::Error: std::fmt::Display,
        {
            self.geo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geo: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id_str: {}", e));
            self
        }
        pub fn in_reply_to_screen_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.in_reply_to_screen_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_reply_to_screen_name: {}",
                    e
                )
            });
            self
        }
        pub fn in_reply_to_status_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdNullable>,
            T::Error: std::fmt::Display,
        {
            self.in_reply_to_status_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_reply_to_status_id: {}",
                    e
                )
            });
            self
        }
        pub fn in_reply_to_status_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStrNullable>,
            T::Error: std::fmt::Display,
        {
            self.in_reply_to_status_id_str = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_reply_to_status_id_str: {}",
                    e
                )
            });
            self
        }
        pub fn in_reply_to_user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdNullable>,
            T::Error: std::fmt::Display,
        {
            self.in_reply_to_user_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_reply_to_user_id: {}",
                    e
                )
            });
            self
        }
        pub fn in_reply_to_user_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStrNullable>,
            T::Error: std::fmt::Display,
        {
            self.in_reply_to_user_id_str = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for in_reply_to_user_id_str: {}",
                    e
                )
            });
            self
        }
        pub fn is_quote_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.is_quote_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_quote_status: {}", e));
            self
        }
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StatusMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn place<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Place>>,
            T::Error: std::fmt::Display,
        {
            self.place = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for place: {}", e));
            self
        }
        pub fn possibly_sensitive<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.possibly_sensitive = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for possibly_sensitive: {}",
                    e
                )
            });
            self
        }
        pub fn quoted_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::Status>>>,
            T::Error: std::fmt::Display,
        {
            self.quoted_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quoted_status: {}", e));
            self
        }
        pub fn quoted_status_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.quoted_status_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for quoted_status_id: {}",
                    e
                )
            });
            self
        }
        pub fn quoted_status_id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IdStr>>,
            T::Error: std::fmt::Display,
        {
            self.quoted_status_id_str = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for quoted_status_id_str: {}",
                    e
                )
            });
            self
        }
        pub fn quoted_status_permalink<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StatusQuotedStatusPermalink>>,
            T::Error: std::fmt::Display,
        {
            self.quoted_status_permalink = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for quoted_status_permalink: {}",
                    e
                )
            });
            self
        }
        pub fn retweet_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StatusRetweetCount>>,
            T::Error: std::fmt::Display,
        {
            self.retweet_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retweet_count: {}", e));
            self
        }
        pub fn retweeted<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.retweeted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retweeted: {}", e));
            self
        }
        pub fn retweeted_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::Status>>>,
            T::Error: std::fmt::Display,
        {
            self.retweeted_status = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retweeted_status: {}",
                    e
                )
            });
            self
        }
        pub fn scopes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StatusScopes>>,
            T::Error: std::fmt::Display,
        {
            self.scopes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scopes: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn timestamp_ms<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StatusTimestampMs>>,
            T::Error: std::fmt::Display,
        {
            self.timestamp_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp_ms: {}", e));
            self
        }
        pub fn truncated<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.truncated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for truncated: {}", e));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::User>>,
            T::Error: std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {}", e));
            self
        }
        pub fn withheld_copyright<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.withheld_copyright = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for withheld_copyright: {}",
                    e
                )
            });
            self
        }
        pub fn withheld_in_countries<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.withheld_in_countries = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for withheld_in_countries: {}",
                    e
                )
            });
            self
        }
        pub fn withheld_scope<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::WithheldScope>>,
            T::Error: std::fmt::Display,
        {
            self.withheld_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for withheld_scope: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Status> for super::Status {
        type Error = String;
        fn try_from(value: Status) -> Result<Self, Self::Error> {
            Ok(Self {
                contributors: value.contributors?,
                coordinates: value.coordinates?,
                created_at: value.created_at?,
                display_text_range: value.display_text_range?,
                entities: value.entities?,
                extended_entities: value.extended_entities?,
                extended_tweet: value.extended_tweet?,
                favorite_count: value.favorite_count?,
                favorited: value.favorited?,
                filter_level: value.filter_level?,
                full_text: value.full_text?,
                geo: value.geo?,
                id: value.id?,
                id_str: value.id_str?,
                in_reply_to_screen_name: value.in_reply_to_screen_name?,
                in_reply_to_status_id: value.in_reply_to_status_id?,
                in_reply_to_status_id_str: value.in_reply_to_status_id_str?,
                in_reply_to_user_id: value.in_reply_to_user_id?,
                in_reply_to_user_id_str: value.in_reply_to_user_id_str?,
                is_quote_status: value.is_quote_status?,
                lang: value.lang?,
                metadata: value.metadata?,
                place: value.place?,
                possibly_sensitive: value.possibly_sensitive?,
                quoted_status: value.quoted_status?,
                quoted_status_id: value.quoted_status_id?,
                quoted_status_id_str: value.quoted_status_id_str?,
                quoted_status_permalink: value.quoted_status_permalink?,
                retweet_count: value.retweet_count?,
                retweeted: value.retweeted?,
                retweeted_status: value.retweeted_status?,
                scopes: value.scopes?,
                source: value.source?,
                text: value.text?,
                timestamp_ms: value.timestamp_ms?,
                truncated: value.truncated?,
                user: value.user?,
                withheld_copyright: value.withheld_copyright?,
                withheld_in_countries: value.withheld_in_countries?,
                withheld_scope: value.withheld_scope?,
            })
        }
    }
    pub struct StatusMetadata {
        iso_language_code: Result<String, String>,
        result_type: Result<super::StatusMetadataResultType, String>,
    }
    impl Default for StatusMetadata {
        fn default() -> Self {
            Self {
                iso_language_code: Err("no value supplied for iso_language_code".to_string()),
                result_type: Err("no value supplied for result_type".to_string()),
            }
        }
    }
    impl StatusMetadata {
        pub fn iso_language_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.iso_language_code = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for iso_language_code: {}",
                    e
                )
            });
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StatusMetadataResultType>,
            T::Error: std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StatusMetadata> for super::StatusMetadata {
        type Error = String;
        fn try_from(value: StatusMetadata) -> Result<Self, Self::Error> {
            Ok(Self {
                iso_language_code: value.iso_language_code?,
                result_type: value.result_type?,
            })
        }
    }
    pub struct StatusQuotedStatusPermalink {
        display: Result<String, String>,
        expanded: Result<String, String>,
        url: Result<String, String>,
    }
    impl Default for StatusQuotedStatusPermalink {
        fn default() -> Self {
            Self {
                display: Err("no value supplied for display".to_string()),
                expanded: Err("no value supplied for expanded".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl StatusQuotedStatusPermalink {
        pub fn display<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.display = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display: {}", e));
            self
        }
        pub fn expanded<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.expanded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expanded: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StatusQuotedStatusPermalink> for super::StatusQuotedStatusPermalink {
        type Error = String;
        fn try_from(value: StatusQuotedStatusPermalink) -> Result<Self, Self::Error> {
            Ok(Self {
                display: value.display?,
                expanded: value.expanded?,
                url: value.url?,
            })
        }
    }
    pub struct StatusScopes {
        followers: Result<Option<bool>, String>,
        place_ids: Result<Vec<String>, String>,
    }
    impl Default for StatusScopes {
        fn default() -> Self {
            Self {
                followers: Ok(Default::default()),
                place_ids: Ok(Default::default()),
            }
        }
    }
    impl StatusScopes {
        pub fn followers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.followers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for followers: {}", e));
            self
        }
        pub fn place_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.place_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for place_ids: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StatusScopes> for super::StatusScopes {
        type Error = String;
        fn try_from(value: StatusScopes) -> Result<Self, Self::Error> {
            Ok(Self {
                followers: value.followers?,
                place_ids: value.place_ids?,
            })
        }
    }
    pub struct Symbol {
        indices: Result<super::Range, String>,
        text: Result<String, String>,
    }
    impl Default for Symbol {
        fn default() -> Self {
            Self {
                indices: Err("no value supplied for indices".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl Symbol {
        pub fn indices<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.indices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for indices: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Symbol> for super::Symbol {
        type Error = String;
        fn try_from(value: Symbol) -> Result<Self, Self::Error> {
            Ok(Self {
                indices: value.indices?,
                text: value.text?,
            })
        }
    }
    pub struct Url {
        display_url: Result<Option<String>, String>,
        expanded_url: Result<Option<String>, String>,
        indices: Result<super::Range, String>,
        url: Result<String, String>,
    }
    impl Default for Url {
        fn default() -> Self {
            Self {
                display_url: Ok(Default::default()),
                expanded_url: Err("no value supplied for expanded_url".to_string()),
                indices: Err("no value supplied for indices".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl Url {
        pub fn display_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.display_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_url: {}", e));
            self
        }
        pub fn expanded_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.expanded_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expanded_url: {}", e));
            self
        }
        pub fn indices<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.indices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for indices: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Url> for super::Url {
        type Error = String;
        fn try_from(value: Url) -> Result<Self, Self::Error> {
            Ok(Self {
                display_url: value.display_url?,
                expanded_url: value.expanded_url?,
                indices: value.indices?,
                url: value.url?,
            })
        }
    }
    pub struct User {
        contributors_enabled: Result<bool, String>,
        created_at: Result<String, String>,
        default_profile: Result<bool, String>,
        default_profile_image: Result<bool, String>,
        description: Result<Option<String>, String>,
        entities: Result<Option<super::UserEntities>, String>,
        ext_has_nft_avatar: Result<Option<bool>, String>,
        ext_is_blue_verified: Result<Option<bool>, String>,
        ext_verified_type: Result<Option<super::UserExtVerifiedType>, String>,
        favourites_count: Result<super::Count, String>,
        follow_request_sent: Result<Option<bool>, String>,
        followers_count: Result<super::Count, String>,
        following: Result<Option<bool>, String>,
        friends_count: Result<super::Count, String>,
        geo_enabled: Result<bool, String>,
        has_extended_profile: Result<Option<bool>, String>,
        id: Result<super::Id, String>,
        id_str: Result<super::IdStr, String>,
        is_translation_enabled: Result<Option<bool>, String>,
        is_translator: Result<bool, String>,
        lang: Result<Option<String>, String>,
        listed_count: Result<super::Count, String>,
        location: Result<Option<String>, String>,
        name: Result<String, String>,
        notifications: Result<Option<bool>, String>,
        profile_background_color: Result<super::UserProfileBackgroundColor, String>,
        profile_background_image_url: Result<Option<String>, String>,
        profile_background_image_url_https: Result<Option<String>, String>,
        profile_background_tile: Result<bool, String>,
        profile_banner_url: Result<Option<String>, String>,
        profile_image_url: Result<String, String>,
        profile_image_url_https: Result<String, String>,
        profile_link_color: Result<super::UserProfileLinkColor, String>,
        profile_sidebar_border_color: Result<super::UserProfileSidebarBorderColor, String>,
        profile_sidebar_fill_color: Result<super::UserProfileSidebarFillColor, String>,
        profile_text_color: Result<super::UserProfileTextColor, String>,
        profile_use_background_image: Result<bool, String>,
        protected: Result<bool, String>,
        screen_name: Result<String, String>,
        show_all_inline_media: Result<Option<bool>, String>,
        snapshot: Result<Option<u64>, String>,
        status: Result<Option<Box<super::Status>>, String>,
        statuses_count: Result<super::Count, String>,
        time_zone: Result<Option<String>, String>,
        translator_type: Result<Option<super::UserTranslatorType>, String>,
        url: Result<Option<String>, String>,
        utc_offset: Result<Option<i32>, String>,
        verified: Result<bool, String>,
        withheld_in_countries: Result<Vec<String>, String>,
        withheld_scope: Result<Option<super::WithheldScope>, String>,
    }
    impl Default for User {
        fn default() -> Self {
            Self {
                contributors_enabled: Err("no value supplied for contributors_enabled".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                default_profile: Err("no value supplied for default_profile".to_string()),
                default_profile_image: Err(
                    "no value supplied for default_profile_image".to_string()
                ),
                description: Err("no value supplied for description".to_string()),
                entities: Ok(Default::default()),
                ext_has_nft_avatar: Ok(Default::default()),
                ext_is_blue_verified: Ok(Default::default()),
                ext_verified_type: Ok(Default::default()),
                favourites_count: Err("no value supplied for favourites_count".to_string()),
                follow_request_sent: Err("no value supplied for follow_request_sent".to_string()),
                followers_count: Err("no value supplied for followers_count".to_string()),
                following: Err("no value supplied for following".to_string()),
                friends_count: Err("no value supplied for friends_count".to_string()),
                geo_enabled: Err("no value supplied for geo_enabled".to_string()),
                has_extended_profile: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                id_str: Err("no value supplied for id_str".to_string()),
                is_translation_enabled: Ok(Default::default()),
                is_translator: Err("no value supplied for is_translator".to_string()),
                lang: Err("no value supplied for lang".to_string()),
                listed_count: Err("no value supplied for listed_count".to_string()),
                location: Err("no value supplied for location".to_string()),
                name: Err("no value supplied for name".to_string()),
                notifications: Err("no value supplied for notifications".to_string()),
                profile_background_color: Err(
                    "no value supplied for profile_background_color".to_string()
                ),
                profile_background_image_url: Err(
                    "no value supplied for profile_background_image_url".to_string(),
                ),
                profile_background_image_url_https: Err(
                    "no value supplied for profile_background_image_url_https".to_string(),
                ),
                profile_background_tile: Err(
                    "no value supplied for profile_background_tile".to_string()
                ),
                profile_banner_url: Ok(Default::default()),
                profile_image_url: Err("no value supplied for profile_image_url".to_string()),
                profile_image_url_https: Err(
                    "no value supplied for profile_image_url_https".to_string()
                ),
                profile_link_color: Err("no value supplied for profile_link_color".to_string()),
                profile_sidebar_border_color: Err(
                    "no value supplied for profile_sidebar_border_color".to_string(),
                ),
                profile_sidebar_fill_color: Err(
                    "no value supplied for profile_sidebar_fill_color".to_string()
                ),
                profile_text_color: Err("no value supplied for profile_text_color".to_string()),
                profile_use_background_image: Err(
                    "no value supplied for profile_use_background_image".to_string(),
                ),
                protected: Err("no value supplied for protected".to_string()),
                screen_name: Err("no value supplied for screen_name".to_string()),
                show_all_inline_media: Ok(Default::default()),
                snapshot: Ok(Default::default()),
                status: Ok(Default::default()),
                statuses_count: Err("no value supplied for statuses_count".to_string()),
                time_zone: Err("no value supplied for time_zone".to_string()),
                translator_type: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
                utc_offset: Err("no value supplied for utc_offset".to_string()),
                verified: Err("no value supplied for verified".to_string()),
                withheld_in_countries: Ok(Default::default()),
                withheld_scope: Ok(Default::default()),
            }
        }
    }
    impl User {
        pub fn contributors_enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.contributors_enabled = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for contributors_enabled: {}",
                    e
                )
            });
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {}", e));
            self
        }
        pub fn default_profile<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.default_profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_profile: {}", e));
            self
        }
        pub fn default_profile_image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.default_profile_image = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for default_profile_image: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn entities<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UserEntities>>,
            T::Error: std::fmt::Display,
        {
            self.entities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entities: {}", e));
            self
        }
        pub fn ext_has_nft_avatar<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.ext_has_nft_avatar = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ext_has_nft_avatar: {}",
                    e
                )
            });
            self
        }
        pub fn ext_is_blue_verified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.ext_is_blue_verified = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ext_is_blue_verified: {}",
                    e
                )
            });
            self
        }
        pub fn ext_verified_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UserExtVerifiedType>>,
            T::Error: std::fmt::Display,
        {
            self.ext_verified_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ext_verified_type: {}",
                    e
                )
            });
            self
        }
        pub fn favourites_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.favourites_count = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for favourites_count: {}",
                    e
                )
            });
            self
        }
        pub fn follow_request_sent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.follow_request_sent = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for follow_request_sent: {}",
                    e
                )
            });
            self
        }
        pub fn followers_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.followers_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for followers_count: {}", e));
            self
        }
        pub fn following<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.following = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for following: {}", e));
            self
        }
        pub fn friends_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.friends_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for friends_count: {}", e));
            self
        }
        pub fn geo_enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.geo_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geo_enabled: {}", e));
            self
        }
        pub fn has_extended_profile<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.has_extended_profile = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for has_extended_profile: {}",
                    e
                )
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id_str: {}", e));
            self
        }
        pub fn is_translation_enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.is_translation_enabled = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for is_translation_enabled: {}",
                    e
                )
            });
            self
        }
        pub fn is_translator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.is_translator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_translator: {}", e));
            self
        }
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {}", e));
            self
        }
        pub fn listed_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.listed_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for listed_count: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn notifications<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.notifications = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notifications: {}", e));
            self
        }
        pub fn profile_background_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserProfileBackgroundColor>,
            T::Error: std::fmt::Display,
        {
            self.profile_background_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_background_color: {}",
                    e
                )
            });
            self
        }
        pub fn profile_background_image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.profile_background_image_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_background_image_url: {}",
                    e
                )
            });
            self
        }
        pub fn profile_background_image_url_https<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.profile_background_image_url_https = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_background_image_url_https: {}",
                    e
                )
            });
            self
        }
        pub fn profile_background_tile<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.profile_background_tile = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_background_tile: {}",
                    e
                )
            });
            self
        }
        pub fn profile_banner_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.profile_banner_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_banner_url: {}",
                    e
                )
            });
            self
        }
        pub fn profile_image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.profile_image_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_image_url: {}",
                    e
                )
            });
            self
        }
        pub fn profile_image_url_https<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.profile_image_url_https = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_image_url_https: {}",
                    e
                )
            });
            self
        }
        pub fn profile_link_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserProfileLinkColor>,
            T::Error: std::fmt::Display,
        {
            self.profile_link_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_link_color: {}",
                    e
                )
            });
            self
        }
        pub fn profile_sidebar_border_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserProfileSidebarBorderColor>,
            T::Error: std::fmt::Display,
        {
            self.profile_sidebar_border_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_sidebar_border_color: {}",
                    e
                )
            });
            self
        }
        pub fn profile_sidebar_fill_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserProfileSidebarFillColor>,
            T::Error: std::fmt::Display,
        {
            self.profile_sidebar_fill_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_sidebar_fill_color: {}",
                    e
                )
            });
            self
        }
        pub fn profile_text_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserProfileTextColor>,
            T::Error: std::fmt::Display,
        {
            self.profile_text_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_text_color: {}",
                    e
                )
            });
            self
        }
        pub fn profile_use_background_image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.profile_use_background_image = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for profile_use_background_image: {}",
                    e
                )
            });
            self
        }
        pub fn protected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.protected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protected: {}", e));
            self
        }
        pub fn screen_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.screen_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for screen_name: {}", e));
            self
        }
        pub fn show_all_inline_media<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.show_all_inline_media = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for show_all_inline_media: {}",
                    e
                )
            });
            self
        }
        pub fn snapshot<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<u64>>,
            T::Error: std::fmt::Display,
        {
            self.snapshot = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for snapshot: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Box<super::Status>>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn statuses_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Count>,
            T::Error: std::fmt::Display,
        {
            self.statuses_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for statuses_count: {}", e));
            self
        }
        pub fn time_zone<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.time_zone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_zone: {}", e));
            self
        }
        pub fn translator_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UserTranslatorType>>,
            T::Error: std::fmt::Display,
        {
            self.translator_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for translator_type: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn utc_offset<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i32>>,
            T::Error: std::fmt::Display,
        {
            self.utc_offset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for utc_offset: {}", e));
            self
        }
        pub fn verified<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.verified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for verified: {}", e));
            self
        }
        pub fn withheld_in_countries<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.withheld_in_countries = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for withheld_in_countries: {}",
                    e
                )
            });
            self
        }
        pub fn withheld_scope<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::WithheldScope>>,
            T::Error: std::fmt::Display,
        {
            self.withheld_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for withheld_scope: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<User> for super::User {
        type Error = String;
        fn try_from(value: User) -> Result<Self, Self::Error> {
            Ok(Self {
                contributors_enabled: value.contributors_enabled?,
                created_at: value.created_at?,
                default_profile: value.default_profile?,
                default_profile_image: value.default_profile_image?,
                description: value.description?,
                entities: value.entities?,
                ext_has_nft_avatar: value.ext_has_nft_avatar?,
                ext_is_blue_verified: value.ext_is_blue_verified?,
                ext_verified_type: value.ext_verified_type?,
                favourites_count: value.favourites_count?,
                follow_request_sent: value.follow_request_sent?,
                followers_count: value.followers_count?,
                following: value.following?,
                friends_count: value.friends_count?,
                geo_enabled: value.geo_enabled?,
                has_extended_profile: value.has_extended_profile?,
                id: value.id?,
                id_str: value.id_str?,
                is_translation_enabled: value.is_translation_enabled?,
                is_translator: value.is_translator?,
                lang: value.lang?,
                listed_count: value.listed_count?,
                location: value.location?,
                name: value.name?,
                notifications: value.notifications?,
                profile_background_color: value.profile_background_color?,
                profile_background_image_url: value.profile_background_image_url?,
                profile_background_image_url_https: value.profile_background_image_url_https?,
                profile_background_tile: value.profile_background_tile?,
                profile_banner_url: value.profile_banner_url?,
                profile_image_url: value.profile_image_url?,
                profile_image_url_https: value.profile_image_url_https?,
                profile_link_color: value.profile_link_color?,
                profile_sidebar_border_color: value.profile_sidebar_border_color?,
                profile_sidebar_fill_color: value.profile_sidebar_fill_color?,
                profile_text_color: value.profile_text_color?,
                profile_use_background_image: value.profile_use_background_image?,
                protected: value.protected?,
                screen_name: value.screen_name?,
                show_all_inline_media: value.show_all_inline_media?,
                snapshot: value.snapshot?,
                status: value.status?,
                statuses_count: value.statuses_count?,
                time_zone: value.time_zone?,
                translator_type: value.translator_type?,
                url: value.url?,
                utc_offset: value.utc_offset?,
                verified: value.verified?,
                withheld_in_countries: value.withheld_in_countries?,
                withheld_scope: value.withheld_scope?,
            })
        }
    }
    pub struct UserEntities {
        description: Result<super::UserEntitiesDescription, String>,
        url: Result<Option<super::UserEntitiesUrl>, String>,
    }
    impl Default for UserEntities {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                url: Ok(Default::default()),
            }
        }
    }
    impl UserEntities {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserEntitiesDescription>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UserEntitiesUrl>>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UserEntities> for super::UserEntities {
        type Error = String;
        fn try_from(value: UserEntities) -> Result<Self, Self::Error> {
            Ok(Self {
                description: value.description?,
                url: value.url?,
            })
        }
    }
    pub struct UserEntitiesDescription {
        urls: Result<Vec<super::Url>, String>,
    }
    impl Default for UserEntitiesDescription {
        fn default() -> Self {
            Self {
                urls: Err("no value supplied for urls".to_string()),
            }
        }
    }
    impl UserEntitiesDescription {
        pub fn urls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Url>>,
            T::Error: std::fmt::Display,
        {
            self.urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urls: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UserEntitiesDescription> for super::UserEntitiesDescription {
        type Error = String;
        fn try_from(value: UserEntitiesDescription) -> Result<Self, Self::Error> {
            Ok(Self { urls: value.urls? })
        }
    }
    pub struct UserEntitiesUrl {
        urls: Result<Vec<super::Url>, String>,
    }
    impl Default for UserEntitiesUrl {
        fn default() -> Self {
            Self {
                urls: Err("no value supplied for urls".to_string()),
            }
        }
    }
    impl UserEntitiesUrl {
        pub fn urls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Url>>,
            T::Error: std::fmt::Display,
        {
            self.urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urls: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UserEntitiesUrl> for super::UserEntitiesUrl {
        type Error = String;
        fn try_from(value: UserEntitiesUrl) -> Result<Self, Self::Error> {
            Ok(Self { urls: value.urls? })
        }
    }
    pub struct UserMention {
        id: Result<super::Id, String>,
        id_str: Result<super::IdStr, String>,
        indices: Result<super::Range, String>,
        name: Result<String, String>,
        screen_name: Result<String, String>,
    }
    impl Default for UserMention {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                id_str: Err("no value supplied for id_str".to_string()),
                indices: Err("no value supplied for indices".to_string()),
                name: Err("no value supplied for name".to_string()),
                screen_name: Err("no value supplied for screen_name".to_string()),
            }
        }
    }
    impl UserMention {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn id_str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IdStr>,
            T::Error: std::fmt::Display,
        {
            self.id_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id_str: {}", e));
            self
        }
        pub fn indices<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.indices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for indices: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn screen_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.screen_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for screen_name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UserMention> for super::UserMention {
        type Error = String;
        fn try_from(value: UserMention) -> Result<Self, Self::Error> {
            Ok(Self {
                id: value.id?,
                id_str: value.id_str?,
                indices: value.indices?,
                name: value.name?,
                screen_name: value.screen_name?,
            })
        }
    }
    pub struct Variant {
        bitrate: Result<Option<super::Count>, String>,
        content_type: Result<super::VariantContentType, String>,
        url: Result<String, String>,
    }
    impl Default for Variant {
        fn default() -> Self {
            Self {
                bitrate: Ok(Default::default()),
                content_type: Err("no value supplied for content_type".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl Variant {
        pub fn bitrate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Count>>,
            T::Error: std::fmt::Display,
        {
            self.bitrate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitrate: {}", e));
            self
        }
        pub fn content_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VariantContentType>,
            T::Error: std::fmt::Display,
        {
            self.content_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content_type: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Variant> for super::Variant {
        type Error = String;
        fn try_from(value: Variant) -> Result<Self, Self::Error> {
            Ok(Self {
                bitrate: value.bitrate?,
                content_type: value.content_type?,
                url: value.url?,
            })
        }
    }
    pub struct VideoInfo {
        aspect_ratio: Result<super::Range, String>,
        duration_millis: Result<Option<super::Count>, String>,
        variants: Result<Vec<super::Variant>, String>,
    }
    impl Default for VideoInfo {
        fn default() -> Self {
            Self {
                aspect_ratio: Err("no value supplied for aspect_ratio".to_string()),
                duration_millis: Ok(Default::default()),
                variants: Err("no value supplied for variants".to_string()),
            }
        }
    }
    impl VideoInfo {
        pub fn aspect_ratio<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Range>,
            T::Error: std::fmt::Display,
        {
            self.aspect_ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aspect_ratio: {}", e));
            self
        }
        pub fn duration_millis<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Count>>,
            T::Error: std::fmt::Display,
        {
            self.duration_millis = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration_millis: {}", e));
            self
        }
        pub fn variants<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Variant>>,
            T::Error: std::fmt::Display,
        {
            self.variants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variants: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<VideoInfo> for super::VideoInfo {
        type Error = String;
        fn try_from(value: VideoInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                aspect_ratio: value.aspect_ratio?,
                duration_millis: value.duration_millis?,
                variants: value.variants?,
            })
        }
    }
}
