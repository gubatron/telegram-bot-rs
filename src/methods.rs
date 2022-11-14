//! This module contains all available methods to use the Bot API</br>
//! To get started, you first have to create a bot
//! ```ignore
//! let bot = Bot::new("your_bot_api_key".to_json());
//! ```
//! Afterwards you can use the bot, e.g.:
//! ```ignore
//! bot.get_me();
//! ```

extern crate json;
extern crate rustc_serialize;
extern crate urlencoding;

use crate::objects::*;
use crate::*;
use json::JsonValue;
use std::sync::mpsc::Sender;
use std::thread;

pub struct Bot {
    key: String,
    offset: i64,
}

const BASE_URL: &str = "https://api.telegram.org/bot";

impl Bot {
    pub fn new<'a>(api_key: String) -> Bot {
        Bot {
            key: api_key,
            offset: 0,
        }
    }

    /// start_polling is used to get updates from the bot.
    /// ```ignore
    /// let (tx, rx) = mpsc::channel();
    /// bot.start_polling(tx);
    /// let received = rx.recv().unwrap();
    /// println!("Got: {}", received);
    /// ```

    pub fn start_polling(&mut self, tx: Sender<Update>) {
        let mut bot = self.clone();
        thread::spawn(move || loop {
            let updates = bot.get_updates(None, None, None);
            match updates {
                Some(us) => {
                    for u in us {
                        assert!(tx.send(u).is_ok());
                    }
                }
                None => (),
            };
        });
    }

    fn send_request(&self, method: String, parameters: String) -> JsonValue {
        let request = format!("{}{}/{}?{}", BASE_URL, self.key, method, parameters);
        // println!("{}", request);
        let res = reqwest::blocking::get(request);
        let mut json_response = JsonValue::Null;
        match res {
            Ok(r) => match r.text() {
                Ok(result) => json_response = json::parse(&*result).unwrap(),
                Err(_) => (),
            },
            Err(_) => (),
        }
        json_response
    }

    pub fn get_updates(
        &mut self,
        limit: Option<i32>,
        timeout: Option<i32>,
        allowed_updates: Option<Vec<String>>,
    ) -> Option<Vec<Update>> {
        let mut parameters = "".to_string();
        parameters.push_str(&*format!("offset={}&", self.offset));
        match limit {
            Some(o) => parameters.push_str(&*format!("limit={}&", Custom::to_json(o))),
            None => (),
        }
        match timeout {
            Some(o) => parameters.push_str(&*format!("timeout={}&", Custom::to_json(o))),
            None => (),
        }
        match allowed_updates {
            Some(o) => parameters.push_str(&*format!("allowed_updates={}&", Custom::to_json(o))),
            None => (),
        }
        parameters.pop();
        let res = self.send_request("getUpdates".to_string(), parameters);
        let ret: Vec<Update> = Custom::from_json(res["result"].clone());
        if !res["ok"].as_bool().unwrap() || ret.len() == 0 {
            None
        } else {
            self.offset = ret[ret.len() - 1].clone().update_id + 1;
            self.send_request("getUpdates".to_string(), format!("offset={}", self.offset));
            Some(ret)
        }
    }

    pub fn get_me(&mut self) -> User {
        let res = self.send_request("getMe".to_string(), "".to_string());
        if !res["ok"].as_bool().unwrap() {
            User::empty()
        } else {
            let ret: User = Custom::from_json(res["result"].clone());
            ret
        }
    }

    pub fn send_message(
        &mut self,
        chat_id: i64,
        text: String,
        parse_mode: Option<String>,
        entities: Option<Vec<MessageEntity>>,
        disable_web_page_preview: Option<bool>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, text
        }
        expand_parameters_opt_into_string! {
            parameters, parse_mode, entities, disable_web_page_preview, disable_notification, reply_to_message_id,
            allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendMessage".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn forward_message(
        &mut self,
        chat_id: i64,
        from_chat_id: i64,
        message_id: i32,
        disable_notification: Option<bool>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, from_chat_id, message_id
        }
        expand_parameters_opt_into_string! {
            parameters, disable_notification
        }
        parameters.pop();
        let res = self.send_request("ForwardMessage".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn copy_message(
        &mut self,
        chat_id: i64,
        from_chat_id: i64,
        message_id: i32,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> i32 {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, from_chat_id, message_id
        }
        expand_parameters_opt_into_string! {
            parameters, caption, parse_mode, caption_entities, disable_notification, reply_to_message_id,
            allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("copyMessage".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            0
        } else {
            let ret: i32 = Custom::from_json(res["result"].clone());
            ret
        }
    }

    pub fn send_photo(
        &mut self,
        chat_id: i64,
        photo: String,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, photo
        }
        expand_parameters_opt_into_string! {
            parameters, caption, parse_mode, caption_entities, disable_notification, reply_to_message_id,
            allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendPhoto".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_audio(
        &mut self,
        chat_id: i64,
        audio: String,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        duration: Option<i32>,
        performer: Option<String>,
        title: Option<String>,
        thumb: Option<String>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, audio
        }
        expand_parameters_opt_into_string! {
            parameters, caption, parse_mode, caption_entities, duration, performer, title, thumb, disable_notification, reply_to_message_id,
            allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendAudio".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_document(
        &mut self,
        chat_id: i64,
        document: String,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_content_type_detection: Option<bool>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, document
        }
        expand_parameters_opt_into_string! {
            parameters, thumb, caption, parse_mode, caption_entities, disable_content_type_detection, disable_notification,
            reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendDocument".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_video(
        &mut self,
        chat_id: i64,
        video: String,
        duration: Option<i32>,
        width: Option<i32>,
        height: Option<i32>,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        supports_streaming: Option<bool>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, video
        }
        expand_parameters_opt_into_string! {
            parameters, duration, width, height, thumb, caption, parse_mode, caption_entities, supports_streaming,
            disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendVideo".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_animation(
        &mut self,
        chat_id: i64,
        animation: String,
        duration: Option<i32>,
        width: Option<i32>,
        height: Option<i32>,
        thumb: Option<String>,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, animation
        }
        expand_parameters_opt_into_string! {
            parameters, duration, width, height, thumb, caption, parse_mode, caption_entities,
            disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendAnimation".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_voice(
        &mut self,
        chat_id: i64,
        voice: String,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        duration: Option<i32>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, voice
        }
        expand_parameters_opt_into_string! {
            parameters, caption, parse_mode, caption_entities, duration,
            disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendVoice".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_video_note(
        &mut self,
        chat_id: i64,
        video_note: String,
        duration: Option<i32>,
        length: Option<i32>,
        thumb: Option<String>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, video_note
        }
        expand_parameters_opt_into_string! {
            parameters, duration, length, thumb, disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendVideoNote".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_media_group(
        &mut self,
        chat_id: i64,
        media: Vec<InputMedia>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
    ) -> Option<Vec<Message>> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, media
        }
        expand_parameters_opt_into_string! {
            parameters, disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        parameters.pop();
        let res = self.send_request("sendMediaGroup".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: Vec<Message> = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn send_location(
        &mut self,
        chat_id: i64,
        latitude: f64,
        longitude: f64,
        horizontal_accuracy: Option<f64>,
        live_period: Option<i32>,
        heading: Option<i32>,
        proximity_alert_radius: Option<i32>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, latitude, longitude
        }
        expand_parameters_opt_into_string! {
            parameters, horizontal_accuracy, live_period, heading, proximity_alert_radius, disable_notification,
            reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendLocation".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn edit_message_live_location(
        &mut self,
        chat_id: Option<i64>,
        message_id: Option<i32>,
        inline_message_id: Option<String>,
        latitude: f64,
        longitude: f64,
        horizontal_accuracy: Option<f64>,
        heading: Option<i32>,
        proximity_alert_radius: Option<i32>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, latitude, longitude
        }
        expand_parameters_opt_into_string! {
            parameters, chat_id, message_id, inline_message_id, horizontal_accuracy, heading, proximity_alert_radius
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("editMessageLiveLocation".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn stop_message_live_location(
        &mut self,
        chat_id: Option<i64>,
        message_id: Option<i32>,
        inline_message_id: Option<String>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_opt_into_string! {
            parameters, chat_id, message_id, inline_message_id
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("stopMessageLiveLocation".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_venue(
        &mut self,
        chat_id: i64,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        foursquare_id: Option<String>,
        foursquare_type: Option<String>,
        google_place_id: Option<String>,
        google_place_type: Option<String>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, latitude, longitude, title, address
        }
        expand_parameters_opt_into_string! {
            parameters, foursquare_id, foursquare_type, google_place_id, google_place_type, disable_notification,
            reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendVenue".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_contact(
        &mut self,
        chat_id: i64,
        phone_number: String,
        first_name: String,
        last_name: Option<String>,
        vcard: Option<String>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, phone_number, first_name
        }
        expand_parameters_opt_into_string! {
            parameters, last_name, vcard, disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendContact".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_poll(
        &mut self,
        chat_id: i64,
        question: String,
        options: Vec<String>,
        is_anonymous: Option<bool>,
        typ: Option<String>,
        allows_multiple_answers: Option<bool>,
        correct_option_id: Option<i32>,
        explanation: Option<String>,
        explanation_parse_mode: Option<String>,
        explanation_entities: Option<Vec<MessageEntity>>,
        open_period: Option<i32>,
        close_date: Option<i32>,
        is_closed: Option<i32>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, question, options
        }
        expand_parameters_opt_into_string! {
            parameters, is_anonymous, typ, allows_multiple_answers, correct_option_id, explanation,
            explanation_parse_mode, explanation_entities, open_period, close_date, is_closed,
            disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendPoll".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_dice(
        &mut self,
        chat_id: i64,
        emoji: Option<String>,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        expand_parameters_opt_into_string! {
            parameters, emoji, disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendDice".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn send_chat_action(&mut self, chat_id: i64, action: String) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, action
        }
        parameters.pop();
        let res = self.send_request("sendChatAction".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn get_user_profile_photos(
        &mut self,
        user_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Option<UserProfilePhotos> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, user_id
        }
        expand_parameters_opt_into_string! {
            parameters, offset, limit
        }
        parameters.pop();
        let res = self.send_request("getUserProfilePhotos".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: UserProfilePhotos = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn kick_chat_member(
        &mut self,
        chat_id: i64,
        user_id: i64,
        until_date: Option<i32>,
        revoke_messages: Option<bool>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, user_id
        }
        expand_parameters_opt_into_string! {
            parameters, until_date, revoke_messages
        }
        parameters.pop();
        let res = self.send_request("kickChatMember".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn unban_chat_member(
        &mut self,
        chat_id: i64,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, user_id
        }
        expand_parameters_opt_into_string! {
            parameters, only_if_banned
        }
        parameters.pop();
        let res = self.send_request("unbanChatMember".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn restrict_chat_member(
        &mut self,
        chat_id: i64,
        user_id: i64,
        permissions: ChatPermissions,
        until_date: Option<i32>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, user_id, permissions
        }
        expand_parameters_opt_into_string! {
            parameters, until_date
        }
        parameters.pop();
        let res = self.send_request("restrictChatMember".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn promote_chat_member(
        &mut self,
        chat_id: i64,
        user_id: i64,
        is_anonymous: Option<bool>,
        can_manage_chat: Option<bool>,
        can_post_messages: Option<bool>,
        can_edit_messages: Option<bool>,
        can_delete_messages: Option<bool>,
        can_manage_voice_chats: Option<bool>,
        can_restrict_members: Option<bool>,
        can_promote_members: Option<bool>,
        can_change_info: Option<bool>,
        can_invite_users: Option<bool>,
        can_pin_messages: Option<bool>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, user_id
        }
        expand_parameters_opt_into_string! {
            parameters, is_anonymous, can_manage_chat, can_post_messages, can_edit_messages,
            can_delete_messages, can_manage_voice_chats, can_restrict_members, can_promote_members,
            can_change_info, can_invite_users, can_pin_messages
        }
        parameters.pop();
        let res = self.send_request("promoteChatMember".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn set_chat_administrator_custom_title(
        &mut self,
        chat_id: i64,
        user_id: i64,
        custom_title: String,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, user_id, custom_title
        }
        parameters.pop();
        let res = self.send_request("setChatAdministratorCustomTitle".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn set_chat_permissions(&mut self, chat_id: i64, permissions: ChatPermissions) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, permissions
        }
        parameters.pop();
        let res = self.send_request("setChatPermissions".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn export_chat_invite_link(&mut self, chat_id: i64) -> Option<String> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("exportChatInviteLink".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: String = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn create_chat_invite_link(
        &mut self,
        chat_id: i64,
        expire_date: Option<i32>,
        member_limit: Option<i32>,
    ) -> Option<ChatInviteLink> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        expand_parameters_opt_into_string! {
            parameters, expire_date, member_limit
        }
        parameters.pop();
        let res = self.send_request("createChatInviteLink".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: ChatInviteLink = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn edit_chat_invite_link(
        &mut self,
        chat_id: i64,
        invite_link: String,
        expire_date: Option<i32>,
        member_limit: Option<i32>,
    ) -> Option<ChatInviteLink> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, invite_link
        }
        expand_parameters_opt_into_string! {
            parameters, expire_date, member_limit
        }
        parameters.pop();
        let res = self.send_request("editChatInviteLink".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: ChatInviteLink = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn revoke_chat_invite_link(
        &mut self,
        chat_id: i64,
        invite_link: String,
    ) -> Option<ChatInviteLink> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, invite_link
        }
        parameters.pop();
        let res = self.send_request("revokeChatInviteLink".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: ChatInviteLink = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn delete_chat_photo(&mut self, chat_id: i64) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("deleteChatPhoto".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn set_chat_title(&mut self, chat_id: i64, title: String) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, title
        }
        parameters.pop();
        let res = self.send_request("setChatTitle".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn set_chat_description(&mut self, chat_id: i64, description: String) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, description
        }
        parameters.pop();
        let res = self.send_request("setChatDescription".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn pin_chat_message(
        &mut self,
        chat_id: i64,
        message_id: i32,
        disable_notification: Option<bool>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, message_id
        }
        expand_parameters_opt_into_string! {
            parameters, disable_notification
        }
        parameters.pop();
        let res = self.send_request("pinChatMessage".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn unpin_chat_message(&mut self, chat_id: i64, message_id: i32) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, message_id
        }
        parameters.pop();
        let res = self.send_request("unpinChatMessage".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn unpin_all_chat_message(&mut self, chat_id: i64) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("unpinAllChatMessages".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn leave_chat(&mut self, chat_id: i64) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("leaveChat".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn get_chat(&mut self, chat_id: i64) -> Option<Chat> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("getChat".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: Chat = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn get_chat_administrators(&mut self, chat_id: i64) -> Option<Vec<ChatMember>> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("getChatAdministrators".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: Vec<ChatMember> = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn get_chat_chat_members_count(&mut self, chat_id: i64) -> Option<i32> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("getChatMembersCount".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: i32 = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn get_chat_chat_member(&mut self, chat_id: i64, user_id: i64) -> Option<ChatMember> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, user_id
        }
        parameters.pop();
        let res = self.send_request("getChatMember".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: ChatMember = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn set_chat_sticker_set(&mut self, chat_id: i64, sticker_set_name: String) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, sticker_set_name
        }
        parameters.pop();
        let res = self.send_request("setChatStickerSet".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn delete_chat_sticker_set(&mut self, chat_id: i64) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id
        }
        parameters.pop();
        let res = self.send_request("deleteChatStickerSet".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn answer_callback_query(
        &mut self,
        callback_query_id: String,
        text: String,
        show_alert: Option<bool>,
        url: Option<String>,
        cache_time: Option<i32>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, callback_query_id, text
        }
        expand_parameters_opt_into_string! {
            parameters, show_alert, url, cache_time
        }
        parameters.pop();
        let res = self.send_request("answerCallbackQuery".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn set_my_commands(&mut self, commands: Vec<BotCommand>) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, commands
        }
        parameters.pop();
        let res = self.send_request("setMyCommands".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn get_my_commands(&mut self) -> Option<Vec<BotCommand>> {
        let res = self.send_request("getMyCommands".to_string(), "".to_string());
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: Vec<BotCommand> = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn edit_message_text(
        &mut self,
        chat_id: Option<i64>,
        message_id: Option<i32>,
        inline_message_id: Option<String>,
        text: Option<String>,
        parse_mode: Option<String>,
        entities: Option<Vec<MessageEntity>>,
        disable_web_page_preview: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_opt_into_string! {
            parameters, chat_id, message_id, inline_message_id, text, parse_mode, entities, disable_web_page_preview
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("editMessageText".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn edit_message_caption(
        &mut self,
        chat_id: Option<i64>,
        message_id: Option<i32>,
        inline_message_id: Option<String>,
        caption: Option<String>,
        parse_mode: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_opt_into_string! {
            parameters, chat_id, message_id, inline_message_id, caption, parse_mode, caption_entities
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("editMessageCaption".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn edit_message_media(
        &mut self,
        chat_id: Option<i64>,
        message_id: Option<i32>,
        inline_message_id: Option<String>,
        media: Option<InputMedia>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_opt_into_string! {
            parameters, chat_id, message_id, inline_message_id, media
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("editMessageMedia".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn edit_message_reply_markup(
        &mut self,
        chat_id: Option<i64>,
        message_id: Option<i32>,
        inline_message_id: Option<String>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_opt_into_string! {
            parameters, chat_id, message_id, inline_message_id
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("editMessageReplyMarkup".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn stop_poll(
        &mut self,
        chat_id: i64,
        message_id: i32,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
    ) -> Option<Poll> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, message_id
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm
        }
        parameters.pop();
        let res = self.send_request("stopPoll".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: Poll = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn delete_message(&mut self, chat_id: i64, message_id: i32) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, message_id
        }
        parameters.pop();
        let res = self.send_request("deleteMessage".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn send_sticker(
        &mut self,
        chat_id: i64,
        sticker: String,
        disable_notification: Option<bool>,
        reply_to_message_id: Option<i32>,
        allow_sending_without_reply: Option<bool>,
        reply_markup_ikm: Option<InlineKeyboardMarkup>,
        reply_markup_rkm: Option<ReplyKeyboardMarkup>,
        reply_markup_rkr: Option<ReplyKeyboardRemove>,
        reply_markup_fr: Option<ForceReply>,
    ) -> Option<Message> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, chat_id, sticker
        }
        expand_parameters_opt_into_string! {
            parameters, disable_notification, reply_to_message_id, allow_sending_without_reply
        }
        expand_parameters_reply_markup_into_string! {
            parameters, reply_markup_ikm, reply_markup_rkm, reply_markup_rkr, reply_markup_fr
        }
        parameters.pop();
        let res = self.send_request("sendSticker".to_string(), parameters);
        expand_make_request_to_message! {
            res
        }
    }

    pub fn get_sticker_set(&mut self, name: String) -> Option<StickerSet> {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, name
        }
        parameters.pop();
        let res = self.send_request("getStickerSet".to_string(), parameters);
        if !res["ok"].as_bool().unwrap() {
            None
        } else {
            let ret: StickerSet = Custom::from_json(res["result"].clone());
            Some(ret)
        }
    }

    pub fn create_new_sticker_set(
        &mut self,
        user_id: i64,
        name: String,
        title: String,
        png_sticker: Option<String>,
        emojis: String,
        contains_masks: Option<bool>,
        mask_position: Option<MaskPosition>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, user_id, name, title, emojis
        }
        expand_parameters_opt_into_string! {
            parameters, png_sticker, contains_masks, mask_position
        }
        parameters.pop();
        let res = self.send_request("createNewStickerSet".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn add_sticker_to_set(
        &mut self,
        user_id: i64,
        name: String,
        png_sticker: Option<String>,
        emojis: String,
        mask_position: Option<MaskPosition>,
    ) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, user_id, name, emojis
        }
        expand_parameters_opt_into_string! {
            parameters, png_sticker, mask_position
        }
        parameters.pop();
        let res = self.send_request("addStickerToSet".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn add_sticker_position_in_set(&mut self, sticker: String, position: i32) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, sticker, position
        }
        parameters.pop();
        let res = self.send_request("setStickerPositionInSet".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn delete_sticker_from_chat(&mut self, sticker: String) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, sticker
        }
        parameters.pop();
        let res = self.send_request("deleteStickerFromSet".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }

    pub fn set_sticker_thumb(&mut self, name: String, user_id: i64, thumb: Option<String>) -> bool {
        let mut parameters = "".to_string();
        expand_parameters_into_string! {
            parameters, name, user_id
        }
        expand_parameters_opt_into_string! {
            parameters, thumb
        }
        parameters.pop();
        let res = self.send_request("setStickerSetThumb".to_string(), parameters);
        expand_make_request_to_bool! {
            res
        }
    }
}

impl Clone for Bot {
    fn clone(&self) -> Self {
        Bot {
            key: self.key.clone(),
            offset: self.offset,
        }
    }
}
