/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::{Engine, MultiplayerApi};
use godot::meta::error::RpcError;
use godot::obj::Singleton;
use godot::prelude::*;
use godot::register::RpcConfig;
use godot::test::itest;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct RpcCallableNode {
    base: Base<Node>,
}

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
}

#[itest]
fn type_safe_rpc_test() {
    let mut node = RpcCallableNode::new_alloc();

    // Registering is done in `UserClass::__before_ready()`, and it requires a multiplayer API to exist.
    let mut scene_tree = Engine::singleton()
        .get_main_loop()
        .unwrap()
        .cast::<SceneTree>();
    scene_tree.set_multiplayer(MultiplayerApi::create_default_interface().as_ref());

    // We have to manually add the RPC config in this test.
    RpcConfig::default().configure_node(node.upcast_mut(), "say_hello_world");
    RpcConfig::default().configure_node(node.upcast_mut(), "say_hello_to");

    let mut root = scene_tree.get_root().unwrap();

    // Before we add the node to the tree, RPCs will fail.
    assert_eq!(
        Err(RpcError::Unconfigured),
        node.bind_mut().rpcs().say_hello_world().call()
    );

    root.add_child(&node);

    assert_eq!(Ok(()), node.rpcs().say_hello_world().call());

    assert_eq!(Ok(()), node.rpcs().say_hello_to("godot".to_owned()).call());
    assert_eq!(
        Ok(()),
        node.bind_mut()
            .rpcs()
            .say_hello_to("godot".to_owned())
            .call()
    );
    root.remove_child(&node);
    node.free();
}
