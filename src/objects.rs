//! This module contains all available types to use the Bot API</br>
//! All types have functions to create them from_json
//! ```ignore
//! let user = User::from_json(json_data);
//! ```
//! or to turn them back into JSON-format
//! ```ignore
//! let json_data = user.to_json();
//! ```
//! as well as creating empty objects (having all fields filled with default data)
//! ```ignore
//! let user = User::empty()
//! ```
//! All types can also be displayed and cloned.

extern crate json;
extern crate rustc_serialize;

use crate::*;
use json::JsonValue;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    Pre,
    TextLink,
    TextMention,
    Spoiler,
    CustomEmoji
}

impl MessageEntityType {
    fn from_string(s: String) -> MessageEntityType {
        let s = s.as_str();
        match s {
            "mention" => MessageEntityType::Mention,
            "hashtag" => MessageEntityType::Hashtag,
            "cashtag" => MessageEntityType::Cashtag,
            "bot_command" => MessageEntityType::BotCommand,
            "url" => MessageEntityType::Url,
            "email" => MessageEntityType::Email,
            "phone_number" => MessageEntityType::PhoneNumber,
            "bold" => MessageEntityType::Bold,
            "italic" => MessageEntityType::Italic,
            "underline" => MessageEntityType::Underline,
            "strikethrough" => MessageEntityType::Strikethrough,
            "code" => MessageEntityType::Code,
            "pre" => MessageEntityType::Pre,
            "text_link" => MessageEntityType::TextLink,
            "text_mention" => MessageEntityType::TextMention,
            "spoiler" => MessageEntityType::Spoiler,
            "custom_emoji" => MessageEntityType::CustomEmoji,
            _ => panic!("can't find MessageEntityType: \"{}\"", s),
        }
    }
}

impl fmt::Display for MessageEntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageEntityType::Mention => write!(f, "mention"),
            MessageEntityType::Hashtag => write!(f, "hashtag"),
            MessageEntityType::Cashtag => write!(f, "cashtag"),
            MessageEntityType::BotCommand => write!(f, "bot_command"),
            MessageEntityType::Url => write!(f, "url"),
            MessageEntityType::Email => write!(f, "email"),
            MessageEntityType::PhoneNumber => write!(f, "phone_number"),
            MessageEntityType::Bold => write!(f, "bold"),
            MessageEntityType::Italic => write!(f, "italic"),
            MessageEntityType::Underline => write!(f, "underline"),
            MessageEntityType::Strikethrough => write!(f, "strikethrough"),
            MessageEntityType::Code => write!(f, "code"),
            MessageEntityType::Pre => write!(f, "pre"),
            MessageEntityType::TextLink => write!(f, "text_link"),
            MessageEntityType::TextMention => write!(f, "text_mention"),
            MessageEntityType::Spoiler => write!(f, "spoiler"),
            MessageEntityType::CustomEmoji => write!(f, "custom_emoji"),
        }
    }
}

vec_to_json_array! {
    vec_me_to_json_array(MessageEntity)
    vec_i32_to_json_array(i32)
    vec_string_to_json_array(String)
    vec_po_to_json_array(PollOption)
    vec_update_to_json_array(Update)
    vec_user_to_json_array(User)
    vec_photo_size_to_json_array(PhotoSize)
    vec_sticker_to_json_array(Sticker)
    vec_keyboard_button_to_json_array(KeyboardButton)
    vec_inline_keyboard_button_to_json_array(InlineKeyboardButton)
    vec_input_media_to_json_array(InputMedia)
    vec_message_to_json_array(Message)
    vec_chat_member_to_json_array(ChatMember)
    vec_bot_command_to_json_array(BotCommand)
}

vec_vec_to_json_array! {
    vec_vec_photo_size_to_json_array(PhotoSize, vec_photo_size_to_json_array)
    vec_vec_keyboard_button_to_json_array(KeyboardButton, vec_keyboard_button_to_json_array)
    vec_vec_inline_keyboard_button_to_json_array(InlineKeyboardButton, vec_inline_keyboard_button_to_json_array)
}

pub(crate) trait Custom {
    fn from_json(s: JsonValue) -> Self;
    fn create_json(j: JsonValue, v: Self, name: &'static str) -> JsonValue;
    fn to_json(v: Self) -> JsonValue;
    fn push(j: Vec<String>, v: Self, name: &'static str) -> Vec<String>;
    fn default() -> Self;
    fn url_encode(v: Self) -> String;
}

expand_custom_direct_i! {
    impl Custom for i64 (as_i64, unwrap, 0)
    impl Custom for i32 (as_i32, unwrap, 0)
    impl Custom for f64 (as_f64, unwrap, 0.0)
}

expand_custom_vec! {
    impl Custom for Vec<i32> (as_vec_i32, unwrap, [].to_vec(), vec_i32_to_json_array)
    impl Custom for Vec<Update> (as_vec_update, unwrap, [].to_vec(), vec_update_to_json_array)
    impl Custom for Vec<PollOption> (as_vec_poll_option, unwrap, [].to_vec(), vec_po_to_json_array)
    impl Custom for Vec<User> (as_vec_user, unwrap, [].to_vec(), vec_user_to_json_array)
    impl Custom for Vec<Sticker> (as_vec_sticker, unwrap, [].to_vec(), vec_sticker_to_json_array)
    impl Custom for Vec<String> (as_vec_string, unwrap, [].to_vec(), vec_string_to_json_array)
    impl Custom for Vec<MessageEntity> (as_vec_message_entity, unwrap, [].to_vec(), vec_me_to_json_array)
    impl Custom for Vec<InputMedia> (as_vec_input_media, unwrap, [].to_vec(), vec_input_media_to_json_array)
    impl Custom for Vec<Message> (as_vec_message, unwrap, [].to_vec(), vec_message_to_json_array)
    impl Custom for Vec<ChatMember> (as_vec_chat_member, unwrap, [].to_vec(), vec_chat_member_to_json_array)
    impl Custom for Vec<BotCommand> (as_vec_bot_command, unwrap, [].to_vec(), vec_bot_command_to_json_array)
}

expand_custom_vec_vec! {
    impl Custom for Vec<Vec<PhotoSize>> (as_vec_vec_photo_size, unwrap, [].to_vec(), vec_vec_photo_size_to_json_array)
    impl Custom for Vec<Vec<KeyboardButton>> (as_vec_vec_keyboard_button, unwrap, [].to_vec(), vec_vec_keyboard_button_to_json_array)
    impl Custom for Vec<Vec<InlineKeyboardButton>> (as_vec_vec_inline_keyboard_button, unwrap, [].to_vec(), vec_vec_inline_keyboard_button_to_json_array)
}

expand_custom_direct_bool! {
    impl Custom for bool (as_bool, unwrap, false)
}

expand_custom! {
    impl Custom for String (to_string, clone, "".to_string())
    impl Custom for MessageEntityType (as_message_entity_type, unwrap, MessageEntityType::Mention)
}

expand_custom_direct_object! {
    impl Custom for Update (as_update, unwrap, Update::empty())
    impl Custom for User (as_user, unwrap, User::empty())
    impl Custom for Message (as_message, unwrap, Message::empty())
    impl Custom for ChatInviteLink (as_chat_invite_link, unwrap, ChatInviteLink::empty())
    impl Custom for ChatMember (as_chat_member, unwrap, ChatMember::empty())
    impl Custom for Chat (as_chat, unwrap, Chat::empty())
    impl Custom for Sticker (as_sticker, unwrap, Sticker::empty())
    impl Custom for KeyboardButton (as_keyboard_button, unwrap, KeyboardButton::empty())
    impl Custom for ReplyKeyboardMarkup (as_reply_keyboard_markup, unwrap, ReplyKeyboardMarkup::empty())
    impl Custom for ReplyKeyboardRemove (as_reply_keyboard_remove, unwrap, ReplyKeyboardRemove::empty())
    impl Custom for ForceReply (as_force_reply, unwrap, ForceReply::empty())
    impl Custom for Location (as_location, unwrap, Location::empty())
    impl Custom for PollOption (as_poll_option, unwrap, PollOption::empty())
    impl Custom for MessageEntity (as_message_entity, unwrap, MessageEntity::empty())
    impl Custom for PhotoSize (as_photo_size, unwrap, PhotoSize::empty())
    impl Custom for InlineKeyboardMarkup (as_inline_keyboard_markup, unwrap, InlineKeyboardMarkup::empty())
    impl Custom for InlineKeyboardButton (as_inline_keyboard_button, unwrap, InlineKeyboardButton::empty())
    impl Custom for InputMedia (as_input_media, unwrap, InputMedia::empty())
    impl Custom for UserProfilePhotos (as_user_profile_photos, unwrap, UserProfilePhotos::empty())
    impl Custom for ChatPermissions (as_chat_permissions, unwrap, ChatPermissions::empty())
    impl Custom for BotCommand (as_bot_command, unwrap, BotCommand::empty())
    impl Custom for Poll (as_poll, unwrap, Poll::empty())
    impl Custom for StickerSet (as_sticker_set, unwrap, StickerSet::empty())
    impl Custom for MaskPosition (as_mask_position, unwrap, MaskPosition::empty())
}

expand_custom_option! {
    impl Custom for Option<bool> (as_bool, unwrap, None)
    impl Custom for Option<i32> (as_i32, unwrap, None)
    impl Custom for Option<i64> (as_i64, unwrap, None)
    impl Custom for Option<f64> (as_f64, unwrap, None)
    impl Custom for Option<String> (to_string, clone, None)
    impl Custom for Option<User> (as_user, unwrap, None)
    impl Custom for Option<Message> (as_message, unwrap, None)
    impl Custom for Option<InlineKeyboardMarkup> (as_inline_keyboard_markup, unwrap, None)
    impl Custom for Option<VoiceChatStarted> (as_voice_chat_started, unwrap, None)
    impl Custom for Option<VoiceChatEnded> (as_voice_chat_ended, unwrap, None)
    impl Custom for Option<VoiceChatScheduled> (as_voice_chat_scheduled, unwrap, None)
    impl Custom for Option<VoiceChatParticipantsInvited> (as_voice_chat_participants_invited, unwrap, None)
    impl Custom for Option<ProximityAlertTriggered> (as_proximity_alert_triggered, unwrap, None)
    impl Custom for Option<MessageAutoDeleteTimerChanged> (as_message_auto_delete_timer_changed, unwrap, None)
    impl Custom for Option<PhotoSize> (as_photo_size, unwrap, None)
    impl Custom for Option<Contact> (as_contact, unwrap, None)
    impl Custom for Option<Dice> (as_dice, unwrap, None)
    impl Custom for Option<Poll> (as_poll, unwrap, None)
    impl Custom for Option<Venue> (as_venue, unwrap, None)
    impl Custom for Option<ChatPermissions> (as_chat_permissions, unwrap, None)
    impl Custom for Option<Location> (as_location, unwrap, None)
    impl Custom for Option<Chat> (as_chat, unwrap, None)
    impl Custom for Option<ChatPhoto> (as_chat_photo, unwrap, None)
    impl Custom for Option<Animation> (as_animation, unwrap, None)
    impl Custom for Option<Audio> (as_audio, unwrap, None)
    impl Custom for Option<ChatInviteLink> (as_chat_invite_link, unwrap, None)
    impl Custom for Option<ChatMember> (as_chat_member, unwrap, None)
    impl Custom for Option<Document> (as_document, unwrap, None)
    impl Custom for Option<LoginUrl> (as_login_url, unwrap, None)
    impl Custom for Option<Sticker> (as_sticker, unwrap, None)
    impl Custom for Option<Video> (as_video, unwrap, None)
    impl Custom for Option<VideoNote> (as_video_note, unwrap, None)
    impl Custom for Option<Voice> (as_voice, unwrap, None)
    impl Custom for Option<MaskPosition> (as_mask_position, unwrap, None)
    impl Custom for Option<KeyboardButtonPollType> (as_keyboard_button_poll_type, unwrap, None)
    impl Custom for Option<ChatLocation> (as_chat_location, unwrap, None)
    impl Custom for Option<CallbackQuery> (as_callback_query, unwrap, None)
    impl Custom for Option<PollAnswer> (as_poll_answer, unwrap, None)
    impl Custom for Option<ChatMemberUpdated> (as_chat_member_updated, unwrap, None)
}

expand_custom_box! {
    impl Custom for Box<Chat> (as_box_chat, unwrap, Box::new(Chat::empty()))
}

expand_custom_option_box! {
    impl Custom for Option<Box<Chat>> (as_box_chat, unwrap, None)
    impl Custom for Option<Box<Message>> (as_box_message, unwrap, None)
}

expand_custom_option_vec! {
    impl Custom for Option<Vec<MessageEntity>> (as_vec_message_entity, unwrap, None, vec_me_to_json_array)
    impl Custom for Option<Vec<PhotoSize>> (as_vec_photo_size, unwrap, None, vec_photo_size_to_json_array)
    impl Custom for Option<Vec<User>> (as_vec_user, unwrap, None, vec_user_to_json_array)
}

trait JsonExt {
    fn as_update(&self) -> Option<Update>;
    fn as_user(&self) -> Option<User>;
    fn as_chat(&self) -> Option<Chat>;
    fn as_message(&self) -> Option<Message>;
    fn as_sticker(&self) -> Option<Sticker>;
    fn as_keyboard_button_poll_type(&self) -> Option<KeyboardButtonPollType>;
    fn as_keyboard_button(&self) -> Option<KeyboardButton>;
    fn as_reply_keyboard_markup(&self) -> Option<ReplyKeyboardMarkup>;
    fn as_reply_keyboard_remove(&self) -> Option<ReplyKeyboardRemove>;
    fn as_force_reply(&self) -> Option<ForceReply>;
    fn as_location(&self) -> Option<Location>;
    fn as_poll_option(&self) -> Option<PollOption>;
    fn as_message_entity(&self) -> Option<MessageEntity>;
    fn as_photo_size(&self) -> Option<PhotoSize>;
    fn as_mask_position(&self) -> Option<MaskPosition>;
    fn as_inline_keyboard_markup(&self) -> Option<InlineKeyboardMarkup>;
    fn as_inline_keyboard_button(&self) -> Option<InlineKeyboardButton>;
    fn as_voice_chat_started(&self) -> Option<VoiceChatStarted>;
    fn as_voice_chat_ended(&self) -> Option<VoiceChatEnded>;
    fn as_voice_chat_scheduled(&self) -> Option<VoiceChatScheduled>;
    fn as_voice_chat_participants_invited(&self) -> Option<VoiceChatParticipantsInvited>;
    fn as_proximity_alert_triggered(&self) -> Option<ProximityAlertTriggered>;
    fn as_message_auto_delete_timer_changed(&self) -> Option<MessageAutoDeleteTimerChanged>;
    fn as_contact(&self) -> Option<Contact>;
    fn as_dice(&self) -> Option<Dice>;
    fn as_poll(&self) -> Option<Poll>;
    fn as_venue(&self) -> Option<Venue>;
    fn as_chat_permissions(&self) -> Option<ChatPermissions>;
    fn as_chat_photo(&self) -> Option<ChatPhoto>;
    fn as_chat_member(&self) -> Option<ChatMember>;
    fn as_chat_location(&self) -> Option<ChatLocation>;
    fn as_animation(&self) -> Option<Animation>;
    fn as_audio(&self) -> Option<Audio>;
    fn as_chat_invite_link(&self) -> Option<ChatInviteLink>;
    fn as_document(&self) -> Option<Document>;
    fn as_video(&self) -> Option<Video>;
    fn as_video_note(&self) -> Option<VideoNote>;
    fn as_voice(&self) -> Option<Voice>;
    fn as_login_url(&self) -> Option<LoginUrl>;
    fn as_callback_query(&self) -> Option<CallbackQuery>;
    fn as_poll_answer(&self) -> Option<PollAnswer>;
    fn as_chat_member_updated(&self) -> Option<ChatMemberUpdated>;
    fn as_input_media(&self) -> Option<InputMedia>;
    fn as_user_profile_photos(&self) -> Option<UserProfilePhotos>;
    fn as_bot_command(&self) -> Option<BotCommand>;
    fn as_sticker_set(&self) -> Option<StickerSet>;
    fn as_vec_poll_option(&self) -> Option<Vec<PollOption>>;
    fn as_vec_string(&self) -> Option<Vec<String>>;
    fn as_vec_update(&self) -> Option<Vec<Update>>;
    fn as_vec_user(&self) -> Option<Vec<User>>;
    fn as_vec_photo_size(&self) -> Option<Vec<PhotoSize>>;
    fn as_vec_sticker(&self) -> Option<Vec<Sticker>>;
    fn as_vec_keyboard_button(&self) -> Option<Vec<KeyboardButton>>;
    fn as_vec_inline_keyboard_button(&self) -> Option<Vec<InlineKeyboardButton>>;
    fn as_vec_input_media(&self) -> Option<Vec<InputMedia>>;
    fn as_vec_message(&self) -> Option<Vec<Message>>;
    fn as_vec_chat_member(&self) -> Option<Vec<ChatMember>>;
    fn as_vec_bot_command(&self) -> Option<Vec<BotCommand>>;
    fn as_vec_vec_photo_size(&self) -> Option<Vec<Vec<PhotoSize>>>;
    fn as_vec_vec_keyboard_button(&self) -> Option<Vec<Vec<KeyboardButton>>>;
    fn as_vec_vec_inline_keyboard_button(&self) -> Option<Vec<Vec<InlineKeyboardButton>>>;
    fn as_vec_i32(&self) -> Option<Vec<i32>>;
    fn as_message_entity_type(&self) -> Option<MessageEntityType>;
    fn as_vec_message_entity(&self) -> Option<Vec<MessageEntity>>;
    fn as_box_chat(&self) -> Option<Box<Chat>>;
    fn as_box_message(&self) -> Option<Box<Message>>;
}

impl JsonExt for JsonValue {
    as_custom! {
        fn as_update(&self) -> Option<Update>
        fn as_user(&self) -> Option<User>
        fn as_chat(&self) -> Option<Chat>
        fn as_message(&self) -> Option<Message>
        fn as_sticker(&self) -> Option<Sticker>
        fn as_keyboard_button_poll_type(&self) -> Option<KeyboardButtonPollType>
        fn as_keyboard_button(&self) -> Option<KeyboardButton>
        fn as_reply_keyboard_markup(&self) -> Option<ReplyKeyboardMarkup>
        fn as_reply_keyboard_remove(&self) -> Option<ReplyKeyboardRemove>
        fn as_force_reply(&self) -> Option<ForceReply>
        fn as_inline_keyboard_button(&self) -> Option<InlineKeyboardButton>
        fn as_location(&self) -> Option<Location>
        fn as_poll_option(&self) -> Option<PollOption>
        fn as_photo_size(&self) -> Option<PhotoSize>
        fn as_mask_position(&self) -> Option<MaskPosition>
        fn as_message_entity(&self) -> Option<MessageEntity>
        fn as_contact(&self) -> Option<Contact>
        fn as_dice(&self) -> Option<Dice>
        fn as_poll(&self) -> Option<Poll>
        fn as_venue(&self) -> Option<Venue>
        fn as_chat_permissions(&self) -> Option<ChatPermissions>
        fn as_chat_photo(&self) -> Option<ChatPhoto>
        fn as_chat_member(&self) -> Option<ChatMember>
        fn as_chat_location(&self) -> Option<ChatLocation>
        fn as_animation(&self) -> Option<Animation>
        fn as_audio(&self) -> Option<Audio>
        fn as_inline_keyboard_markup(&self) -> Option<InlineKeyboardMarkup>
        fn as_voice_chat_started(&self) -> Option<VoiceChatStarted>
        fn as_voice_chat_ended(&self) -> Option<VoiceChatEnded>
        fn as_voice_chat_scheduled(&self) -> Option<VoiceChatScheduled>
        fn as_voice_chat_participants_invited(&self) -> Option<VoiceChatParticipantsInvited>
        fn as_proximity_alert_triggered(&self) -> Option<ProximityAlertTriggered>
        fn as_message_auto_delete_timer_changed(&self) -> Option<MessageAutoDeleteTimerChanged>
        fn as_chat_invite_link(&self) -> Option<ChatInviteLink>
        fn as_document(&self) -> Option<Document>
        fn as_video(&self) -> Option<Video>
        fn as_video_note(&self) -> Option<VideoNote>
        fn as_voice(&self) -> Option<Voice>
        fn as_login_url(&self) -> Option<LoginUrl>
        fn as_callback_query(&self) -> Option<CallbackQuery>
        fn as_poll_answer(&self) -> Option<PollAnswer>
        fn as_chat_member_updated(&self) -> Option<ChatMemberUpdated>
        fn as_input_media(&self) -> Option<InputMedia>
        fn as_user_profile_photos(&self) -> Option<UserProfilePhotos>
        fn as_bot_command(&self) -> Option<BotCommand>
        fn as_sticker_set(&self) -> Option<StickerSet>
    }
    as_vec_custom! {
        fn as_vec_poll_option(&self) -> Option<Vec<PollOption>>
        fn as_vec_update(&self) -> Option<Vec<Update>>
        fn as_vec_user(&self) -> Option<Vec<User>>
        fn as_vec_i32(&self) -> Option<Vec<i32>>
        fn as_vec_string(&self) -> Option<Vec<String>>
        fn as_vec_photo_size(&self) -> Option<Vec<PhotoSize>>
        fn as_vec_sticker(&self) -> Option<Vec<Sticker>>
        fn as_vec_keyboard_button(&self) -> Option<Vec<KeyboardButton>>
        fn as_vec_inline_keyboard_button(&self) -> Option<Vec<InlineKeyboardButton>>
        fn as_vec_message_entity(&self) -> Option<Vec<MessageEntity>>
        fn as_vec_input_media(&self) -> Option<Vec<InputMedia>>
        fn as_vec_message(&self) -> Option<Vec<Message>>
        fn as_vec_chat_member(&self) -> Option<Vec<ChatMember>>
        fn as_vec_bot_command(&self) -> Option<Vec<BotCommand>>
    }
    as_vec_vec_custom! {
        fn as_vec_vec_photo_size(&self, as_vec_photo_size) -> Option<Vec<Vec<PhotoSize>>>
        fn as_vec_vec_keyboard_button(&self, as_vec_keyboard_button) -> Option<Vec<Vec<KeyboardButton>>>
        fn as_vec_vec_inline_keyboard_button(&self, as_vec_inline_keyboard_button) -> Option<Vec<Vec<InlineKeyboardButton>>>
    }
    as_box_custom! {
        fn as_box_chat(&self) -> Option<Box<Chat>>
        fn as_box_message(&self) -> Option<Box<Message>>
    }
    fn as_message_entity_type(&self) -> Option<MessageEntityType> {
        if self.is_empty() {
            None
        } else {
            Some(MessageEntityType::from_string(format!("{}", self)))
        }
    }
}
expand_from! {
    impl From<Update> for JsonValue
    impl From<User> for JsonValue
    impl From<Location> for JsonValue
    impl From<MessageEntity> for JsonValue
    impl From<PhotoSize> for JsonValue
    impl From<PollOption> for JsonValue
    impl From<InlineKeyboardButton> for JsonValue
    impl From<KeyboardButtonPollType> for JsonValue
    impl From<KeyboardButton> for JsonValue
    impl From<ForceReply> for JsonValue
    impl From<ReplyKeyboardMarkup> for JsonValue
    impl From<ReplyKeyboardRemove> for JsonValue
    impl From<Audio> for JsonValue
    impl From<Animation> for JsonValue
    impl From<ChatPhoto> for JsonValue
    impl From<ChatPermissions> for JsonValue
    impl From<Venue> for JsonValue
    impl From<Poll> for JsonValue
    impl From<Dice> for JsonValue
    impl From<Contact> for JsonValue
    impl From<VoiceChatScheduled> for JsonValue
    impl From<VoiceChatStarted> for JsonValue
    impl From<VoiceChatEnded> for JsonValue
    impl From<VoiceChatParticipantsInvited> for JsonValue
    impl From<ProximityAlertTriggered> for JsonValue
    impl From<MessageAutoDeleteTimerChanged> for JsonValue
    impl From<InlineKeyboardMarkup> for JsonValue
    impl From<Message> for JsonValue
    impl From<Chat> for JsonValue
    impl From<Video> for JsonValue
    impl From<Voice> for JsonValue
    impl From<VideoNote> for JsonValue
    impl From<Document> for JsonValue
    impl From<ChatMember> for JsonValue
    impl From<ChatInviteLink> for JsonValue
    impl From<LoginUrl> for JsonValue
    impl From<ChatLocation> for JsonValue
    impl From<CallbackQuery> for JsonValue
    impl From<PollAnswer> for JsonValue
    impl From<ChatMemberUpdated> for JsonValue
    impl From<InputMedia> for JsonValue
    impl From<Sticker> for JsonValue
    impl From<MaskPosition> for JsonValue
    impl From<UserProfilePhotos> for JsonValue
    impl From<BotCommand> for JsonValue
    impl From<StickerSet> for JsonValue
}

add_functionality! {
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
    pub my_chat_member: Option<ChatMemberUpdated>,
    pub chat_member: Option<ChatMemberUpdated>
}

pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>
}

pub struct Chat {
    pub id: i64,
    pub typ: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Message>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<i32>,
    pub message_auto_delete_time: Option<i32>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>
}

pub struct Message {
    pub message_id: i32,
    pub from: Option<User>,
    pub sender_chat: Option<Box<Chat>>,
    pub date: i32,
    pub chat: Box<Chat>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Box<Chat>>,
    pub forward_from_message_id: Option<i32>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<i32>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<i32>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub connected_website: Option<String>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub voice_chat_scheduled: Option<VoiceChatScheduled>,
    pub voice_chat_started: Option<VoiceChatStarted>,
    pub voice_chat_ended: Option<VoiceChatEnded>,
    pub voice_chat_participants_invited: Option<VoiceChatParticipantsInvited>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}

pub struct MessageId {
    pub message_id: i32
}

pub struct MessageEntity {
    pub typ: MessageEntityType,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>
}

pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>
}

pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}

pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
    pub thumb: Option<PhotoSize>
}

pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}

pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}

pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i32>
}

pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}

pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>
}

pub struct Dice {
    pub emoji: String,
    pub value: i32
}

pub struct PollOption {
    pub text: String,
    pub voter_count: i32
}

pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i32>
}

pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub typ: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i32>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i32>,
    pub close_date: Option<i32>
}

pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i32>,
    pub heading: Option<i32>,
    pub proximity_alert_radius: Option<i32>
}

pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>
}

pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i32
}

pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i32
}

pub struct VoiceChatScheduled {
    pub start_date: i32
}

pub struct VoiceChatEnded {
    pub duration: i32
}

pub struct VoiceChatParticipantsInvited {
    pub users: Vec<User>
}

pub struct UserProfilePhotos {
    pub total_count: i32,
    pub photos: Vec<Vec<PhotoSize>>
}

pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<String>,
    pub file_path: Option<String>
}

pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>
}

pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>
}

pub struct KeyboardButtonPollType {
    pub typ: String
}

pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool, // should always be true
    pub selective: Option<bool>
}

pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub pay: Option<bool>
}

pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>
}

pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: Option<String>,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}

pub struct ForceReply {
    pub force_reply: bool, //should always be true
    pub selective: Option<bool>
}

pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String
}

pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub expire_date: Option<i32>,
    pub member_limit: Option<i32>
}

pub struct ChatMember {
    pub user: User,
    pub status: String,
    pub custom_title: Option<String>,
    pub is_anonymous: Option<bool>,
    pub can_be_edited: Option<bool>,
    pub can_manage_chat: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_manage_voice_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub is_member: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub until_date: Option<bool>
}

pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i32,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>
}

pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>
}

pub struct ChatLocation {
    pub location: Location,
    pub address: String
}

pub struct BotCommand {
    pub command: String,
    pub description: String
}

pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i32>,
    pub retry_after: Option<i32>
}

pub struct InputMedia {
    pub typ: String,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub supports_streaming: Option<bool>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub disable_content_type_detection: Option<bool>
}

pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub is_animated: bool,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<i32>
}

pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
    pub thumb: Option<PhotoSize>
}

pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64
}}

add_functionality_empty! {
    pub struct VoiceChatStarted {
}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_user() {
        let actual = format!("{}", User::empty().to_json());
        let reference = "{\"id\":0,\"is_bot\":false,\"first_name\":\"\"}".to_string();
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_minimal_user() {
        let json_user = json::parse("{\"id\":1234,\"is_bot\":true,\"first_name\":\"iamgroot\"}");
        let user;
        match json_user {
            Ok(json_data) => user = User::from_json(json_data),
            Err(_) => user = User::empty(),
        }
        let actual = format!("{}", user.to_json());
        let reference = "{\"id\":1234,\"is_bot\":true,\"first_name\":\"iamgroot\"}".to_string();
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_full_user() {
        let reference = "{\"id\":1234,\"is_bot\":true,\"first_name\":\"iAm\",\
            \"last_name\":\"groot\",\"language_code\":\"US\",\"can_join_groups\":true,\
            \"can_read_all_group_messages\":false,\"supports_inline_queries\":true}"
            .to_string();
        let json_user = json::parse(reference.as_str());
        let user;
        match json_user {
            Ok(json_data) => user = User::from_json(json_data),
            Err(_) => user = User::empty(),
        }
        let actual = format!("{}", user.to_json());
        assert_eq!(actual, reference);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_invalid_user() {
        let json_user = json::parse("{\"id\":1234,\"first_name\":\"iamgroot\"}");
        let _user;
        match json_user {
            Ok(json_data) => _user = User::from_json(json_data),
            Err(_) => _user = User::empty(),
        }
    }

    #[test]
    fn test_clone_user() {
        let mut user = User::empty();
        let orig_user = user.clone();
        user.first_name = "ichangedmyname".to_string();
        let actual1 = format!("{}", user.to_json());
        let reference1 =
            "{\"id\":0,\"is_bot\":false,\"first_name\":\"ichangedmyname\"}".to_string();
        assert_eq!(actual1, reference1);
        let actual2 = format!("{}", orig_user.to_json());
        let reference2 = "{\"id\":0,\"is_bot\":false,\"first_name\":\"\"}".to_string();
        assert_eq!(actual2, reference2);
    }

    #[test]
    fn test_display_user() {
        let user = User::empty();
        let reference = "id: 0; is_bot: false; first_name: ".to_string();
        let actual = format!("{}", user);
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_large_user_id() {
        let mut user = User::empty();
        user.id = 288230376151711744;
        let reference = "id: 288230376151711744; is_bot: false; first_name: ".to_string();
        let actual = format!("{}", user);
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_empty_me() {
        let actual = format!("{}", MessageEntity::empty().to_json());
        let reference = "{\"type\":\"mention\",\"offset\":0,\"length\":0}".to_string();
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_minimal_me() {
        let json_me = json::parse("{\"type\":\"cashtag\",\"offset\":42,\"length\":69}");
        let me;
        match json_me {
            Ok(json_data) => me = MessageEntity::from_json(json_data),
            Err(_) => me = MessageEntity::empty(),
        }
        let actual = format!("{}", me.to_json());
        let reference = "{\"type\":\"cashtag\",\"offset\":42,\"length\":69}".to_string();
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_full_me() {
        let reference = "{\"type\":\"cashtag\",\"offset\":42,\"length\":69,\
            \"url\":\"https://example.org\",\"user\":{\"id\":0,\"is_bot\":false,\"first_name\":\"user\"},\
            \"language\":\"python\"}".to_string();
        let json_me = json::parse(reference.as_str());
        let me;
        match json_me {
            Ok(json_data) => me = MessageEntity::from_json(json_data),
            Err(_) => me = MessageEntity::empty(),
        }
        let actual = format!("{}", me.to_json());
        assert_eq!(actual, reference);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_invalid_me() {
        let json_me = json::parse("{\"type\":\"cashtag\",\"length\":69}");
        let _me;
        match json_me {
            Ok(json_data) => _me = MessageEntity::from_json(json_data),
            Err(_) => _me = MessageEntity::empty(),
        }
    }

    #[test]
    fn test_clone_me() {
        let mut me = MessageEntity::empty();
        let orig_me = me.clone();
        me.offset = 42;
        let actual1 = format!("{}", me.to_json());
        let reference1 = "{\"type\":\"mention\",\"offset\":42,\"length\":0}".to_string();
        assert_eq!(actual1, reference1);
        let actual2 = format!("{}", orig_me.to_json());
        let reference2 = "{\"type\":\"mention\",\"offset\":0,\"length\":0}".to_string();
        assert_eq!(actual2, reference2);
    }

    #[test]
    fn test_display_me() {
        let me = MessageEntity::empty();
        let reference = "type: mention; offset: 0; length: 0".to_string();
        let actual = format!("{}", me);
        assert_eq!(actual, reference);
    }

    #[test]
    fn test_input_media_photo() {
        let reference = r#"{"type":"photo","media":"test1234","caption_entities":[{"type":"mention","offset":0,"length":0},{"type":"mention","offset":1,"length":0}]}"#;
        expand_basic_test! {
            fn run_test(InputMedia, reference)
        }
    }

    #[test]
    fn test_chat_photo() {
        let reference = r#"{"small_file_id":"1","small_file_unique_id":"1234","big_file_id":"2","big_file_unique_id":"2345"}"#;
        expand_basic_test! {
            fn run_test(ChatPhoto, reference)
        }
    }

    #[test]
    fn test_chat_invite_link() {
        let reference = r#"{"invite_link":"hello","creator":{"id":1234,"is_bot":true,"first_name":"groot"},"is_primary":true,"is_revoked":false}"#;
        expand_basic_test! {
            fn run_test(ChatInviteLink, reference)
        }
    }

    #[test]
    fn test_chat_member() {
        let reference =
            r#"{"user":{"id":1234,"is_bot":true,"first_name":"groot"},"status":"creator"}"#;
        expand_basic_test! {
            fn run_test(ChatMember, reference)
        }
    }

    #[test]
    fn test_chat_permissions() {
        let reference = r#"{"can_send_messages":true}"#;
        expand_basic_test! {
            fn run_test(ChatPermissions, reference)
        }
    }

    #[test]
    fn test_bot_command() {
        let reference = r#"{"command":"do_it","description":"Lets do it"}"#;
        expand_basic_test! {
            fn run_test(BotCommand, reference)
        }
    }

    #[test]
    fn test_response_parameters() {
        let reference = r#"{"migrate_to_chat_id":1,"retry_after":120}"#;
        expand_basic_test! {
            fn run_test(ResponseParameters, reference)
        }
    }

    #[test]
    fn test_photo_size() {
        let reference =
            r#"{"file_id":"1","file_unique_id":"1234","width":800,"height":600,"file_size":1024}"#;
        expand_basic_test! {
            fn run_test(PhotoSize, reference)
        }
    }

    #[test]
    fn test_animation() {
        let reference = r#"{"file_id":"1","file_unique_id":"12345","width":600,"height":800,"duration":10,"thumb":{"file_id":"1","file_unique_id":"1234","width":800,"height":600}}"#;
        expand_basic_test! {
            fn run_test(Animation, reference)
        }
    }

    #[test]
    fn test_audio() {
        let reference = r#"{"file_id":"1","file_unique_id":"12345","duration":60}"#;
        expand_basic_test! {
            fn run_test(Audio, reference)
        }
    }

    #[test]
    fn test_document() {
        let reference = r#"{"file_id":"1","file_unique_id":"12345"}"#;
        expand_basic_test! {
            fn run_test(Document, reference)
        }
    }

    #[test]
    fn test_video() {
        let reference =
            r#"{"file_id":"1","file_unique_id":"12345","width":800,"height":600,"duration":24}"#;
        expand_basic_test! {
            fn run_test(Video, reference)
        }
    }

    #[test]
    fn test_video_note() {
        let reference = r#"{"file_id":"1","file_unique_id":"12345","length":120,"duration":10}"#;
        expand_basic_test! {
            fn run_test(VideoNote, reference)
        }
    }

    #[test]
    fn test_voice() {
        let reference = r#"{"file_id":"1","file_unique_id":"12345","duration":5}"#;
        expand_basic_test! {
            fn run_test(Voice, reference)
        }
    }

    #[test]
    fn test_contact() {
        let reference = r#"{"phone_number":"01234","first_name":"me","user_id":1234567890}"#;
        expand_basic_test! {
            fn run_test(Contact, reference)
        }
    }

    #[test]
    fn test_dice() {
        let reference = r#"{"emoji":"dice","value":4}"#;
        expand_basic_test! {
            fn run_test(Dice, reference)
        }
    }

    #[test]
    fn test_poll_option() {
        let reference = r#"{"text":"nein","voter_count":3}"#;
        expand_basic_test! {
            fn run_test(PollOption, reference)
        }
    }

    #[test]
    fn test_poll_answer() {
        let reference = r#"{"poll_id":"01234","user":{"id":123654,"is_bot":true,"first_name":"me"},"option_ids":[1,2,3,4,5]}"#;
        expand_basic_test! {
            fn run_test(PollAnswer, reference)
        }
    }

    #[test]
    fn test_poll() {
        let reference = r#"{"id":"1234","question":"right?","options":[{"text":"nein","voter_count":3},{"text":"ja","voter_count":4}],"total_voter_count":7,"is_closed":true,"is_anonymous":false,"type":"regular","allows_multiple_answers":false,"explanation_entities":[{"type":"mention","offset":10,"length":20},{"type":"cashtag","offset":1,"length":2}]}"#;
        expand_basic_test! {
            fn run_test(Poll, reference)
        }
    }

    #[test]
    fn test_location() {
        let reference = r#"{"longitude":49.5,"latitude":9.4,"horizontal_accuracy":0.2}"#;
        expand_basic_test! {
            fn run_test(Location, reference)
        }
    }

    #[test]
    fn test_venue() {
        let reference =
            r#"{"location":{"longitude":49.5,"latitude":9.4},"title":"home","address":"at home"}"#;
        expand_basic_test! {
            fn run_test(Venue, reference)
        }
    }

    #[test]
    fn test_proximity_alert_triggered() {
        let reference = r#"{"traveler":{"id":123654,"is_bot":true,"first_name":"travel"},"watcher":{"id":123654,"is_bot":true,"first_name":"watch"},"distance":100}"#;
        expand_basic_test! {
            fn run_test(ProximityAlertTriggered, reference)
        }
    }

    #[test]
    fn test_message_id() {
        let reference = r#"{"message_id":12334}"#;
        expand_basic_test! {
            fn run_test(MessageId, reference)
        }
    }

    #[test]
    fn test_message_auto_delete_timer_changed() {
        let reference = r#"{"message_auto_delete_time":100}"#;
        expand_basic_test! {
            fn run_test(MessageAutoDeleteTimerChanged, reference)
        }
    }

    #[test]
    fn test_voice_chat_scheduled() {
        let reference = r#"{"start_date":100}"#;
        expand_basic_test! {
            fn run_test(VoiceChatScheduled, reference)
        }
    }

    #[test]
    fn test_voice_started() {
        let reference = r#"{}"#;
        expand_basic_test! {
            fn run_test(VoiceChatStarted, reference)
        }
    }

    #[test]
    fn test_voice_chat_ended() {
        let reference = r#"{"duration":100}"#;
        expand_basic_test! {
            fn run_test(VoiceChatEnded, reference)
        }
    }

    #[test]
    fn test_voice_chat_participants_invited() {
        let reference = r#"{"users":[{"id":123654,"is_bot":true,"first_name":"user1"},{"id":12365,"is_bot":true,"first_name":"user2"}]}"#;
        expand_basic_test! {
            fn run_test(VoiceChatParticipantsInvited, reference)
        }
    }

    #[test]
    fn test_user_profile_photos() {
        let reference = r#"{"total_count":2,"photos":[[{"file_id":"1","file_unique_id":"1234","width":800,"height":600},{"file_id":"2","file_unique_id":"1234","width":600,"height":800}],[{"file_id":"3","file_unique_id":"1234","width":800,"height":600},{"file_id":"4","file_unique_id":"1234","width":600,"height":800}]]}"#;
        expand_basic_test! {
            fn run_test(UserProfilePhotos, reference)
        }
    }

    #[test]
    fn test_file() {
        let reference = r#"{"file_id":"1","file_unique_id":"1234"}"#;
        expand_basic_test! {
            fn run_test(File, reference)
        }
    }

    #[test]
    fn test_reply_keyboard_markup() {
        let reference = r#"{"keyboard":[[{"text":"quiz1"},{"text":"quiz2"}],[{"text":"quiz3"},{"text":"quiz4"}]]}"#;
        expand_basic_test! {
            fn run_test(ReplyKeyboardMarkup, reference)
        }
    }

    #[test]
    fn test_keyboard_button() {
        let reference = r#"{"text":"quiz","request_poll":{"type":"quiz"}}"#;
        expand_basic_test! {
            fn run_test(KeyboardButton, reference)
        }
    }

    #[test]
    fn test_keyboard_button_poll_type() {
        let reference = r#"{"type":"quiz"}"#;
        expand_basic_test! {
            fn run_test(KeyboardButtonPollType, reference)
        }
    }

    #[test]
    fn test_reply_keyboard_remove() {
        let reference = r#"{"remove_keyboard":true}"#;
        expand_basic_test! {
            fn run_test(ReplyKeyboardRemove, reference)
        }
    }

    #[test]
    fn test_login_url() {
        let reference = r#"{"url":"https://its.me"}"#;
        expand_basic_test! {
            fn run_test(LoginUrl, reference)
        }
    }

    #[test]
    fn test_force_reply() {
        let reference = r#"{"force_reply":true}"#;
        expand_basic_test! {
            fn run_test(ForceReply, reference)
        }
    }

    #[test]
    fn test_chat_location() {
        let reference = r#"{"location":{"longitude":49.5,"latitude":9.4},"address":"home"}"#;
        expand_basic_test! {
            fn run_test(ChatLocation, reference)
        }
    }

    #[test]
    fn test_input_media() {
        let reference = r#"{"type":"video","media":"dummy"}"#;
        expand_basic_test! {
            fn run_test(InputMedia, reference)
        }
    }

    #[test]
    fn test_sticker() {
        let reference = r#"{"file_id":"1","file_unique_id":"1234","width":64,"height":64,"is_animated":true,"thumb":{"file_id":"1","file_unique_id":"1234","width":800,"height":600},"mask_position":{"point":"chin","x_shift":1.1,"y_shift":2.5,"scale":2.1}}"#;
        expand_basic_test! {
            fn run_test(Sticker, reference)
        }
    }

    #[test]
    fn test_sticker_set() {
        let reference = r#"{"name":"stickerset","title":"stickers","is_animated":true,"contains_masks":false,"stickers":[{"file_id":"1","file_unique_id":"1234","width":64,"height":64,"is_animated":true},{"file_id":"2","file_unique_id":"2345","width":32,"height":32,"is_animated":false}]}"#;
        expand_basic_test! {
            fn run_test(StickerSet, reference)
        }
    }

    #[test]
    fn test_mask_position() {
        let reference = r#"{"point":"chin","x_shift":1.3,"y_shift":2.5,"scale":2.1}"#;
        expand_basic_test! {
            fn run_test(MaskPosition, reference)
        }
    }

    #[test]
    fn test_chat_member_updated() {
        let reference = r#"{"chat":{"id":1234,"type":"private"},"from":{"id":1234,"is_bot":true,"first_name":"itsme"},"date":12,"old_chat_member":{"user":{"id":1234,"is_bot":true,"first_name":"groot"},"status":"creator"},"new_chat_member":{"user":{"id":1234,"is_bot":true,"first_name":"root"},"status":"creator"}}"#;
        expand_basic_test! {
            fn run_test(ChatMemberUpdated, reference)
        }
    }

    #[test]
    fn test_callback_query() {
        let reference = r#"{"id":"1234","from":{"id":1234,"is_bot":true,"first_name":"itsme"},"message":{"message_id":10,"date":5,"chat":{"id":12,"type":"private"}}}"#;
        expand_basic_test! {
            fn run_test(CallbackQuery, reference)
        }
    }

    #[test]
    fn test_inline_keyboard_button() {
        let reference = r#"{"text":"hello","login_url":{"url":"https://example.com"}}"#;
        expand_basic_test! {
            fn run_test(InlineKeyboardButton, reference)
        }
    }

    #[test]
    fn test_inline_keyboard_markup() {
        let reference = r#"{"inline_keyboard":[[{"text":"hello1"},{"text":"hello2"}],[{"text":"hello3"},{"text":"hello4"}]]}"#;
        expand_basic_test! {
            fn run_test(InlineKeyboardMarkup, reference)
        }
    }

    #[test]
    fn test_chat() {
        let reference = r#"{"id":1,"type":"private","photo":{"small_file_id":"1","small_file_unique_id":"1234","big_file_id":"2","big_file_unique_id":"2345"},"pinned_message":{"message_id":10,"date":5,"chat":{"id":12,"type":"private"}},"permissions":{"can_send_messages":true},"location":{"location":{"longitude":49.1,"latitude":10.2},"address":"here"}}"#;
        expand_basic_test! {
            fn run_test(Chat, reference)
        }
    }

    #[test]
    fn test_message() {
        let reference = r#"{"message_id":32,"from":{"id":1234,"is_bot":true,"first_name":"itsme"},"sender_chat":{"id":12345,"type":"group"},"date":5,"chat":{"id":12,"type":"private"},"reply_to_message":{"message_id":10,"date":5,"chat":{"id":12,"type":"private"}}}"#;
        expand_basic_test! {
            fn run_test(Message, reference)
        }
    }

    #[test]
    fn test_update() {
        let reference = r#"{"update_id":10,"message":{"message_id":10,"date":5,"chat":{"id":12,"type":"private"}},"poll_answer":{"poll_id":"test","user":{"id":13,"is_bot":false,"first_name":"user"},"option_ids":[0,1,2]}}"#;
        expand_basic_test! {
            fn run_test(Update, reference)
        }
    }
}
