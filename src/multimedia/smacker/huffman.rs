use std::io::Read;
use super::bit_reader::BitReader;

#[derive(Copy, Clone)]
pub struct NodeId(usize);

#[derive(Copy, Clone)]
pub enum HuffmanNode {
    Node {
        bit_count: usize,
        code: usize,
        left_node_id: NodeId,
        right_node_id: NodeId
    },
    Leaf {
        bit_count: usize,
        code: usize,
        value: usize
    }
}
impl Default for HuffmanNode {
    fn default() -> Self {
        Self::Leaf {
            bit_count: 0,
            code: 0,
            value: 0
        }
    }
}

pub struct HuffmanContext {
    node_arena: Vec<HuffmanNode>,
    depth: usize,
    root_node_id: NodeId
}
impl HuffmanContext {
    fn new(depth: usize) -> Self {
        let mut node_arena = Vec::new();
        let root_node_id = NodeId(node_arena.len());
        node_arena.push(Default::default());
        Self {
            node_arena,
            depth,
            root_node_id
        }
    }

    pub fn get_value<TStream: Read>(
        &self,
        bit_reader: &mut BitReader<TStream>
    ) -> std::io::Result<usize> {
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

    fn from_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        max_depth: usize
    ) -> std::io::Result<Self> {
        let mut context = HuffmanContext::new(max_depth);
        let root_node_id = context.root_node_id;
        Self::decode_tree(bit_reader, max_depth, &mut context, root_node_id, 0)?;
        Ok(context)
    }

    fn decode_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        max_depth: usize,
        context: &mut HuffmanContext,
        node_id: NodeId,
        step_depth: usize
    ) -> std::io::Result<()> {
        let (bit_count, code) = match context.node_arena[node_id.0] {
            HuffmanNode::Leaf { bit_count, code, .. } => {
                (bit_count, code)
            },
            _ => unreachable!()
        };

        if bit_reader.read_bits(1)? == 0 {
            if max_depth <= step_depth {
                panic!("maximum huffman tree size exceeded!")
            }
            context.node_arena[node_id.0] = HuffmanNode::Leaf {
                bit_count,
                code,
                value: bit_reader.read_bits(8)?
            };
            Ok(())
        } else {
            context.node_arena[node_id.0] = {
                let mut left_child = HuffmanNode::Leaf {
                    bit_count: bit_count + 1,
                    code: (code << 1) | 0x1,
                    value: 0
                };
                let left_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(left_child);

                let mut right_child = HuffmanNode::Leaf {
                    bit_count: bit_count + 1,
                    code: (code << 2) | 0x11,
                    value: 0
                };
                let right_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(right_child);

                Self::decode_tree(bit_reader, max_depth, context, left_node_id, step_depth + 1)?;
                Self::decode_tree(bit_reader, max_depth, context, right_node_id, step_depth + 1)?;

                HuffmanNode::Node {
                    bit_count,
                    code,
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
        unimplemented!()
    }

    fn decode_big_tree<TStream: Read>(
        bit_reader: &mut BitReader<TStream>,
        header_tree_head: &mut HeaderTreeHead,
        max_depth: usize,
        context: &mut HuffmanContext,
        node_id: NodeId,
        step_depth: usize
    ) -> std::io::Result<()> {
        let (bit_count, code) = match context.node_arena[node_id.0] {
            HuffmanNode::Leaf { bit_count, code, .. } => {
                (bit_count, code)
            },
            _ => unreachable!()
        };

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
                    header_tree_head.last_nodes[i] = Some(node_id);
                    value = 0
                }
            }
            context.node_arena[node_id.0] = HuffmanNode::Leaf {
                bit_count,
                code,
                value
            };
            Ok(())
        } else {
            context.node_arena[node_id.0] = {
                let mut left_child = HuffmanNode::Leaf {
                    bit_count: bit_count + 1,
                    code: (code << 1) | 0x1,
                    value: 0
                };
                let left_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(left_child);

                let mut right_child = HuffmanNode::Leaf {
                    bit_count: bit_count + 1,
                    code: (code << 2) | 0x11,
                    value: 0
                };
                let right_node_id = NodeId(context.node_arena.len());
                context.node_arena.push(right_child);

                Self::decode_big_tree(bit_reader, header_tree_head, max_depth, context, left_node_id, step_depth + 1)?;
                Self::decode_big_tree(bit_reader, header_tree_head, max_depth, context, right_node_id, step_depth + 1)?;

                HuffmanNode::Node {
                    bit_count,
                    code,
                    left_node_id,
                    right_node_id
                }
            };
            Ok(())
        }
    }
}

pub struct HeaderTreeHead {
    low_tree: Option<HuffmanContext>,
    high_tree: Option<HuffmanContext>,
    escapes: [u16; 3],
    last_nodes: [Option<NodeId>; 3],
}

pub struct HeaderTree {
    head: HeaderTreeHead,
    tree: HuffmanContext
}
impl HeaderTree {
    pub(crate) fn new<TStream: Read>(
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
        let last_nodes = [None; 3];

        let mut head = HeaderTreeHead {
            low_tree,
            high_tree,
            escapes,
            last_nodes
        };

        let tree = HuffmanContext::from_big_tree(bit_reader, &mut head, size)?;
        bit_reader.read_bits(1)?;
        Ok(Self {
            head,
            tree
        })
    }

    pub fn reset_last(&mut self) {
        for i in 0..3 {
            match self.head.last_nodes[i] {
                None => {}
                Some(NodeId(id)) => {
                    match &mut self.tree.node_arena[id] {
                        HuffmanNode::Leaf { value, .. } => *value = 0,
                        HuffmanNode::Node { .. } => {}
                    }
                }
            }
        }
    }

    pub fn get_value<TStream: Read>(
        &mut self,
        bit_reader: &mut BitReader<TStream>
    ) -> std::io::Result<usize> {
        let val = self.tree.get_value(bit_reader)?;
        if let Some(NodeId(id)) = self.head.last_nodes[0] {
            if let HuffmanNode::Leaf { value, .. } = self.tree.node_arena[id] {
                if value != val {
                    for i in 0..3 {
                        match self.head.last_nodes[i] {
                            None => {}
                            Some(NodeId(id)) => {
                                match &mut self.tree.node_arena[id] {
                                    HuffmanNode::Leaf { value, .. } => *value = val,
                                    HuffmanNode::Node { .. } => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(val)
    }
}