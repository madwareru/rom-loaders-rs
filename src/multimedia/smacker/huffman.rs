use std::io::Read;
use super::bit_reader::BitReader;

#[derive(Copy, Clone, Default)]
pub(crate) struct NodeId(pub(crate) usize);

#[derive(Copy, Clone)]
pub(crate) enum HuffmanNode {
    Node {
        left_node_id: NodeId,
        right_node_id: NodeId
    },
    Leaf {
        value: u16
    }
}
impl Default for HuffmanNode {
    fn default() -> Self {
        Self::Leaf {
            value: 0
        }
    }
}

pub(crate) struct HuffmanContext {
    node_arena: Vec<HuffmanNode>,
    root_node_id: NodeId
}
impl HuffmanContext {
    ///
    /// It's better not to use this method directly, but if needed,
    /// you can create a context without an actual fill of it
    /// It is needed in situations when we need to reuse exact same
    /// context many times on different trees (e.g. when decoding an audio for example)
    ///
    pub(crate) fn new() -> Self {
        let mut node_arena = Vec::new();
        node_arena.push(Default::default());
        let root_node_id = NodeId(0);
        Self {
            node_arena,
            root_node_id
        }
    }

    pub(crate) fn clear(&mut self) {
        self.node_arena.clear();
        self.node_arena.push(Default::default());
        self.root_node_id = NodeId(0);
    }

    pub(crate) fn get_value<TStream: Read>(
        &self,
        bit_reader: &mut BitReader<TStream>
    ) -> std::io::Result<u16> {
        let mut node_id = self.root_node_id;
        loop {
            let node_content = self.node_arena[node_id.0];
            match node_content {
                HuffmanNode::Leaf { value, .. } => break Ok(value),
                HuffmanNode::Node { left_node_id, right_node_id, .. } => {
                    let bit = bit_reader.read_bits(1)?;
                    match bit {
                        0 => node_id = left_node_id,
                        1 => node_id = right_node_id,
                        _ => unreachable!()
                    }
                }
            }
        }
    }

    pub(crate) fn from_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        max_depth: usize
    ) -> std::io::Result<Self> {
        let mut context = HuffmanContext::new();
        let root_node_id = context.root_node_id;
        Self::decode_tree(bit_reader, max_depth, &mut context, root_node_id, 0)?;
        Ok(context)
    }

    pub(crate) fn decode_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        max_depth: usize,
        context: &mut HuffmanContext,
        node_id: NodeId,
        step_depth: usize
    ) -> std::io::Result<()> {
        if bit_reader.read_bits(1)? == 0 {
            if max_depth <= step_depth {
                panic!("maximum huffman tree size exceeded!")
            }
            context.node_arena[node_id.0] = HuffmanNode::Leaf {
                value: bit_reader.read_bits(8)? as u16
            };
            Ok(())
        } else {
            context.node_arena[node_id.0] = {
                let left_child = HuffmanNode::Leaf {
                    value: 0
                };
                let left_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(left_child);

                let right_child = HuffmanNode::Leaf {
                    value: 0
                };
                let right_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(right_child);

                Self::decode_tree(bit_reader, max_depth, context, left_node_id, step_depth + 1)?;
                Self::decode_tree(bit_reader, max_depth, context, right_node_id, step_depth + 1)?;

                HuffmanNode::Node {
                    left_node_id,
                    right_node_id
                }
            };
            Ok(())
        }
    }

    fn from_big_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        header_tree_head: &mut HeaderTreeHead,
        max_depth: usize,
    ) -> std::io::Result<Self> {
        let mut context = HuffmanContext::new();
        let max_depth = ((max_depth + 3) >> 2) + 4;
        let root_node_id = context.root_node_id;
        Self::decode_big_tree(bit_reader, header_tree_head, max_depth, &mut context, root_node_id, 0)?;
        Ok(context)
    }

    fn decode_big_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        header_tree_head: &mut HeaderTreeHead,
        max_depth: usize,
        context: &mut HuffmanContext,
        node_id: NodeId,
        step_depth: usize
    ) -> std::io::Result<()> {
        if bit_reader.read_bits(1)? == 0 {
            if max_depth <= step_depth {
                panic!("maximum huffman tree size exceeded!")
            }
            let (mut low_byte, mut high_byte) = (0, 0);
            if let Some(low_tree) = &mut header_tree_head.low_tree {
                low_byte = low_tree.get_value(bit_reader)?;
            }
            if let Some(high_tree) = &mut header_tree_head.high_tree {
                high_byte = high_tree.get_value(bit_reader)?;
            }
            let mut value = low_byte + high_byte * 0x100;

            for i in 0..3 {
                if header_tree_head.escapes[i] == value as u16 {
                    println!("found escape {}", i);
                    header_tree_head.last_nodes[i] = node_id;
                    value = 0
                }
            }
            context.node_arena[node_id.0] = HuffmanNode::Leaf {
                value
            };
            Ok(())
        } else {
            context.node_arena[node_id.0] = {
                let left_child = HuffmanNode::Leaf {
                    value: 0
                };
                let left_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(left_child);

                let right_child = HuffmanNode::Leaf {
                    value: 0
                };
                let right_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(right_child);

                Self::decode_big_tree(bit_reader, header_tree_head, max_depth, context, left_node_id, step_depth + 1)?;
                Self::decode_big_tree(bit_reader, header_tree_head, max_depth, context, right_node_id, step_depth + 1)?;

                HuffmanNode::Node {
                    left_node_id,
                    right_node_id
                }
            };
            Ok(())
        }
    }
}

pub(crate) struct HeaderTreeHead {
    low_tree: Option<HuffmanContext>,
    high_tree: Option<HuffmanContext>,
    escapes: [u16; 3],
    last_nodes: [NodeId; 3],
}

pub(crate) struct HeaderTree {
    head: HeaderTreeHead,
    tree: HuffmanContext
}
impl HeaderTree {
    pub(crate) fn read<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        size: usize
    ) -> std::io::Result<Self> {
        let low_tree = if bit_reader.read_bits(1)? == 0 {
            None
        } else {
            let low = HuffmanContext::from_tree(bit_reader, 256)?;
            bit_reader.read_bits(1)?;
            Some(low)
        };
        let high_tree = if bit_reader.read_bits(1)? == 0 {
            None
        } else {
            let high = HuffmanContext::from_tree(bit_reader, 256)?;
            bit_reader.read_bits(1)?;
            Some(high)
        };
        let mut escapes = [0u16; 3];
        escapes[0] = bit_reader.read_bits(16)? as u16;
        escapes[1] = bit_reader.read_bits(16)? as u16;
        escapes[2] = bit_reader.read_bits(16)? as u16;
        println!("escapes: {:?}", escapes);
        let last_nodes = [Default::default(); 3];

        let mut head = HeaderTreeHead {
            low_tree,
            high_tree,
            escapes,
            last_nodes
        };

        let tree = HuffmanContext::from_big_tree(bit_reader, &mut head, size)?;
        let mut result = Self { head, tree };
        if !result.is_leaf(result.head.last_nodes[0]) {
            result.head.last_nodes[0] = NodeId(result.tree.node_arena.len());
            result.tree.node_arena.push(HuffmanNode::Leaf {
                value: 0
            })
        }
        if !result.is_leaf(result.head.last_nodes[1]) {
            result.head.last_nodes[1] = NodeId(result.tree.node_arena.len());
            result.tree.node_arena.push(HuffmanNode::Leaf {
                value: 0
            })
        }
        if !result.is_leaf(result.head.last_nodes[2]) {
            result.head.last_nodes[2] = NodeId(result.tree.node_arena.len());
            result.tree.node_arena.push(HuffmanNode::Leaf {
                value: 0
            })
        }
        bit_reader.read_bits(1)?;
        Ok(result)
    }

    pub(crate) fn reset_last(&mut self) {
        for i in 0..3 {
            match self.head.last_nodes[i] {
                NodeId(id) => {
                    match &mut self.tree.node_arena[id] {
                        HuffmanNode::Leaf { value, .. } => *value = 0,
                        HuffmanNode::Node { .. } => {}
                    }
                }
            }
        }
    }

    fn is_leaf(&self, node_id: NodeId) -> bool {
        match self.tree.node_arena[node_id.0] {
            HuffmanNode::Leaf { .. } => true,
            _ => false
        }
    }

    fn get_leaf_value_by_node_id(&self, node_id: NodeId) -> u16 {
        match self.tree.node_arena[node_id.0] {
            HuffmanNode::Leaf { value, .. } => value,
            _ => unreachable!() // we will panic if someone tries to do it on a non-leaf node
        }
    }

    fn flow_value(&mut self, source_node_id: NodeId, dest_node_id: NodeId) {
        if !self.is_leaf(source_node_id) || !self.is_leaf(dest_node_id) {
            return;
        }
        let source_value = self.get_leaf_value_by_node_id(source_node_id);
        match &mut self.tree.node_arena[dest_node_id.0] {
            HuffmanNode::Leaf { value, .. } => *value = source_value,
            _ => unreachable!() // we will panic if someone tries to do it on a non-leaf node
        }
    }

    pub(crate) fn get_value<TStream: Read>(
        &mut self,
        bit_reader: &mut BitReader<TStream>
    ) -> std::io::Result<u16> {
        let val = self.tree.get_value(bit_reader)?;
        let v0 = self.get_leaf_value_by_node_id(self.head.last_nodes[0]);
        if v0 != val {
            self.flow_value(self.head.last_nodes[1], self.head.last_nodes[2]);
            self.flow_value(self.head.last_nodes[0], self.head.last_nodes[1]);
            match &mut self.tree.node_arena[self.head.last_nodes[0].0] {
                HuffmanNode::Leaf { value, .. } => *value = val,
                _ => unreachable!()
            }
        }
        Ok(val)
    }
}