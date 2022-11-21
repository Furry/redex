pub struct AST {
    nodes: Vec<ASTNode>
}

pub struct ASTNode {
    pub node_type: Operation,
    pub children: Vec<ASTNode>,
    pub value: Option<String>,
    pub line: usize
}

impl ASTNode {
    pub fn new(node_type: Operation, value: Option<String>, line: usize) -> ASTNode {
        ASTNode {
            node_type,
            children: Vec::new(),
            value,
            line
        }
    }

    pub fn add_child(&mut self, child: ASTNode) {
        self.children.push(child);
    }

    pub fn last_child(&self) -> Option<&ASTNode> {
        self.children.last()
    }

    pub fn size(&self) -> usize {
        self.children.len()
    }

    pub fn nested_size(&self) -> usize {
        let mut size = 0;
        for child in self.children.iter() {
            size += child.nested_size();
        }
        return size;
    }
}

impl AST {
    pub fn new() -> AST {
        AST {
            nodes: Vec::new()
        }
    }

    pub fn head(&self) -> Option<&ASTNode> {
        self.nodes.first()
    }

    pub fn add_node(&mut self, node: ASTNode) {
        self.nodes.push(node);
    }
}

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
    Pass
}
