# TrueTree: an implementation of binary balanced tree

This is a balanced tree implementation (also called as AVL tree)

We allow you to define a custom type T to pass a the tree payload

This payload should have the clone, ord, eq and debug trait derived

The Ord trait is used to insert the values and to get it (this allow to match only partial payload)

The Eq trait is used to remove a node

The Clone trait is used to copy the value inside the tree