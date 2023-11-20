// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2023, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

#![cfg_attr(docsrs, doc(cfg(feature = "lua")))]
//! Provides access to `StackGraph` instances from Lua.
//!
//! With the `lua` feature enabled, you can add [`StackGraph`] instances to a [`Lua`][mlua::Lua]
//! interpreter.  You might typically use this to _create_ stack graphs from Lua, by calling a Lua
//! function with an empty stack graph as a parameter.  Note that you'll almost certainly need to
//! use `mlua`'s [scoped values](mlua::Lua::scope) mechanism so that you can still use the
//! [`StackGraph`] on the Rust side once the Lua function has finished.
//!
//! ```
//! # use mlua::Lua;
//! # use stack_graphs::graph::StackGraph;
//! # fn main() -> Result<(), mlua::Error> {
//! let lua = Lua::new();
//! let chunk = r#"
//!     function process_graph(graph)
//!       local file = graph:file("test.py")
//!       local def = file:definition_node("foo")
//!       def:add_edge_from(graph:root_node())
//!     end
//! "#;
//! lua.load(chunk).set_name("stack graph chunk").exec()?;
//! let process_graph: mlua::Function = lua.globals().get("process_graph")?;
//!
//! let mut graph = StackGraph::new();
//! lua.scope(|scope| {
//!     let graph = graph.lua_ref_mut(&scope)?;
//!     process_graph.call(graph)
//! })?;
//! assert_eq!(graph.iter_nodes().count(), 3);
//! # Ok(())
//! # }
//! ```
//!
//! ## Building
//!
//! Lua support is only enabled if you compile with the `lua` feature.  This feature is not enough
//! on its own, because the `mlua` crate supports multiple Lua versions, and can either link
//! against a system-installed copy of Lua, or build its own copy from vendored Lua source.  These
//! choices are all controlled via additional features on the `mlua` crate.
//!
//! When building and testing this crate, make sure to provide all necessary features on the
//! command line:
//!
//! ``` console
//! $ cargo test --features lua,mlua/lua54,mlua/vendored
//! ```
//!
//! When building a crate that depends on this crate, add a dependency on `mlua` so that you can
//! set its feature flags:
//!
//! ``` toml
//! [dependencies]
//! stack-graphs = { version="0.13", features=["lua"] }
//! mlua = { version="0.9", features=["lua54", "vendored"] }
//! ```
//!
//! ## Lua API
//!
//! ### Stack graphs
//!
//! The following Lua methods are available on a stack graph instance:
//!
//! #### `edges`
//!
//! ``` lua
//! let edges = graph:edges()
//! ```
//!
//! Returns an array containing all of the edges in the graph.
//!
//! #### `file`
//!
//! ``` lua
//! local file = graph:file(name)
//! ```
//!
//! Returns the file in the stack graph with the given name, creating it if necessary.
//!
//! #### `jump_to_node`
//!
//! ``` lua
//! local node = graph:jump_to_node()
//! ```
//!
//! Returns the graph's jump-to node.
//!
//! #### `nodes`
//!
//! ``` lua
//! for node in graph:nodes() do
//!   -- whatever
//! end
//! ```
//!
//! Returns an iterator of every node in the stack graph.
//!
//! #### `root_node`
//!
//! ``` lua
//! local node = graph:root_node()
//! ```
//!
//! Returns the graph's root node.
//!
//! ### Files
//!
//! The following Lua methods are available on a file instance:
//!
//! #### `definition_node`
//!
//! ``` lua
//! local node = file:definition_node(symbol)
//! ```
//!
//! Adds a new definition node to this file.  `symbol` must be a string, or an instance that can be
//! converted to a string via its `tostring` method.
//!
//! #### `drop_scopes_node`
//!
//! ``` lua
//! local node = file:drop_scopes_node()
//! ```
//!
//! Adds a new drop scopes node to this file.
//!
//! #### `edges`
//!
//! ``` lua
//! let edges = file:edges()
//! ```
//!
//! Returns an array containing all of the edges starting from or leaving a node in this file.
//!
//! #### `exported_scope_node`
//!
//! ``` lua
//! local node = file:exported_scope_node()
//! ```
//!
//! Adds a new exported scope node to this file.
//!
//! #### `internal_scope_node`
//!
//! ``` lua
//! local node = file:internal_scope_node()
//! ```
//!
//! Adds a new internal scope node to this file.
//!
//! #### `jump_to_node`
//!
//! ``` lua
//! local node = file:jump_to_node()
//! ```
//!
//! Returns the root node of the graph containing this file.
//!
//! #### `pop_scoped_symbol_node`
//!
//! ``` lua
//! local node = file:pop_scoped_symbol_node(symbol)
//! ```
//!
//! Adds a new pop scoped symbol node to this file.  `symbol` must be a string, or an instance that
//! can be converted to a string via its `tostring` method.
//!
//! #### `pop_symbol_node`
//!
//! ``` lua
//! local node = file:pop_symbol_node(symbol)
//! ```
//!
//! Adds a new pop symbol node to this file.  `symbol` must be a string, or an instance that can be
//! converted to a string via its `tostring` method.
//!
//! #### `push_scoped_symbol_node`
//!
//! ``` lua
//! local node = file:push_scoped_symbol_node(symbol, scope)
//! ```
//!
//! Adds a new push scoped symbol node to this file.  `symbol` must be a string, or an instance
//! that can be converted to a string via its `tostring` method.  `scope` must be an exported scope
//! node.
//!
//! #### `push_symbol_node`
//!
//! ``` lua
//! local node = file:push_symbol_node(symbol)
//! ```
//!
//! Adds a new push symbol node to this file.  `symbol` must be a string, or an instance that can
//! be converted to a string via its `tostring` method.
//!
//! #### `reference_node`
//!
//! ``` lua
//! local node = file:reference_node(symbol)
//! ```
//!
//! Adds a new definition node to this file.  `symbol` must be a string, or an instance that can be
//! converted to a string via its `tostring` method.
//!
//! #### `root_node`
//!
//! ``` lua
//! local node = file:root_node()
//! ```
//!
//! Returns the root node of the graph containing this file.
//!
//! #### `scoped_definition_node`
//!
//! ``` lua
//! local node = file:scoped_definition_node(symbol)
//! ```
//!
//! Adds a new scoped definition node to this file.  `symbol` must be a string, or an instance that
//! can be converted to a string via its `tostring` method.
//!
//! #### `scoped_reference_node`
//!
//! ``` lua
//! local node = file:scoped_reference_node(symbol)
//! ```
//!
//! Adds a new scoped reference node to this file.  `symbol` must be a string, or an instance that
//! can be converted to a string via its `tostring` method.
//!
//! ### Nodes
//!
//! The following Lua methods are available on a node instance:
//!
//! #### `add_edge_from`
//!
//! ``` lua
//! node:add_edge_from(other, precedence)
//! ```
//!
//! Adds an edge from another node to this node.  `precedence` is optional; it defaults to 0 if not
//! given.
//!
//! #### `add_edge_to`
//!
//! ``` lua
//! node:add_edge_to(other, precedence)
//! ```
//!
//! Adds an edge from this node to another node.  `precedence` is optional; it defaults to 0 if not
//! given.
//!
//! #### `debug_info`
//!
//! ``` lua
//! let info = node:debug_info()
//! ```
//!
//! Returns a Lua table containing all of the debug info added to this node.
//!
//! #### `definiens_span`
//!
//! ``` lua
//! let span = node:definiens_span()
//! ```
//!
//! Returns the definiens span of this node.  (See [`set_definiens_span`](#set_definiens_span) for
//! the structure of a span.)
//!
//! #### `local_id`
//!
//! ``` lua
//! let local_id = node:local_id()
//! ```
//!
//! Returns the local ID of this node within its file.
//!
//! #### `outgoing_edges`
//!
//! ``` lua
//! let edges = node:outgoing_edges()
//! ```
//!
//! Returns an array containing all of the edges leaving this node.
//!
//! #### `set_debug_info`
//!
//! ``` lua
//! node:add_debug_info(key, value)
//! ```
//!
//! Adds a new debug info to this node.  `key` and `value` must each be a string, or an instance
//! that can be converted to a string via its `tostring` method.
//!
//! #### `set_definiens_span`
//!
//! ``` lua
//! node:set_definiens_span {
//!   start = {
//!     line = 1,
//!     column = { utf8_offset = 1, utf16_offset = 1, grapheme_offset = 1 },
//!     -- UTF-8 offsets within the source file of the line containing the span
//!     containing_line = { start = 1, end = 14 },
//!     -- UTF-8 offsets within the source file of the line containing the span, with leading and
//!     -- trailing whitespace removed
//!     trimmed_line = { start = 2, end = 12 },
//!   },
//!   end = {
//!     line = 2,
//!     column = { utf8_offset = 12, utf16_offset = 10, grapheme_offset = 8 },
//!     containing_line = { start = 1, end = 14 },
//!     trimmed_line = { start = 1, end = 14 },
//!   },
//! }
//! ```
//!
//! Sets the definiens span of this node.  All entries in the table are optional, and default to 0
//! if not provided.
//!
//! #### `set_span`
//!
//! ``` lua
//! node:set_span {
//!   start = {
//!     line = 1,
//!     column = { utf8_offset = 1, utf16_offset = 1, grapheme_offset = 1 },
//!     -- UTF-8 offsets within the source file of the line containing the span
//!     containing_line = { start = 1, end = 14 },
//!     -- UTF-8 offsets within the source file of the line containing the span, with leading and
//!     -- trailing whitespace removed
//!     trimmed_line = { start = 2, end = 12 },
//!   },
//!   end = {
//!     line = 2,
//!     column = { utf8_offset = 12, utf16_offset = 10, grapheme_offset = 8 },
//!     containing_line = { start = 1, end = 14 },
//!     trimmed_line = { start = 1, end = 14 },
//!   },
//! }
//! ```
//!
//! Sets the span of this node.  All entries in the table are optional, and default to 0 if not
//! provided.
//!
//! #### `set_syntax_type`
//!
//! ``` lua
//! node:set_syntax_type(syntax_type)
//! ```
//!
//! Sets the syntax type of this node.  `syntax_type` must be a string, or an instance that can be
//! converted to a string via its `tostring` method.
//!
//! #### `span`
//!
//! ``` lua
//! let span = node:span()
//! ```
//!
//! Returns the span of this node.  (See [`set_span`](#set_span) for the structure of a span.)
//!
//! #### `syntax_type`
//!
//! ``` lua
//! let syntax_type = node:syntax_type()
//! ```
//!
//! Returns the syntax type of this node.

// Implementation notes: Stack graphs, files, and nodes can live inside the Lua interpreter as
// objects.  They are each wrapped in a userdata, with a metatable defining the methods that are
// available.  With mlua, the UserData trait is the way to define these metatables and methods.
//
// Complicating matters is that files and nodes need to be represented by a _pair_ of Lua values:
// the handle of the file or node, and a reference to the StackGraph that the file or node lives
// in.  We need both because some of the methods need to dereference the handle to get e.g. the
// `Node` instance.  It's not safe to dereference the handle when we create the userdata, because
// the resulting pointer is not guaranteed to be stable.  (If you add another node, the arena's
// storage might get resized, moving the node instances around in memory.)
//
// To handle this, we leverage Lua's ability to associate “user values” with each userdata.  For
// files and nodes, we store the graph's userdata (i.e. its Lua representation) as the user value
// of each file and node userdata.
//
// That, in turn, means that we must use `add_function` to define each metatable method, since that
// gives us an `mlua::AnyUserData`, which lets us access the userdata's underlying Rust value _and_
// its user value.  (Typically, you would use the more ergonomic `add_method` or `add_method_mut`,
// which take care of unwrapping the userdata and giving you a &ref or &mut ref to the underlying
// Rust type.  But then you don't have access to the userdata's user value.)

use std::fmt::Write;
use std::num::NonZeroU32;

use controlled_option::ControlledOption;
use lsp_positions::Span;
use mlua::AnyUserData;
use mlua::Lua;
use mlua::Scope;
use mlua::UserData;
use mlua::UserDataMethods;

use crate::arena::Handle;
use crate::graph::Edge;
use crate::graph::File;
use crate::graph::Node;
use crate::graph::StackGraph;

impl StackGraph {
    // Returns a Lua wrapper for this stack graph.  Takes ownership of the stack graph.  If you
    // want to access the stack graph after your Lua code is done with it, use [`lua_ref_mut`]
    // instead.
    pub fn lua_value<'lua>(self, lua: &'lua Lua) -> Result<AnyUserData<'lua>, mlua::Error> {
        lua.create_userdata(self)
    }

    // Returns a scoped Lua wrapper for this stack graph.
    pub fn lua_ref_mut<'lua, 'scope>(
        &'scope mut self,
        scope: &Scope<'lua, 'scope>,
    ) -> Result<AnyUserData<'lua>, mlua::Error> {
        scope.create_userdata_ref_mut(self)
    }

    // Returns a scoped Lua wrapper for a file in this stack graph.
    pub fn file_lua_ref_mut<'lua, 'scope>(
        &'scope mut self,
        file: Handle<File>,
        scope: &Scope<'lua, 'scope>,
    ) -> Result<AnyUserData<'lua>, mlua::Error> {
        let graph_ud = self.lua_ref_mut(scope)?;
        let file_ud = scope.create_userdata(file)?;
        file_ud.set_user_value(graph_ud)?;
        Ok(file_ud)
    }
}

impl UserData for StackGraph {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("edges", |l, graph_ud: AnyUserData| {
            let graph = graph_ud.borrow::<StackGraph>()?;
            let mut edges = Vec::new();
            for node in graph.iter_nodes() {
                for edge in graph.outgoing_edges(node) {
                    let edge_ud = l.create_userdata(edge)?;
                    edge_ud.set_user_value(graph_ud.clone())?;
                    edges.push(edge_ud);
                }
            }
            Ok(edges)
        });

        methods.add_function("file", |l, (graph_ud, name): (AnyUserData, String)| {
            let file = {
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                graph.get_or_create_file(&name)
            };
            let file_ud = l.create_userdata(file)?;
            file_ud.set_user_value(graph_ud)?;
            Ok(file_ud)
        });

        methods.add_function("jump_to_node", |l, graph_ud: AnyUserData| {
            let node = StackGraph::jump_to_node();
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });

        methods.add_function("nodes", |l, graph_ud: AnyUserData| {
            let iter = l.create_function(
                |l, (graph_ud, prev_node_ud): (AnyUserData, Option<AnyUserData>)| {
                    let prev_index = match prev_node_ud {
                        Some(prev_node_ud) => {
                            let prev_node = prev_node_ud.borrow::<Handle<Node>>()?;
                            prev_node.as_u32()
                        }
                        None => 0,
                    };
                    let node_index = {
                        let graph = graph_ud.borrow::<StackGraph>()?;
                        let node_count = graph.nodes.len() as u32;
                        if prev_index == node_count - 1 {
                            return Ok(None);
                        }
                        unsafe { NonZeroU32::new_unchecked(prev_index + 1) }
                    };
                    let node = Handle::new(node_index);
                    let node_ud = l.create_userdata::<Handle<Node>>(node)?;
                    node_ud.set_user_value(graph_ud)?;
                    Ok(Some(node_ud))
                },
            )?;
            Ok((iter, graph_ud, None::<AnyUserData>))
        });

        methods.add_function("root_node", |l, graph_ud: AnyUserData| {
            let node = StackGraph::root_node();
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });
    }
}

impl UserData for Handle<File> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "definition_node",
            |l, (file_ud, symbol): (AnyUserData, String)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_pop_symbol_node(node_id, symbol, true)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function("drop_scopes_node", |l, file_ud: AnyUserData| {
            let file = *file_ud.borrow::<Handle<File>>()?;
            let graph_ud = file_ud.user_value::<AnyUserData>()?;
            let node = {
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                let node_id = graph.new_node_id(file);
                graph
                    .add_drop_scopes_node(node_id)
                    .expect("Node ID collision")
            };
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });

        methods.add_function("edges", |l, file_ud: AnyUserData| {
            let file = *file_ud.borrow::<Handle<File>>()?;
            let graph_ud = file_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let mut edges = Vec::new();
            // First find any edges from the singleton nodes _to_ a node in this file.
            for edge in graph.outgoing_edges(StackGraph::root_node()) {
                if !graph[edge.sink].file().map(|f| f == file).unwrap_or(false) {
                    continue;
                }
                let edge_ud = l.create_userdata(edge)?;
                edge_ud.set_user_value(graph_ud.clone())?;
                edges.push(edge_ud);
            }
            for edge in graph.outgoing_edges(StackGraph::jump_to_node()) {
                if !graph[edge.sink].file().map(|f| f == file).unwrap_or(false) {
                    continue;
                }
                let edge_ud = l.create_userdata(edge)?;
                edge_ud.set_user_value(graph_ud.clone())?;
                edges.push(edge_ud);
            }
            // Then find any edges _starting_ from a node in this file.
            for node in graph.nodes_for_file(file) {
                for edge in graph.outgoing_edges(node) {
                    let edge_ud = l.create_userdata(edge)?;
                    edge_ud.set_user_value(graph_ud.clone())?;
                    edges.push(edge_ud);
                }
            }
            Ok(edges)
        });

        methods.add_function("exported_scope_node", |l, file_ud: AnyUserData| {
            let file = *file_ud.borrow::<Handle<File>>()?;
            let graph_ud = file_ud.user_value::<AnyUserData>()?;
            let node = {
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                let node_id = graph.new_node_id(file);
                graph
                    .add_scope_node(node_id, true)
                    .expect("Node ID collision")
            };
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });

        methods.add_function("internal_scope_node", |l, file_ud: AnyUserData| {
            let file = *file_ud.borrow::<Handle<File>>()?;
            let graph_ud = file_ud.user_value::<AnyUserData>()?;
            let node = {
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                let node_id = graph.new_node_id(file);
                graph
                    .add_scope_node(node_id, false)
                    .expect("Node ID collision")
            };
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });

        methods.add_function("jump_to_node", |l, file_ud: AnyUserData| {
            let graph_ud = file_ud.user_value::<AnyUserData>()?;
            let node = StackGraph::jump_to_node();
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });

        methods.add_function("nodes", |l, file_ud: AnyUserData| {
            let iter = l.create_function(
                |l, (file_ud, prev_node_ud): (AnyUserData, Option<AnyUserData>)| {
                    // Pull out the node handle from the previous iteration.
                    let prev_index = match prev_node_ud {
                        Some(prev_node_ud) => {
                            let prev_node = prev_node_ud.borrow::<Handle<Node>>()?;
                            prev_node.as_u32()
                        }
                        None => 0,
                    };

                    // Loop through the next node handles until we find one belonging to the file.
                    let graph_ud = file_ud.user_value::<AnyUserData>()?;
                    let node = {
                        let file = *file_ud.borrow::<Handle<File>>()?;
                        let graph = graph_ud.borrow::<StackGraph>()?;
                        let node_count = graph.nodes.len() as u32;
                        let mut node_index = unsafe { NonZeroU32::new_unchecked(prev_index + 1) };
                        loop {
                            let handle = Handle::<Node>::new(node_index);

                            // If we reach the end without finding a matching node, return nil
                            // to terminate the iterator.
                            if node_index.get() == node_count {
                                return Ok(None);
                            }

                            // If the node belongs to the file, break out of the loop to use this
                            // node as the next result of the iterator.
                            if graph[handle].file().map(|f| f == file).unwrap_or(false) {
                                break handle;
                            }

                            // Otherwise try the next node.
                            node_index = node_index.checked_add(1).unwrap();
                        }
                    };

                    // Wrap up the node handle that we just found.
                    let node_ud = l.create_userdata::<Handle<Node>>(node)?;
                    node_ud.set_user_value(graph_ud)?;
                    Ok(Some(node_ud))
                },
            )?;
            Ok((iter, file_ud, None::<AnyUserData>))
        });

        methods.add_function(
            "pop_scoped_symbol_node",
            |l, (file_ud, symbol): (AnyUserData, String)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_pop_scoped_symbol_node(node_id, symbol, false)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function(
            "pop_symbol_node",
            |l, (file_ud, symbol): (AnyUserData, String)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_pop_symbol_node(node_id, symbol, false)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function(
            "push_scoped_symbol_node",
            |l, (file_ud, symbol, scope_ud): (AnyUserData, String, AnyUserData)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let scope = *scope_ud.borrow::<Handle<Node>>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let scope_id = {
                        let scope = &graph[scope];
                        if !scope.is_exported_scope() {
                            return Err(mlua::Error::RuntimeError(
                                "Can only push exported scope nodes".to_string(),
                            ));
                        }
                        scope.id()
                    };
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_push_scoped_symbol_node(node_id, symbol, scope_id, false)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function(
            "push_symbol_node",
            |l, (file_ud, symbol): (AnyUserData, String)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_push_symbol_node(node_id, symbol, false)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function(
            "reference_node",
            |l, (file_ud, symbol): (AnyUserData, String)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_push_symbol_node(node_id, symbol, true)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function("root_node", |l, file_ud: AnyUserData| {
            let graph_ud = file_ud.user_value::<AnyUserData>()?;
            let node = StackGraph::root_node();
            let node_ud = l.create_userdata(node)?;
            node_ud.set_user_value(graph_ud)?;
            Ok(node_ud)
        });

        methods.add_function(
            "scoped_definition_node",
            |l, (file_ud, symbol): (AnyUserData, String)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_pop_scoped_symbol_node(node_id, symbol, true)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );

        methods.add_function(
            "scoped_reference_node",
            |l, (file_ud, symbol, scope_ud): (AnyUserData, String, AnyUserData)| {
                let file = *file_ud.borrow::<Handle<File>>()?;
                let graph_ud = file_ud.user_value::<AnyUserData>()?;
                let scope = *scope_ud.borrow::<Handle<Node>>()?;
                let node = {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    let scope_id = {
                        let scope = &graph[scope];
                        if !scope.is_exported_scope() {
                            return Err(mlua::Error::RuntimeError(
                                "Can only push exported scope nodes".to_string(),
                            ));
                        }
                        scope.id()
                    };
                    let symbol = graph.add_symbol(&symbol);
                    let node_id = graph.new_node_id(file);
                    graph
                        .add_push_scoped_symbol_node(node_id, symbol, scope_id, true)
                        .expect("Node ID collision")
                };
                let node_ud = l.create_userdata(node)?;
                node_ud.set_user_value(graph_ud)?;
                Ok(node_ud)
            },
        );
    }
}

impl UserData for Handle<Node> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "add_edge_from",
            |l, (this_ud, from_ud, precedence): (AnyUserData, AnyUserData, Option<i32>)| {
                let this = *this_ud.borrow::<Handle<Node>>()?;
                let from = *from_ud.borrow::<Handle<Node>>()?;
                let graph_ud = this_ud.user_value::<AnyUserData>()?;
                let precedence = precedence.unwrap_or(0);
                {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    graph.add_edge(from, this, precedence);
                }
                let edge = Edge {
                    source: from,
                    sink: this,
                    precedence,
                };
                let edge_ud = l.create_userdata(edge)?;
                edge_ud.set_user_value(graph_ud)?;
                Ok(edge_ud)
            },
        );

        methods.add_function(
            "add_edge_to",
            |l, (this_ud, to_ud, precedence): (AnyUserData, AnyUserData, Option<i32>)| {
                let this = *this_ud.borrow::<Handle<Node>>()?;
                let to = *to_ud.borrow::<Handle<Node>>()?;
                let graph_ud = this_ud.user_value::<AnyUserData>()?;
                let precedence = precedence.unwrap_or(0);
                {
                    let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                    graph.add_edge(this, to, precedence);
                }
                let edge = Edge {
                    source: this,
                    sink: to,
                    precedence,
                };
                let edge_ud = l.create_userdata(edge)?;
                edge_ud.set_user_value(graph_ud)?;
                Ok(edge_ud)
            },
        );

        methods.add_function("debug_info", |l, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let debug_info = match graph.node_debug_info(node) {
                Some(debug_info) => debug_info,
                None => return Ok(None),
            };
            let result = l.create_table()?;
            for entry in debug_info.iter() {
                result.set(&graph[entry.key], &graph[entry.value])?;
            }
            Ok(Some(result))
        });

        methods.add_function("definiens_span", |_, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let source_info = match graph.source_info(node) {
                Some(source_info) => source_info,
                None => return Ok(None),
            };
            Ok(Some(source_info.definiens_span.clone()))
        });

        methods.add_function("local_id", |_, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            Ok(graph[node].id().local_id())
        });

        methods.add_function("outgoing_edges", |l, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let mut edges = Vec::new();
            for edge in graph.outgoing_edges(node) {
                let edge_ud = l.create_userdata(edge)?;
                edge_ud.set_user_value(graph_ud.clone())?;
                edges.push(edge_ud);
            }
            Ok(edges)
        });

        methods.add_function(
            "set_debug_info",
            |_, (node_ud, k, v): (AnyUserData, String, String)| {
                let node = *node_ud.borrow::<Handle<Node>>()?;
                let graph_ud = node_ud.user_value::<AnyUserData>()?;
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                let k = graph.add_string(&k);
                let v = graph.add_string(&v);
                graph.node_debug_info_mut(node).add(k, v);
                Ok(())
            },
        );

        methods.add_function(
            "set_definiens_span",
            |_, (node_ud, definiens_span): (AnyUserData, Span)| {
                let node = *node_ud.borrow::<Handle<Node>>()?;
                let graph_ud = node_ud.user_value::<AnyUserData>()?;
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                graph.source_info_mut(node).definiens_span = definiens_span;
                Ok(())
            },
        );

        methods.add_function("set_span", |_, (node_ud, span): (AnyUserData, Span)| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
            graph.source_info_mut(node).span = span;
            Ok(())
        });

        methods.add_function(
            "set_syntax_type",
            |_, (node_ud, syntax_type): (AnyUserData, String)| {
                let node = *node_ud.borrow::<Handle<Node>>()?;
                let graph_ud = node_ud.user_value::<AnyUserData>()?;
                let mut graph = graph_ud.borrow_mut::<StackGraph>()?;
                let syntax_type = graph.add_string(&syntax_type);
                graph.source_info_mut(node).syntax_type = ControlledOption::some(syntax_type);
                Ok(())
            },
        );

        methods.add_function("span", |_, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let source_info = match graph.source_info(node) {
                Some(source_info) => source_info,
                None => return Ok(None),
            };
            Ok(Some(source_info.span.clone()))
        });

        methods.add_function("syntax_type", |_, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let source_info = match graph.source_info(node) {
                Some(source_info) => source_info,
                None => return Ok(None),
            };
            let syntax_type = match source_info.syntax_type.into_option() {
                Some(syntax_type) => syntax_type,
                None => return Ok(None),
            };
            Ok(Some(graph[syntax_type].to_string()))
        });

        methods.add_meta_function(mlua::MetaMethod::ToString, |_, node_ud: AnyUserData| {
            let node = *node_ud.borrow::<Handle<Node>>()?;
            let graph_ud = node_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let mut display = graph[node].display(&graph).to_string();
            if let Some(source_info) = graph.source_info(node) {
                display.pop(); // remove the trailing ]
                if let Some(syntax_type) = source_info.syntax_type.into_option() {
                    write!(&mut display, " ({})", syntax_type.display(&graph)).unwrap();
                }
                if source_info.span != Span::default() {
                    write!(
                        &mut display,
                        " at {}:{}-{}:{}",
                        source_info.span.start.line,
                        source_info.span.start.column.utf8_offset,
                        source_info.span.end.line,
                        source_info.span.end.column.utf8_offset,
                    )
                    .unwrap();
                }
                if source_info.definiens_span != Span::default() {
                    write!(
                        &mut display,
                        " def {}:{}-{}:{}",
                        source_info.definiens_span.start.line,
                        source_info.definiens_span.start.column.utf8_offset,
                        source_info.definiens_span.end.line,
                        source_info.definiens_span.end.column.utf8_offset,
                    )
                    .unwrap();
                }
                display.push(']');
            }
            Ok(display)
        });
    }
}

impl UserData for Edge {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(mlua::MetaMethod::ToString, |_, edge_ud: AnyUserData| {
            let edge = *edge_ud.borrow::<Edge>()?;
            let graph_ud = edge_ud.user_value::<AnyUserData>()?;
            let graph = graph_ud.borrow::<StackGraph>()?;
            let display = format!(
                "{} -{}-> {}",
                edge.source.display(&graph),
                edge.precedence,
                edge.sink.display(&graph),
            );
            Ok(display)
        });
    }
}