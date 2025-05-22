pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "micro_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__micro_ros_msgs__msg__Graph() -> *const std::os::raw::c_void;
}

#[link(name = "micro_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn micro_ros_msgs__msg__Graph__init(msg: *mut Graph) -> bool;
    fn micro_ros_msgs__msg__Graph__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Graph>, size: usize) -> bool;
    fn micro_ros_msgs__msg__Graph__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Graph>);
    fn micro_ros_msgs__msg__Graph__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Graph>, out_seq: *mut rosidl_runtime_rs::Sequence<Graph>) -> bool;
}

// Corresponds to micro_ros_msgs__msg__Graph
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Graph {
    pub nodes: rosidl_runtime_rs::Sequence<crate::msg::rmw::Node>,
}



impl Default for Graph {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !micro_ros_msgs__msg__Graph__init(&mut msg as *mut _) {
        panic!("Call to micro_ros_msgs__msg__Graph__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Graph {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Graph__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Graph__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Graph__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Graph {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Graph where Self: Sized {
  const TYPE_NAME: &'static str = "micro_ros_msgs/msg/Graph";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__micro_ros_msgs__msg__Graph() }
  }
}


#[link(name = "micro_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__micro_ros_msgs__msg__Node() -> *const std::os::raw::c_void;
}

#[link(name = "micro_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn micro_ros_msgs__msg__Node__init(msg: *mut Node) -> bool;
    fn micro_ros_msgs__msg__Node__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Node>, size: usize) -> bool;
    fn micro_ros_msgs__msg__Node__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Node>);
    fn micro_ros_msgs__msg__Node__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Node>, out_seq: *mut rosidl_runtime_rs::Sequence<Node>) -> bool;
}

// Corresponds to micro_ros_msgs__msg__Node
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node {
    pub node_namespace: rosidl_runtime_rs::BoundedString<256>,
    pub node_name: rosidl_runtime_rs::BoundedString<256>,
    pub entities: rosidl_runtime_rs::Sequence<crate::msg::rmw::Entity>,
}



impl Default for Node {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !micro_ros_msgs__msg__Node__init(&mut msg as *mut _) {
        panic!("Call to micro_ros_msgs__msg__Node__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Node {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Node__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Node__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Node__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Node {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Node where Self: Sized {
  const TYPE_NAME: &'static str = "micro_ros_msgs/msg/Node";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__micro_ros_msgs__msg__Node() }
  }
}


#[link(name = "micro_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__micro_ros_msgs__msg__Entity() -> *const std::os::raw::c_void;
}

#[link(name = "micro_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn micro_ros_msgs__msg__Entity__init(msg: *mut Entity) -> bool;
    fn micro_ros_msgs__msg__Entity__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Entity>, size: usize) -> bool;
    fn micro_ros_msgs__msg__Entity__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Entity>);
    fn micro_ros_msgs__msg__Entity__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Entity>, out_seq: *mut rosidl_runtime_rs::Sequence<Entity>) -> bool;
}

// Corresponds to micro_ros_msgs__msg__Entity
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Entity {
    pub entity_type: u8,
    pub name: rosidl_runtime_rs::BoundedString<256>,
    pub types: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::BoundedString<256>>,
}

impl Entity {
    pub const PUBLISHER: u8 = 0;
    pub const SUBSCRIBER: u8 = 1;
    pub const SERVICE_SERVER: u8 = 2;
    pub const SERVICE_CLIENT: u8 = 3;
}


impl Default for Entity {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !micro_ros_msgs__msg__Entity__init(&mut msg as *mut _) {
        panic!("Call to micro_ros_msgs__msg__Entity__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Entity {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Entity__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Entity__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { micro_ros_msgs__msg__Entity__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Entity {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Entity where Self: Sized {
  const TYPE_NAME: &'static str = "micro_ros_msgs/msg/Entity";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__micro_ros_msgs__msg__Entity() }
  }
}


}  // mod rmw


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Graph {
    pub nodes: Vec<crate::msg::Node>,
}



impl Default for Graph {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::Graph::default())
  }
}

impl rosidl_runtime_rs::Message for Graph {
  type RmwMsg = crate::msg::rmw::Graph;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        nodes: msg.nodes
          .into_iter()
          .map(|elem| crate::msg::Node::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        nodes: msg.nodes
          .iter()
          .map(|elem| crate::msg::Node::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      nodes: msg.nodes
          .into_iter()
          .map(crate::msg::Node::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node {
    pub node_namespace: rosidl_runtime_rs::BoundedString<256>,
    pub node_name: rosidl_runtime_rs::BoundedString<256>,
    pub entities: Vec<crate::msg::Entity>,
}



impl Default for Node {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::Node::default())
  }
}

impl rosidl_runtime_rs::Message for Node {
  type RmwMsg = crate::msg::rmw::Node;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        node_namespace: msg.node_namespace,
        node_name: msg.node_name,
        entities: msg.entities
          .into_iter()
          .map(|elem| crate::msg::Entity::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        node_namespace: msg.node_namespace.clone(),
        node_name: msg.node_name.clone(),
        entities: msg.entities
          .iter()
          .map(|elem| crate::msg::Entity::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      node_namespace: msg.node_namespace,
      node_name: msg.node_name,
      entities: msg.entities
          .into_iter()
          .map(crate::msg::Entity::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Entity {
    pub entity_type: u8,
    pub name: rosidl_runtime_rs::BoundedString<256>,
    pub types: Vec<rosidl_runtime_rs::BoundedString<256>>,
}

impl Entity {
    pub const PUBLISHER: u8 = 0;
    pub const SUBSCRIBER: u8 = 1;
    pub const SERVICE_SERVER: u8 = 2;
    pub const SERVICE_CLIENT: u8 = 3;
}


impl Default for Entity {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::Entity::default())
  }
}

impl rosidl_runtime_rs::Message for Entity {
  type RmwMsg = crate::msg::rmw::Entity;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        entity_type: msg.entity_type,
        name: msg.name,
        types: msg.types.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      entity_type: msg.entity_type,
        name: msg.name.clone(),
        types: msg.types.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      entity_type: msg.entity_type,
      name: msg.name,
      types: msg.types
          .into_iter()
          .collect(),
    }
  }
}


