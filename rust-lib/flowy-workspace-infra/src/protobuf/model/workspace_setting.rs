// This file is generated by rust-protobuf 2.22.1. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `workspace_setting.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct CurrentWorkspaceSetting {
    // message fields
    pub workspace: ::protobuf::SingularPtrField<super::workspace_create::Workspace>,
    // message oneof groups
    pub one_of_latest_view: ::std::option::Option<CurrentWorkspaceSetting_oneof_one_of_latest_view>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CurrentWorkspaceSetting {
    fn default() -> &'a CurrentWorkspaceSetting {
        <CurrentWorkspaceSetting as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum CurrentWorkspaceSetting_oneof_one_of_latest_view {
    latest_view(super::view_create::View),
}

impl CurrentWorkspaceSetting {
    pub fn new() -> CurrentWorkspaceSetting {
        ::std::default::Default::default()
    }

    // .Workspace workspace = 1;


    pub fn get_workspace(&self) -> &super::workspace_create::Workspace {
        self.workspace.as_ref().unwrap_or_else(|| <super::workspace_create::Workspace as ::protobuf::Message>::default_instance())
    }
    pub fn clear_workspace(&mut self) {
        self.workspace.clear();
    }

    pub fn has_workspace(&self) -> bool {
        self.workspace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_workspace(&mut self, v: super::workspace_create::Workspace) {
        self.workspace = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_workspace(&mut self) -> &mut super::workspace_create::Workspace {
        if self.workspace.is_none() {
            self.workspace.set_default();
        }
        self.workspace.as_mut().unwrap()
    }

    // Take field
    pub fn take_workspace(&mut self) -> super::workspace_create::Workspace {
        self.workspace.take().unwrap_or_else(|| super::workspace_create::Workspace::new())
    }

    // .View latest_view = 2;


    pub fn get_latest_view(&self) -> &super::view_create::View {
        match self.one_of_latest_view {
            ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(ref v)) => v,
            _ => <super::view_create::View as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_latest_view(&mut self) {
        self.one_of_latest_view = ::std::option::Option::None;
    }

    pub fn has_latest_view(&self) -> bool {
        match self.one_of_latest_view {
            ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_latest_view(&mut self, v: super::view_create::View) {
        self.one_of_latest_view = ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(v))
    }

    // Mutable pointer to the field.
    pub fn mut_latest_view(&mut self) -> &mut super::view_create::View {
        if let ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(_)) = self.one_of_latest_view {
        } else {
            self.one_of_latest_view = ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(super::view_create::View::new()));
        }
        match self.one_of_latest_view {
            ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_latest_view(&mut self) -> super::view_create::View {
        if self.has_latest_view() {
            match self.one_of_latest_view.take() {
                ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(v)) => v,
                _ => panic!(),
            }
        } else {
            super::view_create::View::new()
        }
    }
}

impl ::protobuf::Message for CurrentWorkspaceSetting {
    fn is_initialized(&self) -> bool {
        for v in &self.workspace {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(ref v)) = self.one_of_latest_view {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.workspace)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_latest_view = ::std::option::Option::Some(CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.workspace.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_latest_view {
            match v {
                &CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.workspace.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_latest_view {
            match v {
                &CurrentWorkspaceSetting_oneof_one_of_latest_view::latest_view(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CurrentWorkspaceSetting {
        CurrentWorkspaceSetting::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::workspace_create::Workspace>>(
                "workspace",
                |m: &CurrentWorkspaceSetting| { &m.workspace },
                |m: &mut CurrentWorkspaceSetting| { &mut m.workspace },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::view_create::View>(
                "latest_view",
                CurrentWorkspaceSetting::has_latest_view,
                CurrentWorkspaceSetting::get_latest_view,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CurrentWorkspaceSetting>(
                "CurrentWorkspaceSetting",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CurrentWorkspaceSetting {
        static instance: ::protobuf::rt::LazyV2<CurrentWorkspaceSetting> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CurrentWorkspaceSetting::new)
    }
}

impl ::protobuf::Clear for CurrentWorkspaceSetting {
    fn clear(&mut self) {
        self.workspace.clear();
        self.one_of_latest_view = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CurrentWorkspaceSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CurrentWorkspaceSetting {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17workspace_setting.proto\x1a\x11view_create.proto\x1a\x16workspace_\
    create.proto\"\x83\x01\n\x17CurrentWorkspaceSetting\x12(\n\tworkspace\
    \x18\x01\x20\x01(\x0b2\n.WorkspaceR\tworkspace\x12(\n\x0blatest_view\x18\
    \x02\x20\x01(\x0b2\x05.ViewH\0R\nlatestViewB\x14\n\x12one_of_latest_view\
    J\xc9\x01\n\x06\x12\x04\0\0\x07\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\
    \n\x02\x03\0\x12\x03\x01\0\x1b\n\t\n\x02\x03\x01\x12\x03\x02\0\x20\n\n\n\
    \x02\x04\0\x12\x04\x04\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x1f\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x1c\n\x0c\n\x05\x04\0\x02\0\x06\
    \x12\x03\x05\x04\r\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\x0e\x17\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\x05\x1a\x1b\n\x0b\n\x04\x04\0\x08\0\x12\
    \x03\x06\x046\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x06\n\x1c\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x06\x1f4\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\
    \x06\x1f#\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x06$/\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x0623b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}