// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: src/proto.capn

#![allow(unused_imports)]
#![allow(dead_code)]

pub mod command {
  use capnp::any_pointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
  use capnp::list::ToU16;

  pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 3, pointers : 0 };

  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_id(&self) -> u32 {
      self.reader.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn which(&self) -> ::std::option::Option<WhichReader<'a>> {
      match self.reader.get_data_field::<u16>(8) {
        0 => {
          return ::std::option::Some(MoveCube(
            FromStructReader::new(self.reader)
          ));
        }
        1 => {
          return ::std::option::Some(Connect(
            FromStructReader::new(self.reader)
          ));
        }
        2 => {
          return ::std::option::Some(Disconnect(
            FromStructReader::new(self.reader)
          ));
        }
        _ => return ::std::option::None
      }
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_id(&self) -> u32 {
      self.builder.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn set_id(&self, value : u32) {
      self.builder.set_data_field::<u32>(0, value);
    }
    #[inline]
    pub fn init_move_cube(&self, ) -> ::proto_capnp::command::move_cube::Builder<'a> {
      self.builder.set_data_field::<u16>(8, 0);
      self.builder.set_data_field::<f32>(1, 0u8 as f32);
      self.builder.set_data_field::<f32>(2, 0u8 as f32);
      self.builder.set_data_field::<f32>(3, 0u8 as f32);
      FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn init_connect(&self, ) -> ::proto_capnp::command::connect::Builder<'a> {
      self.builder.set_data_field::<u16>(8, 1);
      FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn init_disconnect(&self, ) -> ::proto_capnp::command::disconnect::Builder<'a> {
      self.builder.set_data_field::<u16>(8, 2);
      FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn which(&self) -> ::std::option::Option<WhichBuilder<'a>> {
      match self.builder.get_data_field::<u16>(8) {
        0 => {
          return ::std::option::Some(MoveCube(
            FromStructBuilder::new(self.builder)
          ));
        }
        1 => {
          return ::std::option::Some(Connect(
            FromStructBuilder::new(self.builder)
          ));
        }
        2 => {
          return ::std::option::Some(Disconnect(
            FromStructBuilder::new(self.builder)
          ));
        }
        _ => return ::std::option::None
      }
    }
  }

  pub struct Pipeline { _typeless : any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }
  pub enum Which<'a,A0,A1,A2> {
    MoveCube(A0),
    Connect(A1),
    Disconnect(A2),
  }
  pub type WhichReader<'a> = Which<'a,::proto_capnp::command::move_cube::Reader<'a>,::proto_capnp::command::connect::Reader<'a>,::proto_capnp::command::disconnect::Reader<'a>>;
  pub type WhichBuilder<'a> = Which<'a,::proto_capnp::command::move_cube::Builder<'a>,::proto_capnp::command::connect::Builder<'a>,::proto_capnp::command::disconnect::Builder<'a>>;

  pub mod move_cube {
    use capnp::any_pointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{text, data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
    use capnp::list::ToU16;

    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_d_x(&self) -> f32 {
        self.reader.get_data_field::<f32>(1)
      }
      #[inline]
      pub fn get_d_y(&self) -> f32 {
        self.reader.get_data_field::<f32>(2)
      }
      #[inline]
      pub fn get_d_z(&self) -> f32 {
        self.reader.get_data_field::<f32>(3)
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_d_x(&self) -> f32 {
        self.builder.get_data_field::<f32>(1)
      }
      #[inline]
      pub fn set_d_x(&self, value : f32) {
        self.builder.set_data_field::<f32>(1, value);
      }
      #[inline]
      pub fn get_d_y(&self) -> f32 {
        self.builder.get_data_field::<f32>(2)
      }
      #[inline]
      pub fn set_d_y(&self, value : f32) {
        self.builder.set_data_field::<f32>(2, value);
      }
      #[inline]
      pub fn get_d_z(&self) -> f32 {
        self.builder.get_data_field::<f32>(3)
      }
      #[inline]
      pub fn set_d_z(&self, value : f32) {
        self.builder.set_data_field::<f32>(3, value);
      }
    }

    pub struct Pipeline { _typeless : any_pointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod connect {
    use capnp::any_pointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{text, data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
    use capnp::list::ToU16;

    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_cl(&self) -> () {
        ()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_cl(&self) -> () {
        ()
      }
      #[inline]
      pub fn set_cl(&self, _value : ()) {
      }
    }

    pub struct Pipeline { _typeless : any_pointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }

  pub mod disconnect {
    use capnp::any_pointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{text, data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
    use capnp::list::ToU16;

    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_dc(&self) -> () {
        ()
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_dc(&self) -> () {
        ()
      }
      #[inline]
      pub fn set_dc(&self, _value : ()) {
      }
    }

    pub struct Pipeline { _typeless : any_pointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }
}
