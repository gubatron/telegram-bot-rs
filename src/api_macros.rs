//! These macros are only used to create all objects easily as well as simplifying testing and creating methods.
//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// stuff for objects
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! vec_to_json_array {
    ($($fname:ident($name:ident))*) => {
       $(fn $fname(v: Vec<$name>) -> JsonValue {
            let mut tmp: Vec<JsonValue> = vec![];
            for value in v {
                tmp.push(Custom::to_json(value));
            }
            let mut t = json::array![];
            for x in tmp {
                t.push(x).ok();
            }
            t
        })*
    }
}

#[macro_export]
macro_rules! vec_vec_to_json_array {
    ($($fname:ident($name:ident, $vec_func:ident))*) => {
       $(fn $fname(v: Vec<Vec<$name>>) -> JsonValue {
            let mut tmp: Vec<JsonValue> = vec![];
            for value in v {
                tmp.push($vec_func(value));
            }
            let mut t = json::array![];
            for x in tmp {
                t.push(x).ok();
            }
            t
        })*
    }
}

#[macro_export]
macro_rules! add_functionality {
    ($(pub struct $name:ident { $(pub $fname:ident : $ftype:ty), *})*) => {
        $(pub struct $name {
            $(pub $fname : $ftype), *
        }

        impl $name {
            pub fn from_json(data: JsonValue) -> $name {
                let mut new = $name::empty();
                $(
                if stringify!($fname) == "typ" {
                    new.$fname = Custom::from_json(data["type"].clone());
                } else {
                    new.$fname = Custom::from_json(data[stringify!($fname)].clone());
                }); *;
                new
            }

             pub fn to_json(&self) -> JsonValue {
                let mut j = json::object! {};
                $(if stringify!($fname) == "typ" {
                    j = Custom::create_json(j, self.$fname.clone(), "type")
                } else {
                    j = Custom::create_json(j, self.$fname.clone(), stringify!($fname))
                }); *;
                j
            }

            pub fn empty() -> $name {
                $name {
                    $($fname: Custom::default(), )*
                }
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut data = vec![];
                $(if stringify!($fname) == "typ" {
                    data = Custom::push(data, self.$fname.clone(), "type")
                } else {
                    data = Custom::push(data, self.$fname.clone(), stringify!($fname))
                }); *;
                write!(f, "{}", data.join("; "))
            }
        }
        impl Clone for $name {
            fn clone(&self) -> Self {
                $name {
                    $($fname: self.$fname.clone()), *
                }
            }
        })*
    }
}

#[macro_export]
macro_rules! add_functionality_empty {
    (pub struct $name:ident {}) => {
        pub struct $name {}

        impl $name {
            pub fn from_json(_data: JsonValue) -> $name {
                $name {}
            }

            pub fn to_json(&self) -> JsonValue {
                JsonValue::new_object()
            }

            pub fn empty() -> $name {
                $name {}
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", "{}")
            }
        }
        impl Clone for $name {
            fn clone(&self) -> Self {
                $name {}
            }
        }
    };
}

#[macro_export]
macro_rules! custom_from_json {
    (fn from_json(s: JsonValue, $func:ident, $extract:ident) -> $fname:ident) => {
        fn from_json(s: JsonValue) -> $fname {
            s.$func().$extract()
        }
    };
}

#[macro_export]
macro_rules! custom_push {
    (fn push()) => {
        fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
            data.push(format!("{}: {}", name, v));
            data
        }
    };
}

#[macro_export]
macro_rules! custom_default {
    (fn default($default:expr)) => {
        fn default() -> Self {
            $default
        }
    };
}

#[macro_export]
macro_rules! custom_url_encode {
    (fn url_encode(v: Self)) => {
        fn url_encode(v: Self) -> String {
            urlencoding::encode(&*format!("{}", Custom::to_json(v))).to_string()
        }
    };
}

#[macro_export]
macro_rules! expand_custom {
    ($(impl Custom for $fname:ident ($func:ident, $extract:ident, $default:expr))*) => {
        $(impl Custom for $fname {
            custom_from_json! {
                fn from_json(s: JsonValue, $func, $extract) -> $fname
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str)  -> JsonValue {
                j.insert(name, format!("{}", v)).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                JsonValue::String(format!("{}", v))
            }
            custom_push! {
                fn push()
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_direct_i {
    ($(impl Custom for $fname:ident ($func:ident, $extract:ident, $default:expr))*) => {
        $(impl Custom for $fname {
            custom_from_json! {
                fn from_json(s: JsonValue, $func, $extract) -> $fname
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str)  -> JsonValue {
                j.insert(name, v).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                JsonValue::Number(json::number::Number::from(v as i64))
            }
            custom_push! {
                fn push()
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_direct_bool {
    ($(impl Custom for $fname:ident ($func:ident, $extract:ident, $default:expr))*) => {
        $(impl Custom for $fname {
            custom_from_json! {
                fn from_json(s: JsonValue, $func, $extract) -> $fname
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str)  -> JsonValue {
                j.insert(name, v).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                JsonValue::Boolean(v)
            }
            custom_push! {
                fn push()
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_direct_object {
    ($(impl Custom for $fname:ident ($func:ident, $extract:ident, $default:expr))*) => {
        $(impl Custom for $fname {
            custom_from_json! {
                fn from_json(s: JsonValue, $func, $extract) -> $fname
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str)  -> JsonValue {
                j.insert(name, v).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                v.to_json()
            }
            custom_push! {
                fn push()
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_vec {
    ($(impl Custom for Vec<$fname:ident> ($func:ident, $extract:ident, $default:expr, $to_json_array:ident))*) => {
        $(impl Custom for Vec<$fname> {
            fn from_json(s: JsonValue) -> Vec<$fname>{
                if !s.is_array() { [].to_vec() }
                else {s.$func().$extract()}
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str)  -> JsonValue {
                j.insert(name, $to_json_array(v)).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                JsonValue::String(format!("{}", $to_json_array(v)))
            }
            fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
                data.push(format!("{}: {}", name, $to_json_array(v)));
                data
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_vec_vec {
    ($(impl Custom for Vec<Vec<$fname:ident>> ($func:ident, $extract:ident, $default:expr, $to_json_array:ident))*) => {
        $(impl Custom for Vec<Vec<$fname>> {
            fn from_json(s: JsonValue) -> Vec<Vec<$fname>> {
                if !s.is_array() { $default }
                else {s.$func().$extract()}
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str)  -> JsonValue {
                j.insert(name, $to_json_array(v)).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                JsonValue::String(format!("{}", $to_json_array(v)))
            }
            fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
                data.push(format!("{}: {}", name, $to_json_array(v)));
                data
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_option {
    ($(impl Custom for Option<$fname:ident> ($func:ident, $extract:ident, $default: expr))*) => {
        $(impl Custom for Option<$fname> {
            fn from_json(s: JsonValue) -> Option<$fname> {
                if s.is_empty() && s.as_bool() != Some(false) { None }
                else {Some(s.$func().$extract())}
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str) -> JsonValue {
                match v.clone() {
                    Some(value) => {j.insert(name, value).ok();},
                    _ => ()
                };
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                match v.clone() {
                    Some(value) => JsonValue::String(format!("{}", value)),
                    _ => JsonValue::Null
                }

            }
            fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
                match v.clone() {Some(value) => data.push(format!("{}: {}", name, value)), _ => () };
                data
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_box {
    ($(impl Custom for Box<$fname:ident> ($func:ident, $extract:ident, $default: expr))*) => {
        $(impl Custom for Box<$fname> {
            fn from_json(s: JsonValue) -> Box<$fname> {
                if s.is_empty() && s.as_bool() != Some(false) { Box::new($fname::empty()) }
                else {s.$func().$extract()}
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str) -> JsonValue {
                j.insert(name, *v).ok();
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                JsonValue::String(format!("{}", *v))
            }
            fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
                data.push(format!("{}: {}", name, *v));
                data
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_option_box {
    ($(impl Custom for Option<Box<$fname:ident>> ($func:ident, $extract:ident, $default: expr))*) => {
        $(impl Custom for Option<Box<$fname>> {
            fn from_json(s: JsonValue) -> Option<Box<$fname>> {
                if s.is_empty() && s.as_bool() != Some(false) { None }
                else {Some(s.$func().$extract())}
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str) -> JsonValue {
                match v.clone() {
                    Some(value) => {j.insert(name, *value).ok();},
                    _ => ()
                };
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                match v.clone() {
                    Some(value) => JsonValue::String(format!("{}", *value)),
                    _ => JsonValue::Null
                }

            }
            fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
                match v.clone() {Some(value) => data.push(format!("{}: {}", name, *value)), _ => () };
                data
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_custom_option_vec {
    ($(impl Custom for Option<Vec<$fname:ident>> ($func:ident, $extract:ident, $default: expr, $to_json_array:ident))*) => {
        $(impl Custom for Option<Vec<$fname>> {
            fn from_json(s: JsonValue) -> Option<Vec<$fname>> {
                if !s.is_array() { None }
                else {Some(s.$func().$extract())}
            }
            fn create_json(mut j: JsonValue, v: Self, name: &'static str) -> JsonValue {
                match v.clone() {
                    Some(value) => {
                        j.insert(name, $to_json_array(value)).ok();
                    },
                    _ => ()
                };
                j
            }
            fn to_json(v: Self)  -> JsonValue {
                match v.clone() {
                    Some(value) => JsonValue::String(format!("{}", $to_json_array(value))),
                    _ => JsonValue::Null
                }

            }
            fn push(mut data: Vec<String>, v: Self, name: &'static str) -> Vec<String> {
                match v.clone() {
                    Some(value) => {data.push(format!("{}: {:?}", name, $to_json_array(value)));},
                    _ => ()
                };
                data
            }
            custom_default! {
                fn default($default)
            }
            custom_url_encode! {
                fn url_encode(v: Self)
            }
        })*
    }
}

#[macro_export]
macro_rules! as_custom {
    ($(fn $fname:ident(&self) -> Option<$ftype:ident>)*) => {
        $(fn $fname(&self) -> Option<$ftype> {
            if self.is_empty() { None }
            else { Some($ftype::from_json(self.clone())) }
        })*
    }
}

#[macro_export]
macro_rules! as_box_custom {
    ($(fn $fname:ident(&self) -> Option<Box<$ftype:ident>>)*) => {
        $(fn $fname(&self) -> Option<Box<$ftype>> {
            if self.is_empty() { None }
            else { Some(Box::new($ftype::from_json(self.clone()))) }
        })*
    }
}

#[macro_export]
macro_rules! as_vec_custom {
    ($(fn $fname:ident(&self) -> Option<Vec<$ftype:ident>>)*) => {
        $(fn $fname(&self) -> Option<Vec<$ftype>> {
            if !self.is_array() { None }
            else {
                let mut ret: Vec<$ftype> = vec![];
                let mut tmp: Vec<$ftype> = vec![];
                let mut s = self.clone();
                while !s.is_empty() {
                    let t = s.pop();
                    tmp.push($ftype::from_json(t))
                }
                while !tmp.is_empty() {
                    ret.push(tmp.pop().unwrap())
                }
                Some(ret)
            }
        })*
    }
}

#[macro_export]
macro_rules! as_vec_vec_custom {
    ($(fn $fname:ident(&self, $vec_func:ident) -> Option<Vec<Vec<$ftype:ident>>>)*) => {
        $(fn $fname(&self) -> Option<Vec<Vec<$ftype>>> {
            if !self.is_array() { None }
            else {
                let mut ret: Vec<Vec<$ftype>> = vec![];
                let mut tmp: Vec<Vec<$ftype>> = vec![];
                let mut s = self.clone();
                while !s.is_empty() {
                    let t = s.pop();
                    tmp.push(t.$vec_func().unwrap())
                }
                while !tmp.is_empty() {
                    ret.push(tmp.pop().unwrap())
                }
                Some(ret)
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_from {
    ($(impl From<$fname:ident> for JsonValue)*) => {
        $(impl From<$fname> for JsonValue {
            fn from(u: $fname) -> Self {
                u.to_json()
            }
        })*
    }
}

#[macro_export]
macro_rules! expand_basic_test {
    (fn run_test($fname:ident, $reference:expr)) => {
        let json_me = json::parse($reference);
        let me;
        match json_me {
            Ok(json_data) => me = $fname::from_json(json_data),
            Err(_) => me = $fname::empty(),
        }
        let actual = format!("{}", me.to_json());
        assert_eq!(actual, $reference);
    };
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// stuff for methods
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! expand_parameters_into_string {
    ($parameters:ident, $($vname: ident), *) => {
        $($ parameters.push_str(&*format!("{}={}&", stringify!($vname), Custom::url_encode($vname))));*
    }
}

#[macro_export]
macro_rules! expand_parameters_opt_into_string {
    ($parameters:ident, $($vname: ident), *) => {
        $(match $vname {
            Some(v) => {
                if stringify!($vname) == "typ" {
                    $parameters.push_str(&*format!("typ={}&", Custom::url_encode(v)))
                } else {
                    $parameters.push_str(&*format!("{}={}&", stringify!($vname),Custom::url_encode(v)))
                }},
            None => ()
        });*
    }
}

#[macro_export]
macro_rules! expand_parameters_reply_markup_into_string {
    ($parameters:ident, $($vname: ident), *) => {
        $(match $vname { Some(v) => $parameters.push_str(&*format!("reply_markup={}&", Custom::url_encode(v))), None => () });*
    }
}

#[macro_export]
macro_rules! expand_make_request_to_message {
    ($res: ident) => {
        if !$res["ok"].as_bool().unwrap_or(false) {
            None
        } else {
            let ret: Message = Custom::from_json($res["result"].clone());
            Some(ret)
        }
    };
}

#[macro_export]
macro_rules! expand_make_request_to_bool {
    ($res: ident) => {
        if !$res["ok"].as_bool().unwrap_or(false) {
            false
        } else {
            let ret: bool = Custom::from_json($res["result"].clone());
            ret
        }
    };
}
