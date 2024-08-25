// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `bot_management.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:QueryStatus)
pub struct QueryStatus {
    // message fields
    // @@protoc_insertion_point(field:QueryStatus.success)
    pub success: bool,
    // special fields
    // @@protoc_insertion_point(special_field:QueryStatus.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QueryStatus {
    fn default() -> &'a QueryStatus {
        <QueryStatus as ::protobuf::Message>::default_instance()
    }
}

impl QueryStatus {
    pub fn new() -> QueryStatus {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "success",
            |m: &QueryStatus| { &m.success },
            |m: &mut QueryStatus| { &mut m.success },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QueryStatus>(
            "QueryStatus",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QueryStatus {
    const NAME: &'static str = "QueryStatus";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.success = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.success != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.success != false {
            os.write_bool(1, self.success)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> QueryStatus {
        QueryStatus::new()
    }

    fn clear(&mut self) {
        self.success = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QueryStatus {
        static instance: QueryStatus = QueryStatus {
            success: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QueryStatus {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QueryStatus").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QueryStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryStatus {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:EnableStatus)
pub struct EnableStatus {
    // message fields
    // @@protoc_insertion_point(field:EnableStatus.domain)
    pub domain: ::std::string::String,
    // @@protoc_insertion_point(field:EnableStatus.enabled)
    pub enabled: bool,
    // special fields
    // @@protoc_insertion_point(special_field:EnableStatus.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnableStatus {
    fn default() -> &'a EnableStatus {
        <EnableStatus as ::protobuf::Message>::default_instance()
    }
}

impl EnableStatus {
    pub fn new() -> EnableStatus {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "domain",
            |m: &EnableStatus| { &m.domain },
            |m: &mut EnableStatus| { &mut m.domain },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "enabled",
            |m: &EnableStatus| { &m.enabled },
            |m: &mut EnableStatus| { &mut m.enabled },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnableStatus>(
            "EnableStatus",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnableStatus {
    const NAME: &'static str = "EnableStatus";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.domain = is.read_string()?;
                },
                16 => {
                    self.enabled = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.domain.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.domain);
        }
        if self.enabled != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.domain.is_empty() {
            os.write_string(1, &self.domain)?;
        }
        if self.enabled != false {
            os.write_bool(2, self.enabled)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> EnableStatus {
        EnableStatus::new()
    }

    fn clear(&mut self) {
        self.domain.clear();
        self.enabled = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnableStatus {
        static instance: EnableStatus = EnableStatus {
            domain: ::std::string::String::new(),
            enabled: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnableStatus {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnableStatus").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnableStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnableStatus {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:BotChange)
pub struct BotChange {
    // message fields
    // @@protoc_insertion_point(field:BotChange.domain)
    pub domain: ::std::string::String,
    // @@protoc_insertion_point(field:BotChange.to_change)
    pub to_change: ::protobuf::EnumOrUnknown<Bot>,
    // special fields
    // @@protoc_insertion_point(special_field:BotChange.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BotChange {
    fn default() -> &'a BotChange {
        <BotChange as ::protobuf::Message>::default_instance()
    }
}

impl BotChange {
    pub fn new() -> BotChange {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "domain",
            |m: &BotChange| { &m.domain },
            |m: &mut BotChange| { &mut m.domain },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "to_change",
            |m: &BotChange| { &m.to_change },
            |m: &mut BotChange| { &mut m.to_change },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BotChange>(
            "BotChange",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BotChange {
    const NAME: &'static str = "BotChange";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.domain = is.read_string()?;
                },
                16 => {
                    self.to_change = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.domain.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.domain);
        }
        if self.to_change != ::protobuf::EnumOrUnknown::new(Bot::Googlebot) {
            my_size += ::protobuf::rt::int32_size(2, self.to_change.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.domain.is_empty() {
            os.write_string(1, &self.domain)?;
        }
        if self.to_change != ::protobuf::EnumOrUnknown::new(Bot::Googlebot) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.to_change))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BotChange {
        BotChange::new()
    }

    fn clear(&mut self) {
        self.domain.clear();
        self.to_change = ::protobuf::EnumOrUnknown::new(Bot::Googlebot);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BotChange {
        static instance: BotChange = BotChange {
            domain: ::std::string::String::new(),
            to_change: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BotChange {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BotChange").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BotChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BotChange {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:Bot)
pub enum Bot {
    // @@protoc_insertion_point(enum_value:Bot.Googlebot)
    Googlebot = 0,
    // @@protoc_insertion_point(enum_value:Bot.Bingbot)
    Bingbot = 1,
    // @@protoc_insertion_point(enum_value:Bot.UptimeRobot)
    UptimeRobot = 2,
}

impl ::protobuf::Enum for Bot {
    const NAME: &'static str = "Bot";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Bot> {
        match value {
            0 => ::std::option::Option::Some(Bot::Googlebot),
            1 => ::std::option::Option::Some(Bot::Bingbot),
            2 => ::std::option::Option::Some(Bot::UptimeRobot),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Bot] = &[
        Bot::Googlebot,
        Bot::Bingbot,
        Bot::UptimeRobot,
    ];
}

impl ::protobuf::EnumFull for Bot {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("Bot").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Bot {
    fn default() -> Self {
        Bot::Googlebot
    }
}

impl Bot {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Bot>("Bot")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14bot_management.proto\"'\n\x0bQueryStatus\x12\x18\n\x07success\x18\
    \x01\x20\x01(\x08R\x07success\"@\n\x0cEnableStatus\x12\x16\n\x06domain\
    \x18\x01\x20\x01(\tR\x06domain\x12\x18\n\x07enabled\x18\x02\x20\x01(\x08\
    R\x07enabled\"F\n\tBotChange\x12\x16\n\x06domain\x18\x01\x20\x01(\tR\x06\
    domain\x12!\n\tto_change\x18\x02\x20\x01(\x0e2\x04.BotR\x08toChange*2\n\
    \x03Bot\x12\r\n\tGooglebot\x10\0\x12\x0b\n\x07Bingbot\x10\x01\x12\x0f\n\
    \x0bUptimeRobot\x10\x022\x8d\x01\n\rBotManagement\x121\n\x12ChangeEnable\
    Status\x12\r.EnableStatus\x1a\x0c.QueryStatus\x12\"\n\x06AddBot\x12\n.Bo\
    tChange\x1a\x0c.QueryStatus\x12%\n\tRemoveBot\x12\n.BotChange\x1a\x0c.Qu\
    eryStatusJ\xbd\x05\n\x06\x12\x04\0\0\x1a\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\n\n\x02\x06\0\x12\x04\x02\0\x06\x01\n\n\n\x03\x06\0\x01\x12\x03\
    \x02\x08\x15\n\x0b\n\x04\x06\0\x02\0\x12\x03\x03\x02=\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03\x03\x06\x18\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x03\
    \x19%\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x030;\n\x0b\n\x04\x06\0\x02\
    \x01\x12\x03\x04\x02.\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x04\x06\x0c\
    \n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x04\r\x16\n\x0c\n\x05\x06\0\x02\
    \x01\x03\x12\x03\x04!,\n\x0b\n\x04\x06\0\x02\x02\x12\x03\x05\x021\n\x0c\
    \n\x05\x06\0\x02\x02\x01\x12\x03\x05\x06\x0f\n\x0c\n\x05\x06\0\x02\x02\
    \x02\x12\x03\x05\x10\x19\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x05$/\n\n\
    \n\x02\x04\0\x12\x04\x08\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x13\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x02\x13\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\t\x02\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x07\x0e\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\t\x11\x12\n\n\n\x02\x04\x01\x12\x04\x0c\0\
    \x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0c\x08\x14\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\r\x02\x14\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\r\x02\x08\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\t\x0f\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03\r\x12\x13\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0e\x02\x13\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0e\x02\x06\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x0e\x07\x0e\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x0e\x11\x12\n\n\n\x02\x04\x02\x12\x04\x11\0\x14\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x11\x08\x11\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x02\x14\n\
    \x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\x02\x02\
    \0\x01\x12\x03\x12\t\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x12\x12\
    \x13\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x13\x02\x14\n\x0c\n\x05\x04\x02\
    \x02\x01\x06\x12\x03\x13\x02\x05\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\
    \x13\x06\x0f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x13\x12\x13\n\n\n\
    \x02\x05\0\x12\x04\x16\0\x1a\x01\n\n\n\x03\x05\0\x01\x12\x03\x16\x05\x08\
    \n\x0b\n\x04\x05\0\x02\0\x12\x03\x17\x02\x10\n\x0c\n\x05\x05\0\x02\0\x01\
    \x12\x03\x17\x02\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x17\x0e\x0f\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03\x18\x02\x0e\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x18\x02\t\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x18\x0c\r\n\
    \x0b\n\x04\x05\0\x02\x02\x12\x03\x19\x02\x12\n\x0c\n\x05\x05\0\x02\x02\
    \x01\x12\x03\x19\x02\r\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x19\x10\x11\
    b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(QueryStatus::generated_message_descriptor_data());
            messages.push(EnableStatus::generated_message_descriptor_data());
            messages.push(BotChange::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(Bot::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
