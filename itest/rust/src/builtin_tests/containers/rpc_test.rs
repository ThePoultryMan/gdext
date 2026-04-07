/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::meta::error::RpcError;
use godot::prelude::*;
use godot::test::itest;

use crate::framework::TestContext;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct RpcCallableNode {
    base: Base<Node>,
}

#[godot_api]
impl INode for RpcCallableNode {}

#[godot_api]
impl RpcCallableNode {
    #[rpc]
    pub fn say_hello_world(&mut self) {
        godot_print!("hello, world");
    }

    #[rpc]
    pub fn say_hello_to(&mut self, to: String) {
        godot_print!("hello, {to}");
    }

    #[rpc]
    pub fn say_number(&self, number: i32) {
        godot_print!("{number}");
    }
}

#[itest]
fn type_safe_rpc_test(context: &TestContext) {
    let mut node = RpcCallableNode::new_alloc();

    let mut root = context.scene_tree.clone();

    // Before we add the node to the tree, RPCs will fail.
    assert_eq!(
        Err(RpcError::Unconfigured),
        node.bind_mut().rpcs().say_hello_world().call()
    );

    root.add_child(&node);

    assert_eq!(Ok(()), node.rpcs().say_hello_world().call());

    let arg = "godot".to_string();
    assert_eq!(Ok(()), node.rpcs().say_hello_to(arg.clone()).call());
    assert_eq!(Ok(()), node.bind_mut().rpcs().say_hello_to(arg).call());
    assert_eq!(Ok(()), node.rpcs().say_number(3).call());
    root.remove_child(&node);
    node.free();
}
