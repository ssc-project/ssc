use oxc_index::IndexVec;
use ssc_css_ast::{ast::AstNodeId, AstKind};

/// Semantic node contains all the semantic information about an ast node.
#[derive(Debug, Clone, Copy)]
pub struct AstNode<'a> {
    id: AstNodeId,
    /// A pointer to the ast node, which resides in the `bumpalo` memory arena.
    kind: AstKind<'a>,
}

impl<'a> AstNode<'a> {
    pub fn new(kind: AstKind<'a>) -> Self {
        Self { id: AstNodeId::new(0), kind }
    }

    pub fn id(&self) -> AstNodeId {
        self.id
    }

    pub fn kind(&self) -> AstKind<'a> {
        self.kind
    }
}

/// Untyped AST nodes flattened into an vec
#[derive(Debug, Default)]
pub struct AstNodes<'a> {
    root: Option<AstNodeId>,
    nodes: IndexVec<AstNodeId, AstNode<'a>>,
    parent_ids: IndexVec<AstNodeId, Option<AstNodeId>>,
}

impl<'a> AstNodes<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &AstNode<'a>> + '_ {
        self.nodes.iter()
    }

    /// Walk up the AST, iterating over each parent node.
    ///
    /// The first node produced by this iterator is the first parent of the node
    /// pointed to by `node_id`. The last node will usually be a `StyleSheet`.
    pub fn iter_parents(&self, node_id: AstNodeId) -> impl Iterator<Item = &AstNode<'a>> + '_ {
        let curr = Some(self.get_node(node_id));
        AstNodeParentIter { curr, nodes: self }
    }

    pub fn kind(&self, ast_node_id: AstNodeId) -> AstKind<'a> {
        self.nodes[ast_node_id].kind
    }

    pub fn parent_id(&self, ast_node_id: AstNodeId) -> Option<AstNodeId> {
        self.parent_ids[ast_node_id]
    }

    pub fn parent_kind(&self, ast_node_id: AstNodeId) -> Option<AstKind<'a>> {
        self.parent_id(ast_node_id).map(|node_id| self.kind(node_id))
    }

    pub fn parent_node(&self, ast_node_id: AstNodeId) -> Option<&AstNode<'a>> {
        self.parent_id(ast_node_id).map(|node_id| self.get_node(node_id))
    }

    pub fn get_node(&self, ast_node_id: AstNodeId) -> &AstNode<'a> {
        &self.nodes[ast_node_id]
    }

    pub fn get_node_mut(&mut self, ast_node_id: AstNodeId) -> &mut AstNode<'a> {
        &mut self.nodes[ast_node_id]
    }

    /// Get the root `AstNodeId`, It is always pointing to a `StyleSheet`.
    /// Returns `None` if root node isn't set.
    pub fn root(&self) -> Option<AstNodeId> {
        self.root
    }

    /// Set the root node,
    /// SAFETY:
    /// The root `AstNode` should always point to a `StyleSheet` and this should be the real root of
    /// the tree, It isn't possible to statically check for this so user should think about it before
    /// using.
    #[allow(unsafe_code)]
    pub(super) unsafe fn set_root(&mut self, root: &AstNode<'a>) {
        match root.kind() {
            AstKind::StyleSheet(_) => {
                self.root = Some(root.id());
            }
            _ => unreachable!("Expected a `StyleSheet` node as the root of the tree."),
        }
    }

    /// Get the root node as immutable reference, It is always guaranteed to be a `StyleSheet`.
    /// Returns `None` if root node isn't set.
    pub fn root_node(&self) -> Option<&AstNode<'a>> {
        self.root().map(|id| self.get_node(id))
    }

    /// Get the root node as mutable reference, It is always guaranteed to be a `StyleSheet`.
    /// Returns `None` if root node isn't set.
    pub fn root_node_mut(&mut self) -> Option<&mut AstNode<'a>> {
        self.root().map(|id| self.get_node_mut(id))
    }

    /// Walk up the AST, iterating over each parent node.
    ///
    /// The first node produced by this iterator is the first parent of the node
    /// pointed to by `node_id`. The last node will usually be a `StyleSheet`.
    pub fn ancestors(&self, ast_node_id: AstNodeId) -> impl Iterator<Item = AstNodeId> + '_ {
        let parent_ids = &self.parent_ids;
        std::iter::successors(Some(ast_node_id), |node_id| parent_ids[*node_id])
    }

    /// Adds an `AstNode` to the `AstNodes` tree and returns its `AstNodeId`.
    pub fn add_node(&mut self, node: AstNode<'a>, parent_id: Option<AstNodeId>) -> AstNodeId {
        let mut node = node;
        let ast_node_id = self.parent_ids.push(parent_id);
        node.id = ast_node_id;
        self.nodes.push(node);
        ast_node_id
    }
}

#[derive(Debug)]
pub struct AstNodeParentIter<'s, 'a> {
    curr: Option<&'s AstNode<'a>>,
    nodes: &'s AstNodes<'a>,
}

impl<'s, 'a> Iterator for AstNodeParentIter<'s, 'a> {
    type Item = &'s AstNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr;
        self.curr = self.curr.and_then(|curr| self.nodes.parent_node(curr.id()));

        next
    }
}
